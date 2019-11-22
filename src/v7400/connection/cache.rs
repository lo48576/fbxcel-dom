//! Connections cache.

use std::collections::{HashMap, HashSet};

use fbxcel::{
    low::v7400::AttributeValue,
    tree::v7400::{NodeHandle, NodeId, Tree},
};
use log::trace;
use string_interner::StringInterner;

use crate::v7400::{
    connection::{ConnectedNodeType, Connection, ConnectionIndex, ConnectionLabelSym},
    error::{
        connection::ConnectionError,
        load::{LoadError, StructureError},
    },
    object::ObjectId,
};

/// Connections cache.
#[derive(Debug, Clone)]
pub(crate) struct ConnectionsCache {
    /// Connections.
    connections: Vec<Connection>,
    /// Connection label interner.
    labels: StringInterner<ConnectionLabelSym>,
    /// Connection indices by source object ID.
    conn_indices_by_src: HashMap<ObjectId, Vec<ConnectionIndex>>,
    /// Connection indices by destination object ID.
    conn_indices_by_dest: HashMap<ObjectId, Vec<ConnectionIndex>>,
}

impl ConnectionsCache {
    /// Creates a new `ConnectionsCache` from the given FBX data tree.
    pub(crate) fn from_tree(tree: &Tree) -> Result<Self, LoadError> {
        ConnectionsCacheBuilder::default().load(tree)
    }

    /// Returns corresponding label.
    ///
    /// # Panics
    ///
    /// Panics if the given symbol is not registered.
    pub(crate) fn resolve_label(&self, sym: ConnectionLabelSym) -> &str {
        self.labels.resolve(sym).unwrap_or_else(|| {
            panic!(
                "The given connection label symbol is not registered: sym={:?}",
                sym
            )
        })
    }

    /// Returns an iterator of outgoing connections.
    pub(crate) fn outgoing_connections(
        &self,
        source: ObjectId,
    ) -> impl Iterator<Item = &Connection> {
        self.conn_indices_by_src
            .get(&source)
            .into_iter()
            .flatten()
            .map(move |index| &self.connections[index.value()])
    }

    /// Returns an iterator of incoming connections.
    pub(crate) fn incoming_connections(
        &self,
        destination: ObjectId,
    ) -> impl Iterator<Item = &Connection> {
        self.conn_indices_by_dest
            .get(&destination)
            .into_iter()
            .flatten()
            .map(move |index| &self.connections[index.value()])
    }
}

/// Connections cache.
#[derive(Debug)]
struct ConnectionsCacheBuilder {
    /// Connections data and node IDs.
    connections: Vec<(NodeId, Connection)>,
    /// Connection label interner.
    labels: StringInterner<ConnectionLabelSym>,
    /// Connection indices by source object ID.
    conn_indices_by_src: HashMap<ObjectId, Vec<ConnectionIndex>>,
    /// Connection indices by destination object ID.
    conn_indices_by_dest: HashMap<ObjectId, Vec<ConnectionIndex>>,
    /// Connections set to check duplicates.
    ///
    /// Contains `(source, destination, label)`s.
    conn_set: HashSet<(ObjectId, ObjectId, Option<ConnectionLabelSym>)>,
}

impl ConnectionsCacheBuilder {
    /// Loads the connectinos from the tree.
    fn load(mut self, tree: &Tree) -> Result<ConnectionsCache, LoadError> {
        let connections_node = tree
            .root()
            .children_by_name("Connections")
            .next()
            .ok_or(StructureError::MissingConnectionsNode)?;
        for conn_node in connections_node.children_by_name("C") {
            self.add_connection(conn_node)?;
        }

        Ok(self.build())
    }

