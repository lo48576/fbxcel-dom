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

    /// Returns the subclass.
    #[inline]
    #[must_use]
    pub fn subclass(&self) -> &'a str {
        self.object.subclass()
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

/// Typed video.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum TypedVideo<'a> {
    /// `Clip` subclass.
    Clip(VideoClipHandle<'a>),
}

impl<'a> TypedVideo<'a> {
    /// Converts a video into a handle with the type for its class.
    pub fn from_video(video: &AnyVideoHandle<'a>) -> Result<Self> {
        match video.subclass() {
            "Clip" => VideoClipHandle::from_video(video).map(Self::Clip),
            subclass => Err(error!(
                "unknown object subclass {:?} for `Video` class",
                subclass
            )),
        }
    }
}

impl<'a> From<VideoClipHandle<'a>> for TypedVideo<'a> {
    #[inline]
    fn from(v: VideoClipHandle<'a>) -> Self {
        Self::Clip(v)
    }
}
