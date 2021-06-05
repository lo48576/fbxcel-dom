//! Objects with `Deformer` class.

pub mod skin;

use crate::v7400::object::{ObjectHandle, ObjectId, ObjectNodeId, ObjectSubtypeHandle};
use crate::v7400::Result;

pub use self::skin::{DeformerSkinHandle, DeformerSkinNodeId};

/// Node ID for a deformer object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnyDeformerNodeId(ObjectNodeId);

/// Object handle for a deformer object.
#[derive(Debug, Clone, Copy)]
pub struct AnyDeformerHandle<'a> {
    /// Object handle.
    object: ObjectHandle<'a>,
}

impl<'a> AnyDeformerHandle<'a> {
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

impl<'a> ObjectSubtypeHandle<'a> for AnyDeformerHandle<'a> {
    type NodeId = AnyDeformerNodeId;

    fn from_object(object: &ObjectHandle<'a>) -> Result<Self> {
        let class = object.class();
        if class != "Deformer" {
            return Err(error!(
                "not a model object: expected \"Deformer\" class but got {:?} class",
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
        AnyDeformerNodeId(self.object.node_id())
    }
}

impl<'a> AsRef<ObjectHandle<'a>> for AnyDeformerHandle<'a> {
    #[inline]
    fn as_ref(&self) -> &ObjectHandle<'a> {
        self.as_object()
    }
}

/// Subclass of a deformer known to the fbxcel-dom crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DeformerSubclass {
    /// `Skin` subclass.
    Skin,
}

/// Typed deformer.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum TypedDeformer<'a> {
    /// `Skin` subclass.
    Skin(DeformerSkinHandle<'a>),
}

impl<'a> TypedDeformer<'a> {
    /// Converts a deformer into a handle with the type for its class.
    pub fn from_deformer(deformer: &AnyDeformerHandle<'a>) -> Result<Self> {
        match deformer.subclass() {
            "Skin" => DeformerSkinHandle::from_deformer(deformer).map(Self::Skin),
            subclass => Err(error!(
                "unknown object subclass {:?} for `Deformer` class",
                subclass
            )),
        }
    }
}

impl<'a> From<DeformerSkinHandle<'a>> for TypedDeformer<'a> {
    #[inline]
    fn from(v: DeformerSkinHandle<'a>) -> Self {
        Self::Skin(v)
    }
}
