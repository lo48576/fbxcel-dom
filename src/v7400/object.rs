//! Functions and types for objects.

use fbxcel::tree::v7400::{NodeHandle, NodeId};

use crate::v7400::objects_cache::ObjectMeta;
use crate::v7400::properties::{PropertiesNodeHandle, PropertiesNodeId};
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

    /// Creates a new `ObjectHandle` for the given document.
    #[allow(dead_code)] // TODO: Remove when this attr becomes unnecessary.
    #[inline]
    #[must_use]
    pub(super) fn to_object_handle(self, doc: &Document) -> Option<ObjectHandle<'_>> {
        ObjectHandle::from_node_id(self, doc).ok()
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

    /// Creates a new `ObjectHandle` for the given document.
    #[allow(dead_code)] // TODO: Remove when this attr becomes unnecessary.
    #[inline]
    #[must_use]
    pub(super) fn to_object_handle(self, doc: &Document) -> Option<ObjectHandle<'_>> {
        ObjectHandle::from_object_id(self, doc).ok()
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
    meta: &'a ObjectMeta,
    /// Document.
    doc: &'a Document,
}

impl<'a> ObjectHandle<'a> {
    /// Creates a new `ObjectHandle` from the given node ID.
    pub(super) fn from_node_id(node_id: ObjectNodeId, doc: &'a Document) -> Result<Self> {
        let meta = doc
            .objects_cache()
            .meta_from_node_id(node_id)
            .ok_or_else(|| {
                error!(
                    "expected object node ID but was not (node_id={:?})",
                    node_id
                )
            })?;

        Ok(Self { node_id, meta, doc })
    }

    /// Creates a new `ObjectHandle` from the given object ID.
    fn from_object_id(object_id: ObjectId, doc: &'a Document) -> Result<Self> {
        let node_id = doc.objects_cache().node_id(object_id).ok_or_else(|| {
            error!(
                "expected valid object ID but was not (object_id={:?})",
                object_id
            )
        })?;
        let meta = doc
            .objects_cache()
            .meta_from_node_id(node_id)
            .unwrap_or_else(|| {
                unreachable!(
                    "should never fail since it is confirmed by \
                `object_node_id()` call that the object is registered"
                )
            });

        Ok(Self { node_id, meta, doc })
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
        self.meta.id()
    }

    /// Returns the object name.
    #[inline]
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.meta.name()
    }

    /// Returns the object class as a string.
    #[inline]
    #[must_use]
    pub fn class(&self) -> &str {
        self.meta.class(self.doc.objects_cache())
    }

    /// Returns the object subclass as a string.
    #[inline]
    #[must_use]
    pub fn subclass(&self) -> &str {
        self.meta.subclass(self.doc.objects_cache())
    }

    /// Returns the node name.
    ///
    /// This is lowlevel info and maybe useless for usual use case.
    #[inline]
    #[must_use]
    pub fn node_name(&self) -> &'a str {
        self.tree_node().name()
    }

    /// Returns the direct properties node handle.
    #[inline]
    #[must_use]
    fn direct_props_node_id(&self) -> Option<PropertiesNodeId> {
        self.tree_node()
            .first_child_by_name("Properties70")
            .map(|node| PropertiesNodeId::new(node.node_id()))
    }

    /// Returns the direct properties node handle.
    ///
    /// "Direct" here means that the default values are not accessible through
    /// the returned properties node handle.
    #[inline]
    #[must_use]
    pub fn direct_props(&self) -> Option<PropertiesNodeHandle<'_>> {
        self.direct_props_node_id()
            .map(|id| PropertiesNodeHandle::new(id, self.doc))
    }
}
