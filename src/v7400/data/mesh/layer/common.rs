//! Common stuff for layer elements.

use std::convert::{TryFrom, TryInto};

use failure::{bail, format_err, Error};

use crate::{
    fbxcel::tree::v7400::NodeHandle,
    v7400::data::mesh::{layer::LayerElementIndex, TriangleVertexIndex, TriangleVertices},
};

/// Layer element node handle.
#[derive(Debug, Clone, Copy)]
pub struct LayerElementHandle<'a> {
    /// `LayerElement*` node under `Geometry`.
    node: NodeHandle<'a>,
}

impl<'a> LayerElementHandle<'a> {
    /// Creates a new `LayerElementHandle`.
    pub(crate) fn new(node: NodeHandle<'a>) -> Self {
        Self { node }
    }

    /// Returns a reference to the node handle.
    pub fn node(&self) -> &NodeHandle<'a> {
        &self.node
    }

    /// Returns type-local layer element index.
    pub fn typed_index(&self) -> Result<LayerElementIndex, Error> {
        let raw = self
            .node()
            .attributes()
            .get(0)
            .ok_or_else(|| format_err!("No attributes found for `{}` node", self.node().name()))?
            .get_i32_or_type()
            .map_err(|ty| format_err!("Expected `i32` as layer element index, but got {:?}", ty))?;
        if raw < 0 {
            bail!(
                "Expected non-negative integer as layer element index, but got {:?}",
                raw
            );
        }

        Ok(LayerElementIndex::new(raw as u32))
    }

    /// Retuns layer element name.
    ///
    /// This conflicts with `fbxcel::tree::v7400::NodeHandle::name()`.
    /// If you want to get node name, do `obj.node().name()` instead of
    /// `obj.name()`.
    pub fn name(&self) -> Result<&'a str, Error> {
        self.children_by_name("Name")
            .next()
            .ok_or_else(|| {
                format_err!(
                    "Child node `Name` not found for `{}` node",
                    self.node().name()
                )
            })?
            .attributes()
            .get(0)
            .ok_or_else(|| format_err!("No attributes found for `{}` node", self.node().name()))?
            .get_string_or_type()
            .map_err(|ty| format_err!("Expected string as layer element name, but got {:?}", ty))
    }

    /// Returns mapping mode.
    pub fn mapping_mode(&self) -> Result<MappingMode, Error> {
        self.children_by_name("MappingInformationType")
            .next()
            .ok_or_else(|| {
                format_err!(
                    "Child node `MappingInformationType` not found for `{}` node",
                    self.node().name()
                )
            })?
            .attributes()
            .get(0)
            .ok_or_else(|| format_err!("No attributes found for `{}` node", self.node().name()))?
            .get_string_or_type()
            .map_err(|ty| format_err!("Expected string as layer element name, but got {:?}", ty))
            .and_then(str::parse)
    }

    /// Returns reference mode.
    pub fn reference_mode(&self) -> Result<ReferenceMode, Error> {
        self.children_by_name("ReferenceInformationType")
            .next()
            .ok_or_else(|| {
                format_err!(
                    "Child node `ReferenceInformationType` not found for `{}` node",
                    self.node().name()
                )
            })?
            .attributes()
            .get(0)
            .ok_or_else(|| format_err!("No attributes found for `{}` node", self.node().name()))?
            .get_string_or_type()
            .map_err(|ty| format_err!("Expected string as layer element name, but got {:?}", ty))
            .and_then(str::parse)
    }
}

impl<'a> std::ops::Deref for LayerElementHandle<'a> {
    type Target = NodeHandle<'a>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

/// Mapping mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MappingMode {
    /// Undetermined.
    None,
    /// By ontrol point.
    ByControlPoint,
    /// By polygon vertex.
    ByPolygonVertex,
    /// By polygon.
    ByPolygon,
    /// By edge.
    ByEdge,
    /// Single value for all.
    AllSame,
}

impl TryFrom<&str> for MappingMode {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "ByControlPoint" | "ByVertex" | "ByVertice" => Ok(MappingMode::ByControlPoint),
            "ByPolygonVertex" => Ok(MappingMode::ByPolygonVertex),
            "ByPolygon" => Ok(MappingMode::ByPolygon),
            "ByEdge" => Ok(MappingMode::ByEdge),
            "AllSame" => Ok(MappingMode::AllSame),
            s => Err(format_err!("Failed to parse mapping mode: got {:?}", s)),
        }
    }
}

