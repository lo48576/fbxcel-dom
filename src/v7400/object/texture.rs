//! Objects with `Texture` class.

use crate::v7400::object::video::VideoClipHandle;
use crate::v7400::object::{ObjectHandle, ObjectId, ObjectNodeId, ObjectSubtypeHandle};
use crate::v7400::Result;

/// Node ID for a texture object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextureNodeId(ObjectNodeId);

/// Object handle for a texture object.
#[derive(Debug, Clone, Copy)]
pub struct TextureHandle<'a> {
    /// Object handle.
    object: ObjectHandle<'a>,
}

impl<'a> TextureHandle<'a> {
    /// Returns the object ID.
    #[inline]
    #[must_use]
    pub fn object_id(&self) -> ObjectId {
        self.object.id()
    }
}

impl<'a> TextureHandle<'a> {
    /// Returns the child video clip.
    ///
    /// If there are two or more child video clips, one of them is returned.
    /// If you want to get all of them, use [`ObjectHandle::source_objects`]
    /// and filter by yourself.
    #[must_use]
    pub fn child_video_clip(&self) -> Option<VideoClipHandle<'a>> {
        self.as_object()
            .source_objects()
            .filter(|conn| !conn.has_label())
            .filter_map(|conn| conn.source())
            .find_map(|obj| VideoClipHandle::from_object(&obj).ok())
    }
}

impl<'a> ObjectSubtypeHandle<'a> for TextureHandle<'a> {
    type NodeId = TextureNodeId;

    fn from_object(object: &ObjectHandle<'a>) -> Result<Self> {
        let class = object.class();
        if class != "Texture" {
            return Err(error!(
                "not a model object: expected \"Texture\" class but got {:?} class",
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
        TextureNodeId(self.object.node_id())
    }
}

impl<'a> AsRef<ObjectHandle<'a>> for TextureHandle<'a> {
    #[inline]
    fn as_ref(&self) -> &ObjectHandle<'a> {
        self.as_object()
    }
}
