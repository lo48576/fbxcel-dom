//! Objects with `Geometry` class.

mod mesh;

use crate::v7400::object::{ObjectHandle, ObjectId, ObjectNodeId, ObjectSubtypeHandle};
use crate::v7400::Result;

pub use self::mesh::{GeometryMeshHandle, GeometryMeshNodeId};

/// Node ID for a geometry object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeometryNodeId(ObjectNodeId);

/// Object handle for a geometry object.
#[derive(Debug, Clone, Copy)]
pub struct GeometryHandle<'a> {
    /// Object handle.
    object: ObjectHandle<'a>,
}

impl<'a> GeometryHandle<'a> {
    /// Returns the object ID.
    #[inline]
    #[must_use]
    pub fn object_id(&self) -> ObjectId {
        self.object.id()
    }
}

impl<'a> ObjectSubtypeHandle<'a> for GeometryHandle<'a> {
    type NodeId = GeometryNodeId;

    fn from_object(object: &ObjectHandle<'a>) -> Result<Self> {
        let class = object.class();
        if class != "Geometry" {
            return Err(error!(
                "not a model object: expected \"Geometry\" class but got {:?} class",
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
        GeometryNodeId(self.object.node_id())
    }
}
