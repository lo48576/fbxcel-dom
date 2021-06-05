//! Objects with `Video` class and `Clip` subclass.

use crate::v7400::object::video::AnyVideoHandle;
use crate::v7400::object::{ObjectHandle, ObjectId, ObjectNodeId, ObjectSubtypeHandle};
use crate::v7400::Result;

/// Node ID for a video object with subclass `Clip`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VideoClipNodeId(ObjectNodeId);

/// Object handle for a video object with subclass `Clip`.
#[derive(Debug, Clone, Copy)]
pub struct VideoClipHandle<'a> {
    /// Video handle.
    object: AnyVideoHandle<'a>,
}

impl<'a> VideoClipHandle<'a> {
    /// Creates a video (clip) handle from the given video handle.
    fn from_video(object: &AnyVideoHandle<'a>) -> Result<Self> {
        let subclass = object.subclass();
        if subclass != "Clip" {
            return Err(error!(
                "not a `Video(Clip)` object: expected \"Clip\" subclass \
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

    /// Returns the reference to the more generic video handle.
    #[inline]
    #[must_use]
    pub fn as_video(&self) -> &AnyVideoHandle<'a> {
        &self.object
    }
}

impl<'a> ObjectSubtypeHandle<'a> for VideoClipHandle<'a> {
    type NodeId = VideoClipNodeId;

    #[inline]
    fn from_object(object: &ObjectHandle<'a>) -> Result<Self> {
        AnyVideoHandle::from_object(object).and_then(|video| Self::from_video(&video))
    }

    #[inline]
    fn as_object(&self) -> &ObjectHandle<'a> {
        &self.object.as_object()
    }

    #[inline]
    fn node_id(&self) -> Self::NodeId {
        VideoClipNodeId(self.as_object().node_id())
    }
}

impl<'a> AsRef<ObjectHandle<'a>> for VideoClipHandle<'a> {
    #[inline]
    fn as_ref(&self) -> &ObjectHandle<'a> {
        self.as_object()
    }
}

impl<'a> AsRef<AnyVideoHandle<'a>> for VideoClipHandle<'a> {
    #[inline]
    fn as_ref(&self) -> &AnyVideoHandle<'a> {
        self.as_video()
    }
}
