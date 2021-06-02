//! Objects with `SubDeformer` class and `Cluster` subclass.

use crate::v7400::object::subdeformer::SubDeformerHandle;
use crate::v7400::object::{ObjectHandle, ObjectId, ObjectNodeId, ObjectSubtypeHandle};
use crate::v7400::Result;

/// Node ID for a subdeformer object with subclass `Cluster`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubDeformerClusterNodeId(ObjectNodeId);

/// Object handle for a subdeformer object with subclass `Cluster`.
#[derive(Debug, Clone, Copy)]
pub struct SubDeformerClusterHandle<'a> {
    /// SubDeformer handle.
    object: SubDeformerHandle<'a>,
}

impl<'a> SubDeformerClusterHandle<'a> {
    /// Creates a subdeformer (cluster) handle from the given subdeformer handle.
    fn from_subdeformer(object: &SubDeformerHandle<'a>) -> Result<Self> {
        let subclass = object.as_object().subclass();
        if subclass != "Cluster" {
            return Err(error!(
                "not a `SubDeformer(Cluster)` object: expected \"Cluster\" subclass \
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

impl<'a> ObjectSubtypeHandle<'a> for SubDeformerClusterHandle<'a> {
    type NodeId = SubDeformerClusterNodeId;

    #[inline]
    fn from_object(object: &ObjectHandle<'a>) -> Result<Self> {
        SubDeformerHandle::from_object(object)
            .and_then(|subdeformer| Self::from_subdeformer(&subdeformer))
    }

    #[inline]
    fn as_object(&self) -> &ObjectHandle<'a> {
        &self.object.as_object()
    }

    #[inline]
    fn node_id(&self) -> Self::NodeId {
        SubDeformerClusterNodeId(self.as_object().node_id())
    }
}