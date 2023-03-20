//! Tangent.

use anyhow::{bail, format_err, Error};
use mint::Vector3;

use crate::v7400::data::mesh::{
    layer::{
        LayerContentIndex, LayerElementHandle, MappingMode, ReferenceInformation, ReferenceMode,
    },
    TriangleVertexIndex, TriangleVertices,
};

/// Layer element node handle.
#[derive(Debug, Clone, Copy)]
pub struct LayerElementTangentHandle<'a> {
    /// `LayerElementTangent` node.
    node: LayerElementHandle<'a>,
}

impl<'a> LayerElementTangentHandle<'a> {
    /// Creates a new `LayerElementTangentHandle`.
    pub fn new(node: LayerElementHandle<'a>) -> Self {
        Self { node }
    }

    /// Returns `Tangents` data.
    pub fn tangents(&self) -> Result<Tangents<'a>, Error> {
        Tangents::new(self)
    }

    /// Returns reference to the tangents (xyz) slice.
    fn tangents_vec3_slice(&self) -> Result<&'a [f64], Error> {
        self.children_by_name("Tangents")
            .next()
            .ok_or_else(|| format_err!("No `Tangents` found for `LayerElementTangent` node"))?
            .attributes()
            .get(0)
            .ok_or_else(|| format_err!("No attributes found for `Tangents` node"))?
            .get_arr_f64_or_type()
            .map_err(|ty| format_err!("Expected `[f64]` as tangents, but got {:?}", ty))
    }
}

impl<'a> std::ops::Deref for LayerElementTangentHandle<'a> {
    type Target = LayerElementHandle<'a>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

/// Tangents.
#[derive(Debug, Clone, Copy)]
pub struct Tangents<'a> {
    /// Tangents.
    tangents: &'a [f64],
    /// Mapping mode.
    mapping_mode: MappingMode,
}

impl<'a> Tangents<'a> {
    /// Creates a new `Tangents`.
    fn new(handle: &LayerElementTangentHandle<'a>) -> Result<Self, Error> {
        let tangents = handle.tangents_vec3_slice()?;
        let mapping_mode = handle.mapping_mode()?;
        let reference_mode = handle.reference_mode()?;
        if reference_mode != ReferenceMode::Direct {
            bail!(
                "Unsupported reference mode for tangents: {:?}",
                reference_mode
            );
        }
        Ok(Self {
            tangents,
            mapping_mode,
        })
    }

    /// Returns `[f64; 3]` tangent corresponding to the given triangle vertex
    /// index.
    pub fn tangent(
        &self,
        tris: &TriangleVertices<'a>,
        tri_vi: TriangleVertexIndex,
    ) -> Result<Vector3<f64>, Error> {
        let i = LayerContentIndex::control_point_data_from_triangle_vertices(
            ReferenceInformation::Direct,
            self.mapping_mode,
            tris,
            self.tangents.len() / 3,
            tri_vi,
        )?;
        Ok(Vector3::from_slice(&self.tangents[(i.get() * 3)..]))
    }
}
