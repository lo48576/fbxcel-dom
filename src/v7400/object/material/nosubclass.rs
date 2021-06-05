//! Objects with `Material` class and empty subclass.

use crate::v7400::object::material::AnyMaterialHandle;
use crate::v7400::object::{ObjectHandle, ObjectId, ObjectNodeId, ObjectSubtypeHandle};
use crate::v7400::Result;

/// Node ID for a material object with empty subclass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaterialNodeId(ObjectNodeId);

/// Object handle for a material object with empty subclass.
#[derive(Debug, Clone, Copy)]
pub struct MaterialHandle<'a> {
    /// Material handle.
    object: AnyMaterialHandle<'a>,
}

impl<'a> MaterialHandle<'a> {
    /// Creates a material handle from the given material handle.
    pub fn from_material(object: &AnyMaterialHandle<'a>) -> Result<Self> {
        let subclass = object.subclass();
        if !subclass.is_empty() {
            return Err(error!(
                "not a `Material` (with empty subclass) object: expected empty \
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

    /// Returns the reference to the more generic material handle.
    #[inline]
    #[must_use]
    pub fn as_material(&self) -> &AnyMaterialHandle<'a> {
        &self.object
    }
}

impl<'a> ObjectSubtypeHandle<'a> for MaterialHandle<'a> {
    type NodeId = MaterialNodeId;

    #[inline]
    fn from_object(object: &ObjectHandle<'a>) -> Result<Self> {
        AnyMaterialHandle::from_object(object).and_then(|material| Self::from_material(&material))
    }

    #[inline]
    fn as_object(&self) -> &ObjectHandle<'a> {
        &self.object.as_object()
    }

    #[inline]
    fn node_id(&self) -> Self::NodeId {
        MaterialNodeId(self.as_object().node_id())
    }
}

impl<'a> AsRef<ObjectHandle<'a>> for MaterialHandle<'a> {
    #[inline]
    fn as_ref(&self) -> &ObjectHandle<'a> {
        self.as_object()
    }
}

impl<'a> AsRef<AnyMaterialHandle<'a>> for MaterialHandle<'a> {
    #[inline]
    fn as_ref(&self) -> &AnyMaterialHandle<'a> {
        self.as_material()
    }
}
