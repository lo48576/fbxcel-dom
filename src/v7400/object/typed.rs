//! Types for typed objects.

use super::Result;
use crate::v7400::object::deformer::{AnyDeformerHandle, DeformerSubclass};
use crate::v7400::object::geometry::{AnyGeometryHandle, GeometrySubclass};
use crate::v7400::object::material::{AnyMaterialHandle, MaterialSubclass};
use crate::v7400::object::model::{AnyModelHandle, ModelSubclass};
use crate::v7400::object::subdeformer::{AnySubDeformerHandle, SubDeformerSubclass};
use crate::v7400::object::texture::{AnyTextureHandle, TextureSubclass};
use crate::v7400::object::video::{AnyVideoHandle, VideoSubclass};
use crate::v7400::object::ObjectSubtypeHandle as _;
use crate::v7400::ObjectHandle;

/// Object class known to the fbxcel-dom crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Class {
    /// `Deformer` class.
    Deformer,
    /// `Geometry` class.
    Geometry,
    /// `Material` class.
    Material,
    /// `Model` class.
    Model,
    /// `SubDeformer` class.
    SubDeformer,
    /// `Texture` class.
    Texture,
    /// `Video` class.
    Video,
}

/// Object class known to the fbxcel-dom crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Subclass {
    /// Subclass for `Deformer`.
    Deformer(DeformerSubclass),
    /// Subclass for `Geometry`.
    Geometry(GeometrySubclass),
    /// Subclass for `Material`.
    Material(MaterialSubclass),
    /// Subclass for `Model`.
    Model(ModelSubclass),
    /// Subclass for `SubDeformer`.
    SubDeformer(SubDeformerSubclass),
    /// Subclass for `Texture`.
    Texture(TextureSubclass),
    /// Subclass for `Video`.
    Video(VideoSubclass),
}

impl From<Subclass> for Class {
    fn from(sub: Subclass) -> Self {
        match sub {
            Subclass::Deformer(_) => Self::Deformer,
            Subclass::Geometry(_) => Self::Geometry,
            Subclass::Material(_) => Self::Material,
            Subclass::Model(_) => Self::Model,
            Subclass::SubDeformer(_) => Self::SubDeformer,
            Subclass::Texture(_) => Self::Texture,
            Subclass::Video(_) => Self::Video,
        }
    }
}

macro_rules! impl_from_for_subclass {
    ($variant:ident, $ty_subclass:ident) => {
        impl From<$ty_subclass> for Subclass {
            #[inline]
            fn from(v: $ty_subclass) -> Self {
                Self::$variant(v)
            }
        }
    };
}

impl_from_for_subclass!(Deformer, DeformerSubclass);
impl_from_for_subclass!(Geometry, GeometrySubclass);
impl_from_for_subclass!(Material, MaterialSubclass);
impl_from_for_subclass!(Model, ModelSubclass);
impl_from_for_subclass!(SubDeformer, SubDeformerSubclass);
impl_from_for_subclass!(Texture, TextureSubclass);
impl_from_for_subclass!(Video, VideoSubclass);

/// Typed object.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum TypedObject<'a> {
    /// Object for `Deformer`.
    Deformer(AnyDeformerHandle<'a>),
    /// Object for `Geometry`.
    Geometry(AnyGeometryHandle<'a>),
    /// Object for `Material`.
    Material(AnyMaterialHandle<'a>),
    /// Object for `Model`.
    Model(AnyModelHandle<'a>),
    /// Object for `SubDeformer`.
    SubDeformer(AnySubDeformerHandle<'a>),
    /// Object for `Texture`.
    Texture(AnyTextureHandle<'a>),
    /// Object for `Video`.
    Video(AnyVideoHandle<'a>),
}

impl<'a> TypedObject<'a> {
    /// Converts an object into a handle with the type for its class.
    pub fn from_object(object: &ObjectHandle<'a>) -> Result<Self> {
        match object.class() {
            "Deformer" => AnyDeformerHandle::from_object(object).map(Self::Deformer),
            "Geometry" => AnyGeometryHandle::from_object(object).map(Self::Geometry),
            "Material" => AnyMaterialHandle::from_object(object).map(Self::Material),
            "Model" => AnyModelHandle::from_object(object).map(Self::Model),
            "SubDeformer" => AnySubDeformerHandle::from_object(object).map(Self::SubDeformer),
            "Texture" => AnyTextureHandle::from_object(object).map(Self::Texture),
            "Video" => AnyVideoHandle::from_object(object).map(Self::Video),
            class => Err(error!("unknown object class {:?}", class)),
        }
    }
}

macro_rules! impl_from_for_typed_object {
    ($variant:ident, $ty_handle:ident) => {
        impl<'a> From<$ty_handle<'a>> for TypedObject<'a> {
            #[inline]
            fn from(v: $ty_handle<'a>) -> Self {
                Self::$variant(v)
            }
        }
    };
}

impl_from_for_typed_object!(Deformer, AnyDeformerHandle);
impl_from_for_typed_object!(Geometry, AnyGeometryHandle);
impl_from_for_typed_object!(Material, AnyMaterialHandle);
impl_from_for_typed_object!(Model, AnyModelHandle);
impl_from_for_typed_object!(SubDeformer, AnySubDeformerHandle);
impl_from_for_typed_object!(Texture, AnyTextureHandle);
impl_from_for_typed_object!(Video, AnyVideoHandle);
