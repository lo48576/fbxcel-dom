//! Objects with `Video` class.

mod clip;

use crate::v7400::object::{ObjectHandle, ObjectId, ObjectNodeId, ObjectSubtypeHandle};
use crate::v7400::Result;

pub use self::clip::{VideoClipHandle, VideoClipNodeId};

/// Node ID for a video object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnyVideoNodeId(ObjectNodeId);

/// Object handle for a video object.
#[derive(Debug, Clone, Copy)]
pub struct AnyVideoHandle<'a> {
    /// Object handle.
    object: ObjectHandle<'a>,
}

impl<'a> AnyVideoHandle<'a> {
    /// Returns the object ID.
    #[inline]
    #[must_use]
    pub fn object_id(&self) -> ObjectId {
        self.object.id()
    }
}

impl<'a> ObjectSubtypeHandle<'a> for AnyVideoHandle<'a> {
    type NodeId = AnyVideoNodeId;

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
        AnyVideoNodeId(self.object.node_id())
    }
}

impl<'a> AsRef<ObjectHandle<'a>> for AnyVideoHandle<'a> {
    #[inline]
    fn as_ref(&self) -> &ObjectHandle<'a> {
        self.as_object()
    }
}

/// Subclass of a video known to the fbxcel-dom crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum VideoSubclass {
    /// `Clip` subclass.
    Clip,
}
