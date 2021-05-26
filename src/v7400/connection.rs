//! Object connections.

use std::collections::{HashMap, HashSet};
use std::iter;
use std::sync::Arc;

use fbxcel::low::v7400::AttributeValue as A;
use fbxcel::tree::v7400::{NodeHandle, Tree};
use lasso::{MiniSpur, Rodeo, RodeoReader};

use crate::v7400::document::{Document, LoadError};
use crate::v7400::{ObjectHandle, ObjectId};

/// A symbol of an interned connection label string.
// This should not be exposed to users.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct ConnectionLabelSym(MiniSpur);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Type of a connection end.
pub enum ConnectedNodeType {
    /// Object.
    Object,
    /// Property.
    Property,
}

/// Connection index.
///
/// This value is an index for `ConnectionsCache::connections`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ConnectionIndex(usize);

impl ConnectionIndex {
    /// Creates a new index.
    #[inline]
    #[must_use]
    fn new(i: usize) -> Self {
        Self(i)
    }

    /// Returns the raw index.
    #[inline]
    #[must_use]
    fn raw(self) -> usize {
        self.0
    }
}

/// A handle to a connection between two objects (provided by `/Connections/C` node).
#[derive(Debug, Clone, Copy)]
pub struct Connection<'a> {
    /// Inner data.
    inner: &'a ConnectionInner,
    /// Document.
    doc: &'a Document,
}

impl<'a> Connection<'a> {
    /// Creates the new connection handle.
    #[inline]
    #[must_use]
    pub(super) fn new(inner: &'a ConnectionInner, doc: &'a Document) -> Self {
        Self { inner, doc }
    }

    /// Returns the source (child) object ID.
    #[inline]
    #[must_use]
    pub fn source_id(&self) -> ObjectId {
        self.inner.source_id
    }

    /// Returns the source (child) object handle.
    ///
    /// Note that this returns `None` if the source object is a dummy object,
    /// which has no corresponding node.
    #[inline]
    #[must_use]
    pub fn source(&self) -> Option<ObjectHandle<'a>> {
        self.doc.get_object_by_id(self.source_id())
    }

    /// Returns the source (child) node type.
    #[inline]
    #[must_use]
    pub fn source_type(&self) -> ConnectedNodeType {
        self.inner.source_type
    }

    /// Returns the destination (parent) object ID.
    #[inline]
    #[must_use]
    pub fn destination_id(&self) -> ObjectId {
        self.inner.dest_id
    }

    /// Returns the destination (parent) object handle.
    ///
    /// Note that this returns `None` if the destination object is a dummy
    /// object, which has no corresponding node.
    #[inline]
    #[must_use]
    pub fn destination(&self) -> Option<ObjectHandle<'a>> {
        self.doc.get_object_by_id(self.destination_id())
    }

    /// Returns the destination (parent) node type.
    #[inline]
    #[must_use]
    pub fn destination_type(&self) -> ConnectedNodeType {
        self.inner.dest_type
    }

    /// Returns the connection label.
    ///
    /// If you only want to know whether the label exists and don't care about
    /// its content, use [`has_label`] method since it is more efficient than
    /// `self.label().is_some()`.
    ///
    /// [`has_label`]: `Self::has_label`
    #[must_use]
    pub fn label(&self) -> Option<&'a str> {
        self.inner
            .label
            .map(|sym| self.doc.connections_cache().label_strings.resolve(&sym.0))
    }

    /// Returns whether the connection has label.
    ///
    /// This is a little efficient version of `self.label().is_some()`.
    /// [`label`] method looks up the internal string table if it is `Some(_)`,
    /// but `has_label` does not.
    ///
    /// If you only want to know whether the label exists and don't care about
    /// its content, use this method.
    ///
    /// [`label`]: `Self::label`
    #[inline]
    #[must_use]
    pub fn has_label(&self) -> bool {
        self.inner.label.is_some()
    }
}

/// An internal data for a objects connection (provided by `/Connections/C` node).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct ConnectionInner {
    /// Source (child) object ID.
    source_id: ObjectId,
    /// Source (child) node type.
    source_type: ConnectedNodeType,
    /// Destination (parent) object ID.
    dest_id: ObjectId,
    /// Source (parent) node type.
    dest_type: ConnectedNodeType,
    /// Label.
    label: Option<ConnectionLabelSym>,
    /// Connection node index.
    index: ConnectionIndex,
}

impl ConnectionInner {
    /// Creates a new connection data.
    #[inline]
    #[must_use]
    fn new(
        source_id: ObjectId,
        source_type: ConnectedNodeType,
        dest_id: ObjectId,
        dest_type: ConnectedNodeType,
        label: Option<ConnectionLabelSym>,
        index: ConnectionIndex,
    ) -> Self {
        Self {
            source_id,
            source_type,
            dest_id,
            dest_type,
            label,
            index,
        }
    }
}

