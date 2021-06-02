//! Objects with `SubDeformer` class.

use crate::v7400::object::{ObjectHandle, ObjectId, ObjectNodeId, ObjectSubtypeHandle};
use crate::v7400::Result;

/// Node ID for a subdeformer object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubDeformerNodeId(ObjectNodeId);

/// Object handle for a subdeformer object.
#[derive(Debug, Clone, Copy)]
pub struct SubDeformerHandle<'a> {
    /// Object handle.
    object: ObjectHandle<'a>,
}

impl<'a> SubDeformerHandle<'a> {
    /// Returns the object ID.
    #[inline]
    #[must_use]
    pub fn object_id(&self) -> ObjectId {
        self.object.id()
    }
}

impl<'a> ObjectSubtypeHandle<'a> for SubDeformerHandle<'a> {
    type NodeId = SubDeformerNodeId;

    fn from_object(object: &ObjectHandle<'a>) -> Result<Self> {
        let class = object.class();
        if class != "SubDeformer" {
            return Err(error!(
                "not a model object: expected \"SubDeformer\" class but got {:?} class",
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
        SubDeformerNodeId(self.object.node_id())
    }
}
