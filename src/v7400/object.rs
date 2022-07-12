//! Objects-related stuff.
//!
//! # Object
//!
//! There are some data associated to objects.
//!
//! ## Object metadata
//!
//! Each object node has object metadata.
//! A metadata consists of four elements:
//!
//! ### Object ID
//!
//! Each object has object ID, which is unique in the document.
//!
//! Some objects have ID but have no corresponding nodes (in low-level tree).
//! Usually they are safely ignorable.
//!
//! This is represented by [`ObjectId`], and can be retrieved by
//! [`ObjectHandle::object_id()`].
//!
//! ### Name
//!
//! Objects may have names.
//! Note that the name can be empty string, and not guaranteed to be unique.
//!
//! This can be retrieved by [`ObjectHandle::name()`].
//!
//! ### Class and subclass
//!
//! These are used to distinguish actual data type (usage) of the object.
//!
//! It is not users' business to know their mappings, so `fbxcel_dom`
//! provides [`ObjectHandle::get_typed()`] to automatically cast the
//! object handle to "usable" handle types.
//!
//! However, `fbxcel_dom` is not perfect and would miss many mappings (because
//! FBX is proprietary format, and specification is not open).
//! If users want to use object types which are unsupported by `fbxcel_dom`,
//! they can implement new object handle types by their own, and/or use
//! lower-level APIs (such as [`ObjectHandle::source_objects()`],
//! [`ObjectHandle::destination_objects()`], and `fbxcel::tree` APIs).
//!
//! They can be retrieved by [`ObjectHandle::class()`] and
//! [`ObjectHandle::subclass()`].
//!
//! ## Object node ID
//!
//! If an object has corresponding tree node, then the node ID for the object
//! can be represented as "object node ID".
//!
//! This is represented by [`ObjectNodeId`].
//!
//! ## Object handle
//!
//! Object node ID is not useful without the document the object belongs to.
//! Object handle is a struct which contains a document and object identifiers
//! (object ID and object node ID for the same object).
//!
//! This is represented by [`ObjectHandle`].
//!
//! [`ObjectId`]: struct.ObjectId.html
//! [`ObjectHandle`]: struct.ObjectHandle.html
//! [`ObjectHandle::class()`]: struct.ObjectHandle.html#method.class
//! [`ObjectHandle::destination_objects()`]:
//!     struct.ObjectHandle.html#method.destination_objects
//! [`ObjectHandle::get_typed()`]: struct.ObjectHandle.html#method.get_typed
//! [`ObjectHandle::name()`]: struct.ObjectHandle.html#method.name
//! [`ObjectHandle::object_id()`]: struct.ObjectHandle.html#method.object_id
//! [`ObjectHandle::source_objects()`]:
//!     struct.ObjectHandle.html#method.source_objects
//! [`ObjectHandle::subclass()`]: struct.ObjectHandle.html#method.subclass
//! [`ObjectNodeId`]: struct.ObjectNodeId.html

use std::fmt;

use fbxcel::tree::v7400::{NodeHandle, NodeId};

use crate::v7400::{connection::Connection, Document};

use self::property::{ObjectProperties, PropertiesHandle};
pub use self::typed::TypedObjectHandle;
pub(crate) use self::{
    cache::ObjectsCache,
    meta::{ObjectClassSym, ObjectMeta},
};

#[macro_use]
mod macros;

mod cache;
pub mod deformer;
pub mod geometry;
pub mod material;
mod meta;
pub mod model;
pub mod nodeattribute;
pub mod property;
pub mod scene;
pub mod texture;
mod typed;
pub mod video;

/// Node ID of a object node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectNodeId(NodeId);

impl ObjectNodeId {
    /// Creates a new `ObjectNodeId`.
    pub(crate) fn new(node_id: NodeId) -> Self {
        Self(node_id)
    }

    /// Creates a new `ObjectHandle`.
    pub fn to_object_handle(self, doc: &Document) -> ObjectHandle<'_> {
        ObjectHandle::from_object_node_id(self, doc)
    }
}

impl std::ops::Deref for ObjectNodeId {
    type Target = NodeId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ObjectNodeId> for NodeId {
    fn from(v: ObjectNodeId) -> Self {
        v.0
    }
}

/// Object ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectId(i64);

impl ObjectId {
    /// Creates a new `ObjectId`.
    pub(crate) fn new(id: i64) -> Self {
        Self(id)
    }

    /// Creates a new `ObjectHandle`.
    pub fn to_object_handle(self, doc: &Document) -> Option<ObjectHandle<'_>> {
        ObjectHandle::from_object_id(self, doc)
    }

    /// Returns an iterator of destination objects and connection labels.
    pub fn destination_objects(
        self,
        doc: &Document,
    ) -> impl Iterator<Item = ConnectedObjectHandle<'_>> {
        doc.connections_cache()
            .outgoing_connections(self)
            .map(move |conn| ConnectedObjectHandle::new(conn.destination_id(), conn, doc))
    }

    /// Returns an iterator of source objects and connection labels.
    pub fn source_objects(self, doc: &Document) -> impl Iterator<Item = ConnectedObjectHandle<'_>> {
        doc.connections_cache()
            .incoming_connections(self)
            .map(move |conn| ConnectedObjectHandle::new(conn.source_id(), conn, doc))
    }

    /// Returns the raw object ID.
    pub fn raw(self) -> i64 {
        self.0
    }
}