    /// Loads a connection from the given node, and registers it.
    pub(crate) fn add_connection(&mut self, node: NodeHandle<'_>) -> Result<(), ConnectionError> {
        let index = ConnectionIndex::new(self.connections.len());

        let conn = self.load_connection(node, index)?;
        if !self
            .conn_set
            .insert((conn.source_id(), conn.destination_id(), conn.label_sym()))
        {
            let old_conn = self
                .conn_indices_by_src
                .get(&conn.source_id())
                .expect("Should never fail: entry should exist")
                .iter()
                .map(|index| &self.connections[index.value()])
                .find(|(_, old_conn)| {
                    old_conn.destination_id() == conn.destination_id()
                        && old_conn.label_sym() == conn.label_sym()
                })
                .expect("Should never fail: entry should exist");
            return Err(ConnectionError::DuplicateConnection(
                conn.source_id(),
                conn.destination_id(),
                conn.label_sym()
                    .map(|sym| {
                        self.labels.resolve(sym).expect(
                            "Should never fail: connection label symbol should be registered",
                        )
                    })
                    .map(ToOwned::to_owned),
                old_conn.0,
                old_conn.1.index(),
                node.node_id(),
                index,
            ));
        }
        self.connections.push((node.node_id(), conn));
        self.conn_indices_by_src
            .entry(conn.source_id())
            .or_insert_with(Vec::new)
            .push(index);
        self.conn_indices_by_dest
            .entry(conn.destination_id())
            .or_insert_with(Vec::new)
            .push(index);

        assert_eq!(
            self.connections.len(),
            self.conn_set.len(),
            "Connections set should be updated"
        );
        trace!(
            "Loaded connection successfully: node_id={:?}, index={:?}",
            node.node_id(),
            index
        );
        Ok(())
    }

    /// Loads a connection from the given node.
    pub(crate) fn load_connection(
        &mut self,
        node: NodeHandle<'_>,
        index: ConnectionIndex,
    ) -> Result<Connection, ConnectionError> {
        trace!(
            "Loading connection, node_id={:?}, index={:?}",
            node.node_id(),
            index
        );
        let attrs = node.attributes();
        let nodes_ty_str = attrs
            .get(0)
            .ok_or_else(|| ConnectionError::MissingNodeTypes(node.node_id(), index))?
            .get_string_or_type()
            .map_err(|ty| ConnectionError::InvalidNodeTypesType(node.node_id(), index, ty))?;
        let (destination_type, source_type) = match nodes_ty_str {
            "OO" => (ConnectedNodeType::Object, ConnectedNodeType::Object),
            "OP" => (ConnectedNodeType::Object, ConnectedNodeType::Property),
            "PO" => (ConnectedNodeType::Property, ConnectedNodeType::Object),
            "PP" => (ConnectedNodeType::Property, ConnectedNodeType::Property),
            v => {
                return Err(ConnectionError::InvalidNodeTypesValue(
                    node.node_id(),
                    index,
                    v.to_owned(),
                ));
            }
        };
        let source_id = attrs
            .get(1)
            .ok_or_else(|| ConnectionError::MissingSourceId(node.node_id(), index))?
            .get_i64_or_type()
            .map(ObjectId::new)
            .map_err(|ty| ConnectionError::InvalidSourceIdType(node.node_id(), index, ty))?;
        let destination_id = attrs
            .get(2)
            .ok_or_else(|| ConnectionError::MissingDestinationId(node.node_id(), index))?
            .get_i64_or_type()
            .map(ObjectId::new)
            .map_err(|ty| ConnectionError::InvalidDestinationIdType(node.node_id(), index, ty))?;
        let label = attrs
            .get(3)
            .map(AttributeValue::get_string_or_type)
            .transpose()
            .map_err(|ty| ConnectionError::InvalidLabelType(node.node_id(), index, ty))?;
        let label_sym = label.map(|s| self.labels.get_or_intern(s));
        trace!(
            "Successfully loaded connection: node_id={:?}, index={:?}, \
             dst_type={:?}, src_type={:?}, src_id={:?}, dest_id={:?}, label={:?}",
            node.node_id(),
            index,
            destination_type,
            source_type,
            source_id,
            destination_id,
            label
        );
        Ok(Connection::new(
            source_id,
            source_type,
            destination_id,
            destination_type,
            label_sym,
            index,
        ))
    }

    /// Builds the `ConnectionsCache`.
    fn build(self) -> ConnectionsCache {
        ConnectionsCache {
            connections: self
                .connections
                .into_iter()
                .map(|(_node_id, conn)| conn)
                .collect(),
            labels: self.labels,
            conn_indices_by_src: self.conn_indices_by_src,
            conn_indices_by_dest: self.conn_indices_by_dest,
        }
    }
}

impl Default for ConnectionsCacheBuilder {
    fn default() -> Self {
        Self {
            connections: Default::default(),
            labels: StringInterner::new(),
            conn_indices_by_src: Default::default(),
            conn_indices_by_dest: Default::default(),
            conn_set: Default::default(),
        }
    }
}
