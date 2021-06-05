//! Objects with `Texture` class and empty subclass.

use crate::v7400::object::texture::AnyTextureHandle;
use crate::v7400::object::{ObjectHandle, ObjectId, ObjectNodeId, ObjectSubtypeHandle};
use crate::v7400::Result;

/// Node ID for a texture object with empty subclass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextureNodeId(ObjectNodeId);

/// Object handle for a texture object with empty subclass.
#[derive(Debug, Clone, Copy)]
pub struct TextureHandle<'a> {
    /// Texture handle.
    object: AnyTextureHandle<'a>,
}

impl<'a> TextureHandle<'a> {
    /// Creates a texture handle from the given texture handle.
    pub fn from_texture(object: &AnyTextureHandle<'a>) -> Result<Self> {
        let subclass = object.subclass();
        if !subclass.is_empty() {
            return Err(error!(
                "not a `Texture` (with empty subclass) object: expected empty \
                subclass but got {:?} subclass",
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

    /// Returns the reference to the more generic texture handle.
    #[inline]
    #[must_use]
    pub fn as_texture(&self) -> &AnyTextureHandle<'a> {
        &self.object
    }
}

impl<'a> ObjectSubtypeHandle<'a> for TextureHandle<'a> {
    type NodeId = TextureNodeId;

    #[inline]
    fn from_object(object: &ObjectHandle<'a>) -> Result<Self> {
        AnyTextureHandle::from_object(object).and_then(|texture| Self::from_texture(&texture))
    }

    #[inline]
    fn as_object(&self) -> &ObjectHandle<'a> {
        &self.object.as_object()
    }

    #[inline]
    fn node_id(&self) -> Self::NodeId {
        TextureNodeId(self.as_object().node_id())
    }
}

impl<'a> AsRef<ObjectHandle<'a>> for TextureHandle<'a> {
    #[inline]
    fn as_ref(&self) -> &ObjectHandle<'a> {
        self.as_object()
    }
}

impl<'a> AsRef<AnyTextureHandle<'a>> for TextureHandle<'a> {
    #[inline]
    fn as_ref(&self) -> &AnyTextureHandle<'a> {
        self.as_texture()
    }
}
