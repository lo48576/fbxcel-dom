//! Objects with `SubDeformer` class and `Cluster` subclass.

use crate::v7400::object::deformer::DeformerSkinHandle;
use crate::v7400::object::model::ModelLimbNodeHandle;
use crate::v7400::object::subdeformer::AnySubDeformerHandle;
use crate::v7400::object::{ObjectHandle, ObjectId, ObjectNodeId, ObjectSubtypeHandle};
use crate::v7400::Result;

/// Node ID for a subdeformer object with subclass `Cluster`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubDeformerClusterNodeId(ObjectNodeId);

/// Object handle for a subdeformer object with subclass `Cluster`.
#[derive(Debug, Clone, Copy)]
pub struct SubDeformerClusterHandle<'a> {
    /// SubDeformer handle.
    object: AnySubDeformerHandle<'a>,
}

impl<'a> SubDeformerClusterHandle<'a> {
    /// Creates a subdeformer (cluster) handle from the given subdeformer handle.
    fn from_subdeformer(object: &AnySubDeformerHandle<'a>) -> Result<Self> {
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

    /// Returns the reference to the more generic subdeformer handle.
    #[inline]
    #[must_use]
    pub fn as_subdeformer(&self) -> &AnySubDeformerHandle<'a> {
        &self.object
    }
}

impl<'a> SubDeformerClusterHandle<'a> {
    /// Returns the parent model node.
    ///
    /// If there are two or more parent models, one of them is returned.
    /// If you want to get all of them, use [`ObjectHandle::destination_objects`]
    /// and filter by yourself.
    #[must_use]
    pub fn parent_skeleton_node(&self) -> Option<DeformerSkinHandle<'a>> {
        self.as_object()
            .destination_objects()
            .filter(|conn| !conn.has_label())
            .filter_map(|conn| conn.destination())
            .find_map(|obj| DeformerSkinHandle::from_object(&obj).ok())
    }

    /// Returns the child limb node.
    ///
    /// If there are two or more child limb nodes, one of them is returned.
    /// If you want to get all of them, use [`ObjectHandle::source_objects`]
    /// and filter by yourself.
    #[must_use]
    pub fn child_limb_node(&self) -> Option<ModelLimbNodeHandle<'a>> {
        self.as_object()
            .source_objects()
            .filter(|conn| !conn.has_label())
            .filter_map(|conn| conn.source())
            .find_map(|obj| ModelLimbNodeHandle::from_object(&obj).ok())
    }
}

impl<'a> ObjectSubtypeHandle<'a> for SubDeformerClusterHandle<'a> {
    type NodeId = SubDeformerClusterNodeId;

    #[inline]
    fn from_object(object: &ObjectHandle<'a>) -> Result<Self> {
        AnySubDeformerHandle::from_object(object)
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

impl<'a> AsRef<ObjectHandle<'a>> for SubDeformerClusterHandle<'a> {
    #[inline]
    fn as_ref(&self) -> &ObjectHandle<'a> {
        self.as_object()
    }
}

impl<'a> AsRef<AnySubDeformerHandle<'a>> for SubDeformerClusterHandle<'a> {
    #[inline]
    fn as_ref(&self) -> &AnySubDeformerHandle<'a> {
        self.as_subdeformer()
    }
}