impl std::str::FromStr for MappingMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.try_into()
    }
}

/// Reference mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReferenceMode {
    /// Direct.
    Direct,
    /// Index to direct.
    IndexToDirect,
}

impl TryFrom<&str> for ReferenceMode {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Direct" => Ok(ReferenceMode::Direct),
            "IndexToDirect" => Ok(ReferenceMode::IndexToDirect),
            s => Err(format_err!("Failed to parse reference mode: got {:?}", s)),
        }
    }
}

impl std::str::FromStr for ReferenceMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.try_into()
    }
}

/// Reference information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReferenceInformation<'a> {
    /// Direct.
    Direct,
    /// Index to direct.
    IndexToDirect(&'a [i32]),
}

impl ReferenceInformation<'_> {
    /// Returns direct index.
    pub(crate) fn get_direct(&self, i: usize) -> Result<LayerContentIndex, Error> {
        match self {
            ReferenceInformation::Direct => Ok(LayerContentIndex::new(i)),
            ReferenceInformation::IndexToDirect(indices) => {
                let direct = indices.get(i).cloned().ok_or_else(|| {
                    format_err!(
                        "Index out of range: indices.len()={:?}, i={:?}",
                        indices.len(),
                        i
                    )
                })?;
                if direct < 0 {
                    bail!("Negative index is not allowed: direct={:?}", direct);
                }
                Ok(LayerContentIndex::new(direct as usize))
            }
        }
    }
}

impl From<ReferenceInformation<'_>> for ReferenceMode {
    fn from(v: ReferenceInformation<'_>) -> Self {
        match v {
            ReferenceInformation::Direct => ReferenceMode::Direct,
            ReferenceInformation::IndexToDirect(_) => ReferenceMode::IndexToDirect,
        }
    }
}

/// Index of value in a layer element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct LayerContentIndex(usize);

impl LayerContentIndex {
    /// Creates a new `LayerContentIndex`.
    pub(crate) fn new(i: usize) -> Self {
        Self(i)
    }

    /// Returns the index.
    pub(crate) fn get(self) -> usize {
        self.0
    }

    /// Returns the layer content index for the corresponding control point.
    pub(crate) fn control_ponint_data_from_triangle_vertices(
        reference_info: ReferenceInformation<'_>,
        mapping_mode: MappingMode,
        triangle_vertices: &TriangleVertices<'_>,
        layer_element_array_len: usize,
        tri_vi: TriangleVertexIndex,
    ) -> Result<LayerContentIndex, Error> {
        let index = match mapping_mode {
            MappingMode::None | MappingMode::ByEdge => bail!("Unsupported mapping mode: {:?}"),
            MappingMode::ByControlPoint => {
                let cpi = triangle_vertices
                    .control_point_index(tri_vi)
                    .ok_or_else(|| {
                        format_err!("Failed to get control point index: tri_vi={:?}", tri_vi)
                    })?;
                reference_info.get_direct(cpi.to_u32() as usize)?
            }
            MappingMode::ByPolygonVertex => {
                let pvi = triangle_vertices
                    .polygon_vertex_index(tri_vi)
                    .ok_or_else(|| {
                        format_err!("Failed to get polygon vertex index: tri_vi={:?}", tri_vi)
                    })?;
                reference_info.get_direct(pvi.to_usize() as usize)?
            }
            MappingMode::ByPolygon => {
                let poly_i = triangle_vertices
                    .get_polygon_index(tri_vi.triangle_index())
                    .ok_or_else(|| {
                        format_err!("Failed to get polygon vertex index: tri_vi={:?}", tri_vi)
                    })?;
                reference_info.get_direct(poly_i.to_usize())?
            }
            MappingMode::AllSame => reference_info.get_direct(0)?,
        };
        if index.get() >= layer_element_array_len {
            bail!(
                "Calculated index out of range: index={:?}, array_len={:?}",
                index,
                layer_element_array_len
            );
        }

        Ok(index)
    }
}
