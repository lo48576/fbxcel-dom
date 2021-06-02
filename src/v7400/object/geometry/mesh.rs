//! Objects with `Geometry` class and `Mesh` subclass.

use crate::v7400::object::deformer::DeformerSkinHandle;
use crate::v7400::object::geometry::GeometryHandle;
use crate::v7400::object::model::ModelMeshHandle;
use crate::v7400::object::{ObjectHandle, ObjectId, ObjectNodeId, ObjectSubtypeHandle};
use crate::v7400::Result;

/// Node ID for a geometry object with subclass `Mesh`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeometryMeshNodeId(ObjectNodeId);

/// Object handle for a geometry object with subclass `Mesh`.
#[derive(Debug, Clone, Copy)]
pub struct GeometryMeshHandle<'a> {
    /// Geometry handle.
    object: GeometryHandle<'a>,
}

impl<'a> GeometryMeshHandle<'a> {
    /// Creates a geometry (mesh) handle from the given geometry handle.
    fn from_geometry(object: &GeometryHandle<'a>) -> Result<Self> {
        let subclass = object.as_object().subclass();
        if subclass != "Mesh" {
            return Err(error!(
                "not a `Geometry(Mesh)` object: expected \"Mesh\" subclass \
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

    /// Returns the reference to the more generic geometry handle.
    #[inline]
    #[must_use]
    pub fn as_geometry(&self) -> &GeometryHandle<'a> {
        &self.object
    }
}

impl<'a> GeometryMeshHandle<'a> {
    /// Returns the parent model mesh node.
    ///
    /// If there are two or more parent models, one of them is returned.
    /// If you want to get all of them, use [`ObjectHandle::destination_objects`]
    /// and filter by yourself.
    // NOTE: I (the author) am not sure the parent `Model`(`Mesh`) object
    // is just one.
    pub fn parent_model_mesh(&self) -> Result<ModelMeshHandle<'a>> {
        self.as_object()
            .destination_objects()
            .filter(|conn| !conn.has_label())
            .filter_map(|conn| conn.destination())
            .find_map(|obj| ModelMeshHandle::from_object(&obj).ok())
            .ok_or_else(|| {
                error!(
                    "`Geometry(Mesh)` object is expected to have \
                    a parent `Model(Mesh)` object, but not found"
                )
            })
    }

    /// Returns the child skin node.
    ///
    /// If there are two or more child skins, one of them is returned.
    /// If you want to get all of them, use [`ObjectHandle::source_objects`]
    /// and filter by yourself.
    // NOTE: I (the author) am not sure the number of child `Deformer`(`Skin`)
    // object is at most one.
    #[must_use]
    pub fn child_deformer_skin(&self) -> Option<DeformerSkinHandle<'a>> {
        self.as_object()
            .source_objects()
            .filter(|conn| !conn.has_label())
            .filter_map(|conn| conn.source())
            .find_map(|obj| DeformerSkinHandle::from_object(&obj).ok())
    }
}

impl<'a> ObjectSubtypeHandle<'a> for GeometryMeshHandle<'a> {
    type NodeId = GeometryMeshNodeId;

    #[inline]
    fn from_object(object: &ObjectHandle<'a>) -> Result<Self> {
        GeometryHandle::from_object(object).and_then(|geometry| Self::from_geometry(&geometry))
    }

    #[inline]
    fn as_object(&self) -> &ObjectHandle<'a> {
        &self.object.as_object()
    }

    #[inline]
    fn node_id(&self) -> Self::NodeId {
        GeometryMeshNodeId(self.as_object().node_id())
    }
}
