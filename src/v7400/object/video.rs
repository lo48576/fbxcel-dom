//! Objects with `Video` class.

use crate::v7400::object::{ObjectHandle, ObjectId, ObjectNodeId, ObjectSubtypeHandle};
use crate::v7400::Result;

/// Node ID for a video object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VideoNodeId(ObjectNodeId);

/// Object handle for a video object.
#[derive(Debug, Clone, Copy)]
pub struct VideoHandle<'a> {
    /// Object handle.
    object: ObjectHandle<'a>,
}

impl<'a> VideoHandle<'a> {
    /// Returns the object ID.
    #[inline]
    #[must_use]
    pub fn object_id(&self) -> ObjectId {
        self.object.id()
    }
}

impl<'a> ObjectSubtypeHandle<'a> for VideoHandle<'a> {
    type NodeId = VideoNodeId;

    fn from_object(object: &ObjectHandle<'a>) -> Result<Self> {
        let class = object.class();
        if class != "Video" {
            return Err(error!(
                "not a model object: expected \"Video\" class but got {:?} class",
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
        VideoNodeId(self.object.node_id())
    }
}

impl<'a> AsRef<ObjectHandle<'a>> for VideoHandle<'a> {
    #[inline]
    fn as_ref(&self) -> &ObjectHandle<'a> {
        self.as_object()
    }
}