/// Object handle.
///
/// See the [module-level documentation](index.html) for more detail.
#[derive(Clone, Copy)]
pub struct ObjectHandle<'a> {
    /// Node ID.
    node_id: ObjectNodeId,
    /// Object metadata.
    object_meta: &'a ObjectMeta,
    /// Document.
    doc: &'a Document,
}

impl<'a> ObjectHandle<'a> {
    /// Creates a new `ObjectHandle` from the given object node ID.
    ///
    /// # Panics
    ///
    /// This may panic if the object node with the given ID does not exist in
    /// the given document.
    fn from_object_node_id(node_id: ObjectNodeId, doc: &'a Document) -> Self {
        let object_meta = doc
            .objects_cache()
            .meta_from_node_id(node_id)
            .unwrap_or_else(|| panic!("No corresponding object metadata: node_id={:?}", node_id));
        Self {
            node_id,
            object_meta,
            doc,
        }
    }

    /// Creates a new `ObjectHandle` from the given object node ID.
    ///
    /// Returns `None` if the given object ID has no corresponding FBX node.
    fn from_object_id(obj_id: ObjectId, doc: &'a Document) -> Option<Self> {
        let node_id = doc.objects_cache().node_id(obj_id)?;
        let object_meta = doc
            .objects_cache()
            .meta_from_node_id(node_id)
            .expect("Should never fail: object cache should be consistent");
        assert_eq!(obj_id, object_meta.object_id(), "Object ID mismatch");
        Some(Self {
            node_id,
            object_meta,
            doc,
        })
    }

    /// Returns object node ID.
    pub fn object_node_id(&self) -> ObjectNodeId {
        self.node_id
    }

    /// Returns object ID.
    pub fn object_id(&self) -> ObjectId {
        self.object_meta.object_id()
    }

    /// Returns a reference to the document.
    pub fn document(&self) -> &'a Document {
        self.doc
    }

    /// Returns the node handle.
    pub fn node(&self) -> NodeHandle<'a> {
        self.node_id.to_handle(self.doc.tree())
    }

    /// Returns the object type.
    pub fn get_typed(&self) -> TypedObjectHandle<'a> {
        TypedObjectHandle::new(*self)
    }

    /// Returns object name.
    pub fn name(&self) -> Option<&'a str> {
        self.object_meta.name()
    }

    /// Returns object class.
    pub fn class(&self) -> &'a str {
        self.doc
            .objects_cache()
            .resolve_class_string(self.object_meta.class_sym())
    }

    /// Returns object subclass.
    pub fn subclass(&self) -> &'a str {
        self.doc
            .objects_cache()
            .resolve_class_string(self.object_meta.subclass_sym())
    }

    /// Returns an iterator of destination objects and connection labels.
    pub fn destination_objects(&self) -> impl Iterator<Item = ConnectedObjectHandle<'a>> {
        self.object_id().destination_objects(self.doc)
    }

    /// Returns an iterator of source objects and connection labels.
    pub fn source_objects(&self) -> impl Iterator<Item = ConnectedObjectHandle<'a>> {
        self.object_id().source_objects(self.doc)
    }

    /// Returns a handle of the directly associated properties node.
    pub fn direct_properties(&self) -> Option<PropertiesHandle<'a>> {
        PropertiesHandle::from_object(self)
    }

    /// Returns a proxy to object properties using the given native typename.
    ///
    /// `native_typename` should be the value of the first attribute of
    /// the `PropertyTemplate` node to be used.
    pub fn properties_by_native_typename(&self, native_typename: &str) -> ObjectProperties<'a> {
        ObjectProperties::from_object(self, native_typename)
    }
}

impl fmt::Debug for ObjectHandle<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /// Object metadata type for debug printing.
        #[derive(Debug)]
        #[allow(dead_code)] // Fields are intended for being printed by `Debug`.
        struct ObjectMeta<'a> {
            /// Object ID.
            id: ObjectId,
            /// Name (if exists).
            name: Option<&'a str>,
            /// Class.
            class: &'a str,
            /// Subclass.
            subclass: &'a str,
        }
        let meta = ObjectMeta {
            id: self.object_id(),
            name: self.name(),
            class: self.class(),
            subclass: self.subclass(),
        };
        f.debug_struct("ObjectHandle")
            .field("node_id", &self.node_id)
            .field("meta", &meta)
            .finish()
    }
}

/// Object handle (or ID) for connected object.
#[derive(Debug, Clone, Copy)]
pub struct ConnectedObjectHandle<'a> {
    /// Connected object.
    object_id: ObjectId,
    /// Connection.
    connection: &'a Connection,
    /// Document.
    doc: &'a Document,
}

impl<'a> ConnectedObjectHandle<'a> {
    /// Creates a new `ConnectedObjectHandle`.
    fn new(object_id: ObjectId, connection: &'a Connection, doc: &'a Document) -> Self {
        Self {
            object_id,
            connection,
            doc,
        }
    }

    /// Returns object ID.
    pub fn object_id(&self) -> ObjectId {
        self.object_id
    }

    /// Returns object handle if corresponding object node is available.
    pub fn object_handle(&self) -> Option<ObjectHandle<'a>> {
        self.object_id.to_object_handle(self.doc)
    }

    /// Returns connection label if available.
    pub fn label(&self) -> Option<&'a str> {
        self.connection
            .label_sym()
            .map(|sym| self.doc.connections_cache().resolve_label(sym))
    }
}
