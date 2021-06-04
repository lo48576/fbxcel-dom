//! Types for typed objects.

use crate::v7400::object::deformer::DeformerSubclass;
use crate::v7400::object::geometry::GeometrySubclass;
use crate::v7400::object::material::MaterialSubclass;
use crate::v7400::object::model::ModelSubclass;
use crate::v7400::object::subdeformer::SubDeformerSubclass;
use crate::v7400::object::texture::TextureSubclass;
use crate::v7400::object::video::VideoSubclass;

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
