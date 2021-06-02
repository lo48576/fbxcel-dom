//! Objects with `Model` class and `Null` subclass.

use crate::v7400::object::model::{ChildSkeletonNodes, ModelHandle, SkeletonHierarchyNode};
use crate::v7400::object::{ObjectHandle, ObjectId, ObjectNodeId, ObjectSubtypeHandle};
use crate::v7400::Result;

/// Node ID for a model object with subclass `Null`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelNullNodeId(ObjectNodeId);

/// Object handle for a model object with subclass `Null`.
#[derive(Debug, Clone, Copy)]
pub struct ModelNullHandle<'a> {
    /// Model handle.
    object: ModelHandle<'a>,
}

impl<'a> ModelNullHandle<'a> {
    /// Creates a model (null) handle from the given model handle.
    pub(super) fn from_model(object: &ModelHandle<'a>) -> Result<Self> {
        let subclass = object.as_object().subclass();
        if subclass != "Null" {
            return Err(error!(
                "not a `Model(Null)` object: expected \"Null\" subclass \
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
    pub fn as_model(&self) -> &ModelHandle<'a> {
        &self.object
    }
}

impl<'a> ModelNullHandle<'a> {
    /// Returns the parent model node.
    ///
    /// If there are two or more parent models, one of them is returned.
    /// If you want to get all of them, use [`ObjectHandle::destination_objects`]
    /// and filter by yourself.
    #[must_use]
    pub fn parent_skeleton_node(&self) -> Option<SkeletonHierarchyNode<'a>> {
        self.as_object()
            .destination_objects()
            .filter(|conn| !conn.has_label())
            .filter_map(|conn| conn.destination())
            .find_map(|obj| SkeletonHierarchyNode::from_object(&obj).ok())
    }

    /// Returns an iterator of the child limb nodes.
    #[inline]
    #[must_use]
    pub fn child_skeleton_nodes(&self) -> ChildSkeletonNodes<'a> {
        ChildSkeletonNodes::from_parent(self.as_object())
    }
}

impl<'a> ObjectSubtypeHandle<'a> for ModelNullHandle<'a> {
    type NodeId = ModelNullNodeId;

    #[inline]
    fn from_object(object: &ObjectHandle<'a>) -> Result<Self> {
        ModelHandle::from_object(object).and_then(|model| Self::from_model(&model))
    }

    #[inline]
    fn as_object(&self) -> &ObjectHandle<'a> {
        &self.object.as_object()
    }

    #[inline]
    fn node_id(&self) -> Self::NodeId {
        ModelNullNodeId(self.as_object().node_id())
    }
}

impl<'a> AsRef<ObjectHandle<'a>> for ModelNullHandle<'a> {
    #[inline]
    fn as_ref(&self) -> &ObjectHandle<'a> {
        self.as_object()
    }
}

impl<'a> AsRef<ModelHandle<'a>> for ModelNullHandle<'a> {
    #[inline]
    fn as_ref(&self) -> &ModelHandle<'a> {
        self.as_model()
    }
}
