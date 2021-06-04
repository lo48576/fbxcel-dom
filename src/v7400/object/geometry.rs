//! Objects with `Geometry` class.

mod mesh;

use crate::v7400::object::{ObjectHandle, ObjectId, ObjectNodeId, ObjectSubtypeHandle};
use crate::v7400::Result;

pub use self::mesh::{GeometryMeshHandle, GeometryMeshNodeId};

/// Node ID for a geometry object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnyGeometryNodeId(ObjectNodeId);

/// Object handle for a geometry object.
#[derive(Debug, Clone, Copy)]
pub struct AnyGeometryHandle<'a> {
    /// Object handle.
    object: ObjectHandle<'a>,
}

impl<'a> AnyGeometryHandle<'a> {
    /// Returns the object ID.
    #[inline]
    #[must_use]
    pub fn object_id(&self) -> ObjectId {
        self.object.id()
    }
}

impl<'a> ObjectSubtypeHandle<'a> for AnyGeometryHandle<'a> {
    type NodeId = AnyGeometryNodeId;

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
        AnyGeometryNodeId(self.object.node_id())
    }
}

impl<'a> AsRef<ObjectHandle<'a>> for AnyGeometryHandle<'a> {
    #[inline]
    fn as_ref(&self) -> &ObjectHandle<'a> {
        self.as_object()
    }
}

/// Subclass of a geometry known to the fbxcel-dom crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum GeometrySubclass {
    /// `Mesh` subclass.
    Mesh,
}
