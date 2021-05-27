//! Objects with `Model` class.

use crate::v7400::connection::ConnectionsForObject;
use crate::v7400::object::{ObjectHandle, ObjectId, ObjectNodeId, ObjectSubtypeHandle};
use crate::v7400::Result;

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