/// Object connections cache.
#[derive(Debug, Clone)]
pub(super) struct ConnectionsCache {
    /// Connections.
    connections: Vec<ConnectionInner>,
    /// Interned label strings.
    label_strings: Arc<RodeoReader<MiniSpur>>,
    /// A map from source (child) object ID to connection indices.
    connections_by_src: HashMap<ObjectId, Vec<ConnectionIndex>>,
    /// A map from destination (parent) object ID to connection indices.
    connections_by_dest: HashMap<ObjectId, Vec<ConnectionIndex>>,
}

impl ConnectionsCache {
    /// Creates a new connections cache from the given tree.
    #[inline]
    pub(super) fn from_tree(tree: &Tree) -> Result<Self, LoadError> {
        ConnectionsCacheBuilder::default().load(tree)
    }
}

/// Objcets cache builder.
#[derive(Debug)]
struct ConnectionsCacheBuilder {
    /// Connections data and node IDs.
    connections: Vec<ConnectionInner>,
    /// Interned label strings.
    label_strings: Rodeo<MiniSpur>,
    /// A map from source (child) object ID to connection indices.
    connections_by_src: HashMap<ObjectId, Vec<ConnectionIndex>>,
    /// A map from destination (parent) object ID to connection indices.
    connections_by_dest: HashMap<ObjectId, Vec<ConnectionIndex>>,
    /// A set of connections to find duplicates.
    ///
    /// This is used only to find duplicates, and is not included in `ConnectionsCache`.
    ///
    /// Contains `(source, destination, label)`s.
    conn_set: HashSet<(ObjectId, ObjectId, Option<ConnectionLabelSym>)>,
}

// Workaround for lasso-0.5.0. See <https://github.com/Kixiron/lasso/issues/26>.
impl Default for ConnectionsCacheBuilder {
    fn default() -> Self {
        Self {
            connections: Default::default(),
            label_strings: Rodeo::new(),
            connections_by_src: Default::default(),
            connections_by_dest: Default::default(),
            conn_set: Default::default(),
        }
    }
}

impl ConnectionsCacheBuilder {
    /// Creates a connections cache from the given tree.
    fn load(mut self, tree: &Tree) -> Result<ConnectionsCache, LoadError> {
        let connections = tree
            .root()
            .first_child_by_name("Connections")
            .ok_or_else(|| {
                LoadError::from_msg("expected toplevel `Connections` node to exist but not found")
            })?;

        for (index, conn_node) in connections.children_by_name("C").enumerate() {
            let index = ConnectionIndex::new(index);
            let conn = self.load_connection(conn_node, index)?;
            self.register_connection(conn)?;
        }

        Ok(self.build())
    }

    /// Builds the connections cache.
    fn build(self) -> ConnectionsCache {
        ConnectionsCache {
            connections: self.connections,
            label_strings: Arc::new(self.label_strings.into_reader()),
            connections_by_src: self.connections_by_src,
            connections_by_dest: self.connections_by_dest,
        }
    }

    /// Loads a connection.
    fn load_connection(
        &mut self,
        node: NodeHandle<'_>,
        conn_index: ConnectionIndex,
    ) -> Result<ConnectionInner, LoadError> {
        let (node_types, source_id, dest_id, label): (&str, i64, i64, Option<&str>) =
            match node.attributes() {
                [A::String(types), A::I64(source_id), A::I64(dest_id), A::String(label)] => {
                    (types, *source_id, *dest_id, Some(label))
                }
                [A::String(types), A::I64(source_id), A::I64(dest_id)] => {
                    (types, *source_id, *dest_id, None)
                }
                [a0, a1, a2] => {
                    return Err(LoadError::from_msg(format!(
                        "invalid node attributes: expected `(String, i64, i64)` attributes, \
                        but got `({:?}, {:?}, {:?})`",
                        a0.type_(),
                        a1.type_(),
                        a2.type_()
                    )))
                }
                [a0, a1, a2, a3] => {
                    return Err(LoadError::from_msg(format!(
                        "invalid node attributes: expected `(String, i64, i64, String)` \
                            attributes, but got `({:?}, {:?}, {:?}, {:?})`",
                        a0.type_(),
                        a1.type_(),
                        a2.type_(),
                        a3.type_()
                    )))
                }
                v => {
                    return Err(LoadError::from_msg(format!(
                        "invalid object node attributes: \
                        expected three or four attributes but got {}",
                        v.len()
                    )))
                }
            };
        let source_id = ObjectId::new(source_id);
        let dest_id = ObjectId::new(dest_id);
        let (dest_type, source_type) = match node_types {
            "OO" => (ConnectedNodeType::Object, ConnectedNodeType::Object),
            "OP" => (ConnectedNodeType::Object, ConnectedNodeType::Property),
            "PO" => (ConnectedNodeType::Property, ConnectedNodeType::Object),
            "PP" => (ConnectedNodeType::Property, ConnectedNodeType::Property),
            v => {
                return Err(LoadError::from_msg(format!(
                    "invalid node types value (conn_index={:?}, value={:?})",
                    conn_index, v
                )));
            }
        };
        let label_sym = label.map(|s| ConnectionLabelSym(self.label_strings.get_or_intern(s)));

        Ok(ConnectionInner::new(
            source_id,
            source_type,
            dest_id,
            dest_type,
            label_sym,
            conn_index,
        ))
    }

