//! Functions and types for objects.

pub mod deformer;
pub mod geometry;
pub mod material;
pub mod model;
mod scene;
pub mod subdeformer;
pub mod texture;
mod typed;
pub mod video;

use fbxcel::tree::v7400::{NodeHandle, NodeId};

use crate::v7400::connection::{ConnectionsForObject, ConnectionsForObjectByLabel};
use crate::v7400::objects_cache::ObjectMeta;
use crate::v7400::properties::{PropertiesHandle, PropertiesNodeId};
use crate::v7400::{Document, ObjectProperties, Result};

pub use self::scene::{SceneHandle, SceneIter, SceneRootChildren};
pub use self::typed::{Class, Subclass, TypedObject};

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
    pub(super) fn to_handle(self, doc: &Document) -> Option<ObjectHandle<'_>> {
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
    pub(super) fn to_handle(self, doc: &Document) -> Option<ObjectHandle<'_>> {
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
#[derive(Debug, Clone, Copy)]
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
    pub(super) fn from_object_id(object_id: ObjectId, doc: &'a Document) -> Result<Self> {
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

    /// Returns the lowlevel node handle.
    #[inline]
    #[must_use]
    pub(super) fn tree_node(&self) -> NodeHandle<'a> {
        self.tree_node_id().to_handle(self.doc.tree())
    }

    /// Returns the lowlevel node ID.
    #[inline]
    #[must_use]
    fn tree_node_id(&self) -> NodeId {
        self.node_id.tree_node_id()
    }

    /// Returns the object node ID.
    #[inline]
    #[must_use]
    pub fn node_id(&self) -> ObjectNodeId {
        self.node_id
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

    /// Returns the object properties handle.
    #[inline]
    #[must_use]
    pub fn props(&self, native_typename: Option<&str>) -> ObjectProperties<'a> {
        let direct_props = self.direct_props_node_id();
        let default_props = native_typename.and_then(|ty| self.default_props_node_id(ty));
        ObjectProperties::new(direct_props, default_props, self.doc)
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
    pub fn direct_props(&self) -> Option<PropertiesHandle<'_>> {
        self.direct_props_node_id().map(|id| id.to_handle(self.doc))
    }

    /// Returns the default properties node handle.
    #[inline]
    #[must_use]
    fn default_props_node_id(&self, native_typename: &str) -> Option<PropertiesNodeId> {
        self.doc
            .definitions_cache()
            .props_node_id(self.node_name(), native_typename)
    }

    /// Returns the direct properties node handle.
    ///
    /// "Default" here means that the values are stored under the `Definitions`
    /// node, rather than under the target object.
    #[inline]
    #[must_use]
    pub fn default_props(&self, native_typename: &str) -> Option<PropertiesHandle<'_>> {
        self.default_props_node_id(native_typename)
            .map(|id| id.to_handle(self.doc))
    }

    /// Returns an iterator of source (child) objects.
    #[inline]
    #[must_use]
    pub fn source_objects(&self) -> ConnectionsForObject<'a> {
        self.doc.source_objects(self.id())
    }

    /// Returns an iterator of destination (parent) objects.
    #[inline]
    #[must_use]
    pub fn destination_objects(&self) -> ConnectionsForObject<'a> {
        self.doc.destination_objects(self.id())
    }

    /// Returns an iterator of source (child) objects.
    #[inline]
    #[must_use]
    pub fn source_objects_by_label(
        &self,
        label: Option<&'_ str>,
    ) -> ConnectionsForObjectByLabel<'a> {
        self.doc.source_objects_by_label(self.id(), label)
    }

    /// Returns an iterator of destination (parent) objects.
    #[inline]
    #[must_use]
    pub fn destination_objects_by_label(
        &self,
        label: Option<&'_ str>,
    ) -> ConnectionsForObjectByLabel<'a> {
        self.doc.destination_objects_by_label(self.id(), label)
    }
}

/// A trait for subtypes of object nodes.
pub trait ObjectSubtypeHandle<'a>: Sized {
    /// Object node ID type.
    type NodeId: Copy + Eq;

    /// Creates the object subtype handle from the given (generic) object handle.
    fn from_object(object: &ObjectHandle<'a>) -> Result<Self>;
    /// Creates a generic object handle.
    fn as_object(&self) -> &ObjectHandle<'a>;
    /// Returns the node ID of the dedicated ID type for this kind of objects.
    fn node_id(&self) -> Self::NodeId;
}
