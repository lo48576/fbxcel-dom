//! Layers.

use std::convert::{TryFrom, TryInto};

use anyhow::{bail, format_err, Error};

use fbxcel::{low::v7400::AttributeValue, tree::v7400::NodeHandle};

pub(crate) use self::common::LayerContentIndex;
pub use self::{
    color::LayerElementColorHandle,
    common::{LayerElementHandle, MappingMode, ReferenceInformation, ReferenceMode},
    material::LayerElementMaterialHandle,
    normal::LayerElementNormalHandle,
    uv::LayerElementUvHandle,
};
use crate::v7400::data::mesh::layer::binormal::LayerElementBinormalHandle;
use crate::v7400::data::mesh::layer::smoothing::LayerElementSmoothingHandle;
use crate::v7400::data::mesh::layer::tangent::LayerElementTangentHandle;

pub mod binormal;
pub mod color;
mod common;
pub mod material;
pub mod normal;
pub mod smoothing;
pub mod tangent;
pub mod uv;

/// Layer node.
#[derive(Debug, Clone, Copy)]
pub struct LayerHandle<'a> {
    /// `Layer` node under `Geometry`.
    node: NodeHandle<'a>,
}

impl<'a> LayerHandle<'a> {
    /// Creates a new `LayerHandle`.
    pub(crate) fn new(node: NodeHandle<'a>) -> Self {
        Self { node }
    }

    /// Get layer index.
    pub fn get_index(&self) -> Result<LayerIndex, Error> {
        let raw = self
            .node
            .attributes()
            .get(0)
            .ok_or_else(|| format_err!("Attributes not found for `Layer` element"))?
            .get_i32_or_type()
            .map_err(|ty| format_err!("Expected `i32` as layer index, but got {:?}", ty))?;
        if raw < 0 {
            bail!(
                "Expected non-negative integer as layer index, but got {:?}",
                raw
            );
        }

        Ok(LayerIndex::new(raw as u32))
    }

    /// Returns an iterator of layer element entries.
    pub fn layer_element_entries(&self) -> impl Iterator<Item = LayerElementEntryHandle<'a>> {
        self.children_by_name("LayerElement")
            .map(LayerElementEntryHandle::new)
    }
}

impl<'a> std::ops::Deref for LayerHandle<'a> {
    type Target = NodeHandle<'a>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

/// Layer index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayerIndex(u32);

impl LayerIndex {
    /// Creates a new `LayerElement`.
    fn new(v: u32) -> Self {
        Self(v)
    }

    /// Returns the underlying value.
    pub fn to_u32(self) -> u32 {
        self.0
    }

    /// Returns the underlying value.
    #[deprecated(since = "0.0.3", note = "Renamed to `to_u32`")]
    pub fn get_u32(self) -> u32 {
        self.to_u32()
    }
}

/// Layer element entry node.
///
/// The nodes may be children of a `Layer` element.
/// They have simple metadata but not layer element content itself.
#[derive(Debug, Clone, Copy)]
pub struct LayerElementEntryHandle<'a> {
    /// `LayerElement` node under `Layer`.
    node: NodeHandle<'a>,
}

impl<'a> LayerElementEntryHandle<'a> {
    /// Creates a new `LayerElementEntryHandle` from the given node handle.
    fn new(node: NodeHandle<'a>) -> Self {
        Self { node }
    }

    /// Returns layer element type string.
    pub fn type_str(&self) -> Result<&'a str, Error> {
        self.children_by_name("Type")
            .next()
            .ok_or_else(|| format_err!("Child node `Type` not found for `LayerElement`"))?
            .attributes()
            .get(0)
            .ok_or_else(|| format_err!("Attributes not found for `Type`"))?
            .get_string_or_type()
            .map_err(|ty| format_err!("Expected string but got {:?}", ty))
    }

    /// Returns layer element type.
    pub fn type_(&self) -> Result<LayerElementType, Error> {
        self.type_str()?.parse()
    }

    /// Returns the layer element index in the same type.
    pub fn typed_index(&self) -> Result<LayerElementIndex, Error> {
        let raw = self
            .children_by_name("TypedIndex")
            .next()
            .ok_or_else(|| format_err!("Child node `TypedIndex` not found for `LayerElement`"))?
            .attributes()
            .get(0)
            .ok_or_else(|| format_err!("Attributes not found for `TypedIndex`"))?
            .get_i32_or_type()
            .map_err(|ty| format_err!("Expected `i32` but got {:?}", ty))?;
        if raw < 0 {
            bail!(
                "Expected non-negative integer as layer element index, but got {:?}",
                raw
            );
        }

        Ok(LayerElementIndex::new(raw as u32))
    }

    /// Returns typed layer element handle.
    pub fn typed_layer_element(&self) -> Result<TypedLayerElementHandle<'a>, Error> {
        let geometry_node = self.parent().and_then(|p| p.parent()).ok_or_else(|| {
            format_err!(
                "Failed to get parent of parent of `LayerElement` node, \
                 this is not supposed to happen"
            )
        })?;
        let ty = self.type_()?;
        let index = self.typed_index()?;
        geometry_node
            .children_by_name(ty.type_name())
            .find(|node| {
                node.attributes()
                    .get(0)
                    .and_then(AttributeValue::get_i32)
                    .map_or(false, |v| v == index.to_u32() as i32)
            })
            .ok_or_else(|| {
                format_err!(
                    "Layer element node not found: type={:?}, index={:?}",
                    ty,
                    index
                )
            })
            .map(|node| TypedLayerElementHandle::new(ty, node))
    }
}

