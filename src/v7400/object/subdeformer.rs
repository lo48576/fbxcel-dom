//! Objects with `SubDeformer` class.

mod cluster;

use crate::v7400::object::{ObjectHandle, ObjectId, ObjectNodeId, ObjectSubtypeHandle};
use crate::v7400::Result;

pub use self::cluster::{SubDeformerClusterHandle, SubDeformerClusterNodeId};

/// Node ID for a subdeformer object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnySubDeformerNodeId(ObjectNodeId);

/// Object handle for a subdeformer object.
#[derive(Debug, Clone, Copy)]
pub struct AnySubDeformerHandle<'a> {
    /// Object handle.
    object: ObjectHandle<'a>,
}

impl<'a> AnySubDeformerHandle<'a> {
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

impl<'a> ObjectSubtypeHandle<'a> for AnySubDeformerHandle<'a> {
    type NodeId = AnySubDeformerNodeId;

    fn from_object(object: &ObjectHandle<'a>) -> Result<Self> {
        let class = object.class();
        if class != "SubDeformer" {
            return Err(error!(
                "not a model object: expected \"SubDeformer\" class but got {:?} class",
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
        AnySubDeformerNodeId(self.object.node_id())
    }
}

impl<'a> AsRef<ObjectHandle<'a>> for AnySubDeformerHandle<'a> {
    #[inline]
    fn as_ref(&self) -> &ObjectHandle<'a> {
        self.as_object()
    }
}

/// Subclass of a deformer known to the fbxcel-dom crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SubDeformerSubclass {
    /// `Cluster` subclass.
    Cluster,
}

/// Typed subdeformer.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum TypedSubDeformer<'a> {
    /// `Cluster` subclass.
    Cluster(SubDeformerClusterHandle<'a>),
}

impl<'a> TypedSubDeformer<'a> {
    /// Converts a subdeformer into a handle with the type for its class.
    pub fn from_subdeformer(subdeformer: &AnySubDeformerHandle<'a>) -> Result<Self> {
        match subdeformer.subclass() {
            "Cluster" => SubDeformerClusterHandle::from_subdeformer(subdeformer).map(Self::Cluster),
            subclass => Err(error!(
                "unknown object subclass {:?} for `SubDeformer` class",
                subclass
            )),
        }
    }
}

impl<'a> From<SubDeformerClusterHandle<'a>> for TypedSubDeformer<'a> {
    #[inline]
    fn from(v: SubDeformerClusterHandle<'a>) -> Self {
        Self::Cluster(v)
    }
}
