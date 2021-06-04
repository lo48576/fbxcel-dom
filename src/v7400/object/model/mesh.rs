//! Objects with `Model` class and `Mesh` subclass.

use crate::v7400::connection::ConnectionsForObject;
use crate::v7400::object::geometry::GeometryMeshHandle;
use crate::v7400::object::material::AnyMaterialHandle;
use crate::v7400::object::model::AnyModelHandle;
use crate::v7400::object::{ObjectHandle, ObjectId, ObjectNodeId, ObjectSubtypeHandle};
use crate::v7400::Result;

/// Node ID for a model object with subclass `Mesh`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelMeshNodeId(ObjectNodeId);

/// Object handle for a model object with subclass `Mesh`.
#[derive(Debug, Clone, Copy)]
pub struct ModelMeshHandle<'a> {
    /// Model handle.
    object: AnyModelHandle<'a>,
}

impl<'a> ModelMeshHandle<'a> {
    /// Creates a model (mesh) handle from the given model handle.
    pub(super) fn from_model(object: &AnyModelHandle<'a>) -> Result<Self> {
        let subclass = object.as_object().subclass();
        if subclass != "Mesh" {
            return Err(error!(
                "not a `Model(Mesh)` object: expected \"Mesh\" subclass \
                but got {:?} subclass",
                subclass
            ));
        }

        Ok(Self { object: *object })
    }

    /// Returns the object ID.
    #[inline]
    #[must_use]
    pub fn object_id(&self) -> ObjectId {
        self.as_object().id()
    }

    /// Returns the reference to the more generic model handle.
    #[inline]
    #[must_use]
    pub fn as_model(&self) -> &AnyModelHandle<'a> {
        &self.object
    }
}

impl<'a> ModelMeshHandle<'a> {
    /// Returns the child geometry mesh.
    ///
    /// If there are two or more child geometry meshes, one of them is returned.
    /// If you want to get all of them, use [`ObjectHandle::source_objects`]
    /// and filter by yourself.
    #[must_use]
    pub fn child_geometry_mesh(&self) -> Option<GeometryMeshHandle<'a>> {
        self.as_object()
            .source_objects()
            .filter(|conn| !conn.has_label())
            .filter_map(|conn| conn.source())
            .find_map(|obj| GeometryMeshHandle::from_object(&obj).ok())
    }

    /// Returns the child material.
    #[inline]
    #[must_use]
    pub fn child_materials(&self) -> ChildMaterials<'a> {
        ChildMaterials {
            sources: self.as_object().source_objects(),
        }
    }
}

impl<'a> ObjectSubtypeHandle<'a> for ModelMeshHandle<'a> {
    type NodeId = ModelMeshNodeId;

    #[inline]
    fn from_object(object: &ObjectHandle<'a>) -> Result<Self> {
        AnyModelHandle::from_object(object).and_then(|model| Self::from_model(&model))
    }

    #[inline]
    fn as_object(&self) -> &ObjectHandle<'a> {
        &self.object.as_object()
    }

    #[inline]
    fn node_id(&self) -> Self::NodeId {
        ModelMeshNodeId(self.as_object().node_id())
    }
}

impl<'a> AsRef<ObjectHandle<'a>> for ModelMeshHandle<'a> {
    #[inline]
    fn as_ref(&self) -> &ObjectHandle<'a> {
        self.as_object()
    }
}

impl<'a> AsRef<AnyModelHandle<'a>> for ModelMeshHandle<'a> {
    #[inline]
    fn as_ref(&self) -> &AnyModelHandle<'a> {
        self.as_model()
    }
}

/// Iterator of `Material` nodes which are children of a `Model`(`Mesh`) node.
#[derive(Debug, Clone)]
pub struct ChildMaterials<'a> {
    /// Source objects.
    sources: ConnectionsForObject<'a>,
}

impl<'a> Iterator for ChildMaterials<'a> {
    type Item = AnyMaterialHandle<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.sources
            .by_ref()
            .filter(|conn| !conn.has_label())
            .filter_map(|conn| conn.source())
            .find_map(|obj| AnyMaterialHandle::from_object(&obj).ok())
    }
}
