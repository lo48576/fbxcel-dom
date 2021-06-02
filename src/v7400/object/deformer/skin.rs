//! Objects with `Deformer` class and `Skin` subclass.

use crate::v7400::object::deformer::DeformerHandle;
use crate::v7400::object::{ObjectHandle, ObjectId, ObjectNodeId, ObjectSubtypeHandle};
use crate::v7400::Result;

/// Node ID for a deformer object with subclass `Skin`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeformerSkinNodeId(ObjectNodeId);

/// Object handle for a deformer object with subclass `Skin`.
#[derive(Debug, Clone, Copy)]
pub struct DeformerSkinHandle<'a> {
    /// Deformer handle.
    object: DeformerHandle<'a>,
}

impl<'a> DeformerSkinHandle<'a> {
    /// Creates a deformer (skin) handle from the given deformer handle.
    fn from_deformer(object: &DeformerHandle<'a>) -> Result<Self> {
        let subclass = object.as_object().subclass();
        if subclass != "Skin" {
            return Err(error!(
                "not a `Deformer(Skin)` object: expected \"Skin\" subclass \
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
}

impl<'a> ObjectSubtypeHandle<'a> for DeformerSkinHandle<'a> {
    type NodeId = DeformerSkinNodeId;

    #[inline]
    fn from_object(object: &ObjectHandle<'a>) -> Result<Self> {
        DeformerHandle::from_object(object).and_then(|deformer| Self::from_deformer(&deformer))
    }

    #[inline]
    fn as_object(&self) -> &ObjectHandle<'a> {
        &self.object.as_object()
    }

    #[inline]
    fn node_id(&self) -> Self::NodeId {
        DeformerSkinNodeId(self.as_object().node_id())
    }
}
