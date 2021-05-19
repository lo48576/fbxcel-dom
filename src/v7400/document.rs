//! Document-level structures.

mod load;
pub mod meta;

use fbxcel::tree::v7400::{Children, NodeHandle, Tree};

use crate::v7400::connection::{
    ConnectionsCache, ConnectionsForObject, ConnectionsForObjectByLabel,
};
use crate::v7400::definitions_cache::DefinitionsCache;
use crate::v7400::objects_cache::ObjectsCache;
use crate::v7400::{ObjectHandle, ObjectId, ObjectNodeId};

pub use self::load::{LoadError, Loader};
pub use self::meta::DocumentMeta;

/// FBX document.
// This is intended to be a read-only structure as of writing this.
#[derive(Debug, Clone)]
pub struct Document {
    /// Low level tree.
    tree: Tree,
    /// Objects cache.
    objects_cache: ObjectsCache,
    /// Object properties template definitions cache.
    definitions_cache: DefinitionsCache,
    /// Objects connections cache.
    connections_cache: ConnectionsCache,
}

impl Document {
    /// Returns a reference to the lowlevel tree.
    #[inline]
    #[must_use]
    pub fn tree(&self) -> &Tree {
        &self.tree
    }

    /// Returns the root node.
    #[inline]
    #[must_use]
    pub(super) fn root_node(&self) -> NodeHandle<'_> {
        self.tree.root()
    }

    /// Returns a proxy to the document metadata.
    #[inline]
    #[must_use]
    pub fn meta(&self) -> DocumentMeta<'_> {
        DocumentMeta::new(self)
    }

    /// Returns an iterator of objects.
    #[must_use]
    pub fn objects(&self) -> Objects<'_> {
        let objects = self
            .root_node()
            .first_child_by_name("Objects")
            .map(|node| node.children());
        Objects {
            children: objects,
            doc: self,
        }
    }

    /// Returns an object with the given ID, if exists.
    #[inline]
    #[must_use]
    pub fn get_object_by_id(&self, id: ObjectId) -> Option<ObjectHandle<'_>> {
        ObjectHandle::from_object_id(id, self).ok()
    }

    /// Returns the objects cache.
    #[inline]
    #[must_use]
    pub(super) fn objects_cache(&self) -> &ObjectsCache {
        &self.objects_cache
    }

    /// Returns the object properties template definitions cache.
    #[inline]
    #[must_use]
    pub(super) fn definitions_cache(&self) -> &DefinitionsCache {
        &self.definitions_cache
    }

    /// Returns the object connections cache.
    #[inline]
    #[must_use]
    pub(super) fn connections_cache(&self) -> &ConnectionsCache {
        &self.connections_cache
    }

    /// Returns an iterator of source (child) objects.
    #[inline]
    #[must_use]
    pub fn source_objects(&self, dest_id: ObjectId) -> ConnectionsForObject<'_> {
        ConnectionsForObject::with_destination(dest_id, self.connections_cache())
    }

    /// Returns an iterator of destination (parent) objects.
    #[inline]
    #[must_use]
    pub fn destination_objects(&self, source_id: ObjectId) -> ConnectionsForObject<'_> {
        ConnectionsForObject::with_source(source_id, self.connections_cache())
    }

    /// Returns an iterator of source (child) objects.
    #[inline]
    #[must_use]
    pub fn source_objects_by_label(
        &self,
        dest_id: ObjectId,
        label: Option<&'_ str>,
    ) -> ConnectionsForObjectByLabel<'_> {
        ConnectionsForObjectByLabel::with_destination(dest_id, label, self.connections_cache())
    }

    /// Returns an iterator of destination (parent) objects.
    #[inline]
    #[must_use]
    pub fn destination_objects_by_label(
        &self,
        source_id: ObjectId,
        label: Option<&'_ str>,
    ) -> ConnectionsForObjectByLabel<'_> {
        ConnectionsForObjectByLabel::with_source(source_id, label, self.connections_cache())
    }
}

impl Document {
    /// Creates a new loader.
    #[inline]
    #[must_use]
    pub fn loader() -> Loader {
        Loader::new()
    }
}

/// Iterator of objects in a document.
#[derive(Debug, Clone)]
pub struct Objects<'a> {
    /// Children of `/Objects`.
    children: Option<Children<'a>>,
    /// Document.
    doc: &'a Document,
}

impl<'a> Iterator for Objects<'a> {
    type Item = ObjectHandle<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let doc = self.doc;
        self.children
            .as_mut()?
            .map(|node| ObjectNodeId::new(node.node_id()))
            .find_map(|node_id| match ObjectHandle::from_node_id(node_id, doc) {
                Ok(v) => Some(v),
                Err(e) => {
                    log::warn!("non-object node found under `/Objects`: {}", e);
                    None
                }
            })
    }
}
