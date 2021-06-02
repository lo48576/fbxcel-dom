//! Objects with `Model` class.

pub mod limb_node;
pub mod mesh;
mod null;

use crate::v7400::connection::ConnectionsForObject;
use crate::v7400::object::{ObjectHandle, ObjectId, ObjectNodeId, ObjectSubtypeHandle};
use crate::v7400::Result;

pub use self::limb_node::{ModelLimbNodeHandle, ModelLimbNodeNodeId};
pub use self::mesh::{ModelMeshHandle, ModelMeshNodeId};
pub use self::null::{ModelNullHandle, ModelNullNodeId};

/// Node ID for a model object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelNodeId(ObjectNodeId);

/// Object handle for a model object.
#[derive(Debug, Clone, Copy)]
pub struct ModelHandle<'a> {
    /// Object handle.
    object: ObjectHandle<'a>,
}

impl<'a> ModelHandle<'a> {
    /// Returns the object ID.
    #[inline]
    #[must_use]
    pub fn object_id(&self) -> ObjectId {
        self.object.id()
    }

    /// Returns the parent model if available.
    ///
    /// If there are two or more parent models, one of them is returned.
    /// If you want to get all of them, use [`ObjectHandle::destination_objects`]
    /// and filter by yourself.
    #[must_use]
    pub fn parent_model(&self) -> Option<Self> {
        self.object
            .destination_objects()
            .filter(|conn| !conn.has_label())
            .filter_map(|conn| conn.destination())
            .find_map(|obj| Self::from_object(&obj).ok())
    }

    /// Returns an iterator of the child models.
    #[inline]
    #[must_use]
    pub fn child_models(&self) -> ModelChildren<'a> {
        ModelChildren {
            sources: self.object.source_objects(),
        }
    }
}

impl<'a> ObjectSubtypeHandle<'a> for ModelHandle<'a> {
    type NodeId = ModelNodeId;

    fn from_object(object: &ObjectHandle<'a>) -> Result<Self> {
        let class = object.class();
        if class != "Model" {
            return Err(error!(
                "not a model object: expected \"Model\" class but got {:?} class",
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
        ModelNodeId(self.object.node_id())
    }
}

impl<'a> AsRef<ObjectHandle<'a>> for ModelHandle<'a> {
    #[inline]
    fn as_ref(&self) -> &ObjectHandle<'a> {
        self.as_object()
    }
}

/// A node which constitutes hierarchy of a skeleton.
///
/// Specifically, a `Model` node whose subclass is `LimbNode` or `Null`.
#[derive(Debug, Clone, Copy)]
pub enum SkeletonHierarchyNode<'a> {
    /// Limb Node.
    LimbNode(ModelLimbNodeHandle<'a>),
    /// Null.
    Null(ModelNullHandle<'a>),
}

impl<'a> SkeletonHierarchyNode<'a> {
    /// Creates a value from the given object handle.
    fn from_object(object: &ObjectHandle<'a>) -> Result<Self> {
        ModelHandle::from_object(object).and_then(|model| match model.as_object().subclass() {
            "LimbNode" => ModelLimbNodeHandle::from_model(&model).map(Self::LimbNode),
            "Null" => ModelNullHandle::from_model(&model).map(Self::Null),
            subclass => Err(error!(
                "expected subclass `LimbNode` or `Null`, but got {:?}",
                subclass
            )),
        })
    }
}

/// Iterator of `Model` nodes which are children of another `Model` node.
#[derive(Debug, Clone)]
pub struct ModelChildren<'a> {
    /// Source objects.
    sources: ConnectionsForObject<'a>,
}

impl<'a> Iterator for ModelChildren<'a> {
    type Item = ModelHandle<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.sources
            .by_ref()
            .filter(|conn| !conn.has_label())
            .filter_map(|conn| conn.source())
            .find_map(|obj| ModelHandle::from_object(&obj).ok())
    }
}

/// Iterator of skeleton nodes which are children of another skeleton node.
#[derive(Debug, Clone)]
pub struct ChildSkeletonNodes<'a> {
    /// Source objects.
    sources: ConnectionsForObject<'a>,
}

impl<'a> ChildSkeletonNodes<'a> {
    /// Creates an iterator from the given parent object.
    #[inline]
    #[must_use]
    fn from_parent(parent: &ObjectHandle<'a>) -> Self {
        ChildSkeletonNodes {
            sources: parent.source_objects(),
        }
    }
}

impl<'a> Iterator for ChildSkeletonNodes<'a> {
    type Item = ModelLimbNodeHandle<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.sources
            .by_ref()
            .filter(|conn| !conn.has_label())
            .filter_map(|conn| conn.source())
            .find_map(|obj| ModelLimbNodeHandle::from_object(&obj).ok())
    }
}