    /// Registers a connection.
    fn register_connection(&mut self, conn: ConnectionInner) -> Result<(), LoadError> {
        let index = conn.index;

        // Check if the same connection edge is already registered.
        if !self
            .conn_set
            .insert((conn.source_id, conn.dest_id, conn.label))
        {
            // Duplicates found.
            let old_conn = self
                .connections_by_src
                .get(&conn.source_id)
                .expect("should never fail: connection with the source conn.source_id exists")
                .iter()
                .map(|index| &self.connections[index.raw()])
                .find(|old_conn| {
                    (old_conn.dest_id == conn.dest_id) && (old_conn.label == conn.label)
                })
                .expect(
                    "should never fail: duplicate connection is known to exist \
                    thanks to self.conn_set",
                );
            let label = conn.label.map(|label| self.label_strings.resolve(&label.0));
            return Err(LoadError::from_msg(format!(
                "duplicate connection from {:?} to {:?} with label {:?} \
                (old_index={:?}, new_index={:?})",
                conn.source_id, conn.dest_id, label, old_conn.index, index
            )));
        }

        self.connections.push(conn);
        self.connections_by_src
            .entry(conn.source_id)
            .or_insert_with(Vec::new)
            .push(index);
        self.connections_by_dest
            .entry(conn.dest_id)
            .or_insert_with(Vec::new)
            .push(index);

        Ok(())
    }
}

/// An iterator of connections from / to an object.
#[derive(Debug, Clone)]
pub struct ConnectionsForObject<'a> {
    /// Connections for the object.
    iter: std::slice::Iter<'a, ConnectionIndex>,
    /// Document.
    doc: &'a Document,
}

impl<'a> ConnectionsForObject<'a> {
    /// Creates an empty iterator, which returns nothing.
    #[inline]
    #[must_use]
    fn empty(doc: &'a Document) -> Self {
        Self {
            iter: [].iter(),
            doc,
        }
    }

    /// Creates an iterator with the fixed source object ID.
    #[must_use]
    pub(super) fn with_source(source_id: ObjectId, doc: &'a Document) -> Self {
        Self {
            iter: doc
                .connections_cache()
                .connections_by_src
                .get(&source_id)
                .map_or(&[] as &[_], |vec| &*vec)
                .iter(),
            doc,
        }
    }

    /// Creates an iterator with the fixed destination object ID.
    #[must_use]
    pub(super) fn with_destination(dest_id: ObjectId, doc: &'a Document) -> Self {
        Self {
            iter: doc
                .connections_cache()
                .connections_by_dest
                .get(&dest_id)
                .map_or(&[] as &[_], |vec| &*vec)
                .iter(),
            doc,
        }
    }
}

impl<'a> Iterator for ConnectionsForObject<'a> {
    type Item = Connection<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.iter.next()?;
        let inner = &self.doc.connections_cache().connections[index.0];
        Some(Connection::new(inner, self.doc))
    }
}

impl iter::FusedIterator for ConnectionsForObject<'_> {}

/// An iterator of connections from / to an object, filtered by connection label.
#[derive(Debug, Clone)]
pub struct ConnectionsForObjectByLabel<'a> {
    /// Connections iterator.
    iter: ConnectionsForObject<'a>,
    /// Connection label.
    ///
    /// `None` indicates that the iterator should iterate connections with no label.
    label: Option<ConnectionLabelSym>,
}

impl<'a> ConnectionsForObjectByLabel<'a> {
    /// Creates an empty iterator, which returns nothing.
    #[inline]
    #[must_use]
    fn empty(doc: &'a Document) -> Self {
        Self {
            iter: ConnectionsForObject::empty(doc),
            label: None,
        }
    }

    /// Creates an iterator with the fixed source object ID and label.
    #[must_use]
    pub(super) fn with_source(
        source_id: ObjectId,
        label: Option<&'_ str>,
        doc: &'a Document,
    ) -> Self {
        let label = match label {
            Some(label) => match doc.connections_cache().label_strings.get(label) {
                Some(sym) => Some(ConnectionLabelSym(sym)),
                None => return Self::empty(doc),
            },
            None => None,
        };
        Self {
            iter: ConnectionsForObject::with_source(source_id, doc),
            label,
        }
    }

    /// Creates an iterator with the fixed destination object ID and label.
    #[must_use]
    pub(super) fn with_destination(
        dest_id: ObjectId,
        label: Option<&'_ str>,
        doc: &'a Document,
    ) -> Self {
        let label = match label {
            Some(label) => match doc.connections_cache().label_strings.get(label) {
                Some(sym) => Some(ConnectionLabelSym(sym)),
                None => return Self::empty(doc),
            },
            None => None,
        };
        Self {
            iter: ConnectionsForObject::with_destination(dest_id, doc),
            label,
        }
    }
}

impl<'a> Iterator for ConnectionsForObjectByLabel<'a> {
    type Item = Connection<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let label = self.label;
        self.iter.find(|conn| conn.inner.label == label)
    }
}

impl iter::FusedIterator for ConnectionsForObjectByLabel<'_> {}
