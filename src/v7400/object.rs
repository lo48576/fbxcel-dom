//! Functions and types for objects.

use fbxcel::tree::v7400::{NodeHandle, NodeId};

use crate::v7400::{Document, Result};

/// ID of an object node in the lowlevel tree.
///
/// Note that this is not "object ID".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectNodeId(NodeId);

impl ObjectNodeId {
    /// Creates a new `ObjectNodeId`.
    #[inline]
    #[must_use]
    pub(super) fn new(node_id: NodeId) -> Self {
        Self(node_id)
    }

    /// Returns the internal node ID.
    #[inline]
    #[must_use]
    pub(super) fn tree_node_id(self) -> NodeId {
        self.0
    }
}

/// Object ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectId(i64);

impl ObjectId {
    /// Creates a new ID.
    #[inline]
    #[must_use]
    pub(super) fn new(id: i64) -> Self {
        Self(id)
    }

    /// Returns the raw object ID value.
    ///
    /// This is lowlevel info, and maybe useless for usual case.
    #[inline]
    #[must_use]
    pub fn raw(self) -> i64 {
        self.0
    }
}

/// Object handle.
#[derive(Debug, Clone)]
pub struct ObjectHandle<'a> {
    /// Node ID.
    node_id: ObjectNodeId,
    /// Object ID.
    object_id: ObjectId,
    /// Document.
    doc: &'a Document,
}

impl<'a> ObjectHandle<'a> {
    /// Creates a new `ObjectHandle` from the given node ID.
    pub(super) fn from_node_id(node_id: ObjectNodeId, doc: &'a Document) -> Result<Self> {
        // TODO: Get object metadata from a cache.
        let object_id = get_object_id_from_node(&node_id.tree_node_id().to_handle(doc.tree()))?;

        Ok(Self {
            node_id,
            object_id,
            doc,
        })
    }

    /// Returns the node ID.
    #[inline]
    #[must_use]
    pub(super) fn tree_node(&self) -> NodeHandle<'a> {
        self.tree_node_id().to_handle(self.doc.tree())
    }

    /// Returns the node ID.
    #[inline]
    #[must_use]
    fn tree_node_id(&self) -> NodeId {
        self.node_id.tree_node_id()
    }

    /// Returns the object ID.
    #[inline]
    #[must_use]
    pub fn id(&self) -> ObjectId {
        self.object_id
    }

    /// Returns the object name.
    #[inline]
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        todo!()
    }

    /// Returns the object class in string.
    #[inline]
    #[must_use]
    pub fn class_str(&self) -> &str {
        todo!()
    }

    /// Returns the object subclass in string.
    #[inline]
    #[must_use]
    pub fn subclass_str(&self) -> &str {
        todo!()
    }

    /// Returns the node name.
    ///
    /// This is lowlevel info and maybe useless for usual use case.
    #[inline]
    #[must_use]
    pub fn node_name(&self) -> &'a str {
        self.tree_node().name()
    }
}

/// Fetches the object ID from the given node.
fn get_object_id_from_node(node: &NodeHandle<'_>) -> Result<ObjectId> {
    let object_id = node
        .attributes()
        .get(0)
        .ok_or_else(|| error!("expected object ID attribute but not found"))?
        .get_i64_or_type()
        .map_err(|ty| error!("expected `i64` object ID attribute but got {:?}", ty))?;
    Ok(ObjectId::new(object_id))
}
