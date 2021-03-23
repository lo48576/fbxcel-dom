//! FBX DOM.

use fbxcel::tree::v7400::Tree;

use crate::v7400::{
    connection::ConnectionsCache,
    definition::DefinitionsCache,
    object::property::{PropertiesHandle, PropertiesNodeId},
    object::{scene::SceneHandle, ObjectHandle, ObjectsCache},
};

pub use self::loader::Loader;

mod loader;

/// FBX DOM.
#[derive(Debug, Clone)]
pub struct Document {
    /// FBX data tree.
    tree: Tree,
    /// Objects cache.
    objects: ObjectsCache,
    /// Objects connection cache.
    connections: ConnectionsCache,
    /// Object template definitions.
    definitions: DefinitionsCache,
}

impl Document {
    /// Returns a reference to the tree.
    pub fn tree(&self) -> &Tree {
        &self.tree
    }

    /// Returns a reference to the objects cache.
    pub(crate) fn objects_cache(&self) -> &ObjectsCache {
        &self.objects
    }

    /// Returns a reference to the connections cache.
    pub(crate) fn connections_cache(&self) -> &ConnectionsCache {
        &self.connections
    }

    /// Returns a reference to the object template definitions.
    pub(crate) fn definitions_cache(&self) -> &DefinitionsCache {
        &self.definitions
    }

    /// Returns an iterator of all object nodes.
    pub fn objects(&self) -> impl Iterator<Item = ObjectHandle<'_>> {
        self.objects
            .object_node_ids()
            .map(move |id| id.to_object_handle(self))
    }

    /// Returns `Document` object nodes, which have root object ID of scenes.
    pub fn scenes(&self) -> impl Iterator<Item = SceneHandle<'_>> {
        self.objects.document_nodes().iter().map(move |obj_id| {
            SceneHandle::new(obj_id.to_object_handle(self))
                .expect("Should never fail: Actually using `Document` objects")
        })
    }

    /// Returns the "GlobalSettings" root level property block, if one exists.
    pub fn global_settings(&self) -> Option<PropertiesHandle> {
        let property_node = self
            .tree()
            .root()
            .children_by_name("GlobalSettings")
            .next()?
            .children_by_name("Properties70")
            .next()?;

        let handle = PropertiesHandle::new(PropertiesNodeId::new(property_node.node_id()), self);
        return Some(handle);
    }
}

impl AsRef<Tree> for Document {
    fn as_ref(&self) -> &Tree {
        &self.tree
    }
}