impl<'a> std::ops::Deref for LayerElementEntryHandle<'a> {
    type Target = NodeHandle<'a>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

/// Layer element type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayerElementType {
    /// Color.
    Color,
    /// Material.
    Material,
    /// Normal.
    Normal,
    /// Binormal.
    Binormal,
    /// Tangent.
    Tangent,
    /// UV.
    Uv,
    /// Smoothing.
    Smoothing,
}

impl LayerElementType {
    /// Returns type name.
    pub fn type_name(self) -> &'static str {
        match self {
            LayerElementType::Color => "LayerElementColor",
            LayerElementType::Material => "LayerElementMaterial",
            LayerElementType::Normal => "LayerElementNormal",
            LayerElementType::Tangent => "LayerElementTangent",
            LayerElementType::Binormal => "LayerElementBinormal",
            LayerElementType::Uv => "LayerElementUV",
            LayerElementType::Smoothing => "LayerElementSmoothing",
        }
    }
}

impl TryFrom<&str> for LayerElementType {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "LayerElementColor" => Ok(LayerElementType::Color),
            "LayerElementMaterial" => Ok(LayerElementType::Material),
            "LayerElementNormal" => Ok(LayerElementType::Normal),
            "LayerElementBinormal" => Ok(LayerElementType::Binormal),
            "LayerElementTangent" => Ok(LayerElementType::Tangent),
            "LayerElementUV" => Ok(LayerElementType::Uv),
            "LayerElementSmoothing" => Ok(LayerElementType::Smoothing),
            _ => Err(format_err!("Unknown layer element type: {:?}", s)),
        }
    }
}

impl std::str::FromStr for LayerElementType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.try_into()
    }
}

/// Type-local layer element index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayerElementIndex(u32);

impl LayerElementIndex {
    /// Creates a new `LayerElementIndex`.
    fn new(v: u32) -> Self {
        Self(v)
    }

    /// Returns the underlying value.
    pub fn to_u32(self) -> u32 {
        self.0
    }

    /// Returns the underlying value.
    #[deprecated(since = "0.0.3", note = "Renamed to `to_u32`")]
    pub fn get_u32(self) -> u32 {
        self.to_u32()
    }
}

/// Typed layer element.
#[derive(Debug, Clone, Copy)]
pub enum TypedLayerElementHandle<'a> {
    /// Color.
    Color(LayerElementColorHandle<'a>),
    /// Material.
    Material(LayerElementMaterialHandle<'a>),
    /// Normal.
    Normal(LayerElementNormalHandle<'a>),
    /// Binormal.
    Binormal(LayerElementBinormalHandle<'a>),
    /// Tangent.
    Tangent(LayerElementTangentHandle<'a>),
    /// UV.
    Uv(LayerElementUvHandle<'a>),
    /// Smoothing.
    Smoothing(LayerElementSmoothingHandle<'a>),
}

impl<'a> TypedLayerElementHandle<'a> {
    /// Creates a new `TypedLayerElementHandle`.
    fn new(ty: LayerElementType, node: NodeHandle<'a>) -> Self {
        let base = LayerElementHandle::new(node);
        match ty {
            LayerElementType::Color => {
                TypedLayerElementHandle::Color(LayerElementColorHandle::new(base))
            }
            LayerElementType::Material => {
                TypedLayerElementHandle::Material(LayerElementMaterialHandle::new(base))
            }
            LayerElementType::Normal => {
                TypedLayerElementHandle::Normal(LayerElementNormalHandle::new(base))
            }
            LayerElementType::Binormal => {
                TypedLayerElementHandle::Binormal(LayerElementBinormalHandle::new(base))
            }
            LayerElementType::Tangent => {
                TypedLayerElementHandle::Tangent(LayerElementTangentHandle::new(base))
            }
            LayerElementType::Uv => TypedLayerElementHandle::Uv(LayerElementUvHandle::new(base)),
            LayerElementType::Smoothing => {
                TypedLayerElementHandle::Smoothing(LayerElementSmoothingHandle::new(base))
            }
        }
    }
}

impl<'a> std::ops::Deref for TypedLayerElementHandle<'a> {
    type Target = LayerElementHandle<'a>;

    fn deref(&self) -> &Self::Target {
        match self {
            TypedLayerElementHandle::Color(v) => &**v,
            TypedLayerElementHandle::Normal(v) => &**v,
            TypedLayerElementHandle::Binormal(v) => &**v,
            TypedLayerElementHandle::Tangent(v) => &**v,
            TypedLayerElementHandle::Material(v) => &**v,
            TypedLayerElementHandle::Uv(v) => &**v,
            TypedLayerElementHandle::Smoothing(v) => &**v,
        }
    }
}
