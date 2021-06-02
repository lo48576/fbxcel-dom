//! Objects with `Model` class and `LimbNode` subclass.

use crate::v7400::object::model::ModelHandle;
use crate::v7400::object::{ObjectHandle, ObjectId, ObjectNodeId, ObjectSubtypeHandle};
use crate::v7400::Result;

/// Node ID for a model object with subclass `LimbNode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelLimbNodeNodeId(ObjectNodeId);

/// Object handle for a model object with subclass `LimbNode`.
#[derive(Debug, Clone, Copy)]
pub struct ModelLimbNodeHandle<'a> {
    /// Model handle.
    object: ModelHandle<'a>,
}

impl<'a> ModelLimbNodeHandle<'a> {
    /// Creates a model (limb node) handle from the given model handle.
    fn from_model(object: &ModelHandle<'a>) -> Result<Self> {
        let subclass = object.as_object().subclass();
        if subclass != "LimbNode" {
            return Err(error!(
                "not a `Model(LimbNode)` object: expected \"LimbNode\" subclass \
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

impl<'a> ObjectSubtypeHandle<'a> for ModelLimbNodeHandle<'a> {
    type NodeId = ModelLimbNodeNodeId;

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
        ModelLimbNodeNodeId(self.as_object().node_id())
    }
}
