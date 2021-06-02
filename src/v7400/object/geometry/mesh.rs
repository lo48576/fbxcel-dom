//! Objects with `Geometry` class and `Mesh` subclass.

use crate::v7400::object::geometry::GeometryHandle;
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
