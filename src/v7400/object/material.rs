//! Objects with `Material` class.

use crate::v7400::connection::ConnectionsForObject;
use crate::v7400::object::model::ModelMeshHandle;
use crate::v7400::object::texture::TextureHandle;
use crate::v7400::object::{ObjectHandle, ObjectId, ObjectNodeId, ObjectSubtypeHandle};
use crate::v7400::Result;

/// Node ID for a material object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaterialNodeId(ObjectNodeId);

/// Object handle for a material object.
#[derive(Debug, Clone, Copy)]
pub struct MaterialHandle<'a> {
    /// Object handle.
    object: ObjectHandle<'a>,
}

impl<'a> MaterialHandle<'a> {
    /// Returns the object ID.
    #[inline]
    #[must_use]
    pub fn object_id(&self) -> ObjectId {
        self.object.id()
    }
}

impl<'a> MaterialHandle<'a> {
    /// Returns the parent model mesh nodes.
    ///
    /// This returns an iterator of model meshes since a material can be used by
    /// multiple meshes.
    #[inline]
    #[must_use]
    pub fn parent_model_meshes(&self) -> ParentModelMeshes<'a> {
        ParentModelMeshes {
            destinations: self.as_object().destination_objects(),
        }
    }

    /// Returns the diffuse texture.
    ///
    /// If there are two or more child diffuse textures, one of them is returned.
    /// If you want to get all of them, use [`ObjectHandle::source_objects`]
    /// and filter by yourself.
    #[inline]
    #[must_use]
    pub fn texture_diffuse_color(&self) -> Option<TextureHandle<'a>> {
        self.as_object()
            .source_objects_by_label(Some("DiffuseColor"))
            .filter_map(|conn| conn.source())
            .find_map(|obj| TextureHandle::from_object(&obj).ok())
    }

    /// Returns the diffuse texture with transparency?
    ///
    /// If there are two or more child transparent color textures, one of them is returned.
    /// If you want to get all of them, use [`ObjectHandle::source_objects`]
    /// and filter by yourself.
    #[inline]
    #[must_use]
    pub fn texture_transparent_color(&self) -> Option<TextureHandle<'a>> {
        self.as_object()
            .source_objects_by_label(Some("TransparentColor"))
            .filter_map(|conn| conn.source())
            .find_map(|obj| TextureHandle::from_object(&obj).ok())
    }
}

impl<'a> ObjectSubtypeHandle<'a> for MaterialHandle<'a> {
    type NodeId = MaterialNodeId;

    fn from_object(object: &ObjectHandle<'a>) -> Result<Self> {
        let class = object.class();
        if class != "Material" {
            return Err(error!(
                "not a model object: expected \"Material\" class but got {:?} class",
                class
            ));
        }

        Ok(Self { object: *object })
    }

    #[inline]
    fn as_object(&self) -> &ObjectHandle<'a> {
        &self.object
    }

    #[inline]
    fn node_id(&self) -> Self::NodeId {
        MaterialNodeId(self.object.node_id())
    }
}

impl<'a> AsRef<ObjectHandle<'a>> for MaterialHandle<'a> {
    #[inline]
    fn as_ref(&self) -> &ObjectHandle<'a> {
        self.as_object()
    }
}

/// Iterator of `Model`(`Mesh`) nodes which are children of a `Material` node.
#[derive(Debug, Clone)]
pub struct ParentModelMeshes<'a> {
    /// Destination objects.
    destinations: ConnectionsForObject<'a>,
}

impl<'a> Iterator for ParentModelMeshes<'a> {
    type Item = ModelMeshHandle<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.destinations
            .by_ref()
            .filter(|conn| !conn.has_label())
            .filter_map(|conn| conn.destination())
            .find_map(|obj| ModelMeshHandle::from_object(&obj).ok())
    }
}

/// Subclass of a material known to the fbxcel-dom crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum MaterialSubclass {
    /// Empty subclass.
    None,
}
