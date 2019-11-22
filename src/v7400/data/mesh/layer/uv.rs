//! UV.

use anyhow::{format_err, Error};
use mint::Point2;

use crate::v7400::data::mesh::{
    layer::{
        LayerContentIndex, LayerElementHandle, MappingMode, ReferenceInformation, ReferenceMode,
    },
    TriangleVertexIndex, TriangleVertices,
};

/// Layer element node handle.
#[derive(Debug, Clone, Copy)]
pub struct LayerElementUvHandle<'a> {
    /// `LayerElementUv` node.
    node: LayerElementHandle<'a>,
}

impl<'a> LayerElementUvHandle<'a> {
    /// Creates a new `LayerElementUvHandle`.
    pub fn new(node: LayerElementHandle<'a>) -> Self {
        Self { node }
    }

    /// Returns `UV` data.
    pub fn uv(&self) -> Result<Uv<'a>, Error> {
        Uv::new(self)
    }

    /// Returns reference to the uv slice.
    fn uv_slice(&self) -> Result<&'a [f64], Error> {
        self.children_by_name("UV")
            .next()
            .ok_or_else(|| format_err!("No `UV` found for `LayerElementUV` node"))?
            .attributes()
            .get(0)
            .ok_or_else(|| format_err!("No attributes found for `UV` node"))?
            .get_arr_f64_or_type()
            .map_err(|ty| format_err!("Expected `[f64]` as UVs, but got {:?}", ty))
    }

    /// Returns reference to the uv index slice.
    fn uv_index_slice(&self) -> Result<&'a [i32], Error> {
        self.children_by_name("UVIndex")
            .next()
            .ok_or_else(|| format_err!("No `UVIndex` found for `LayerElementUV` node"))?
            .attributes()
            .get(0)
            .ok_or_else(|| format_err!("No attributes found for `UVIndex` node"))?
            .get_arr_i32_or_type()
            .map_err(|ty| format_err!("Expected `[i32]` as UV indices, but got {:?}", ty))
    }
}

impl<'a> std::ops::Deref for LayerElementUvHandle<'a> {
    type Target = LayerElementHandle<'a>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

/// UV.
#[derive(Debug, Clone, Copy)]
pub struct Uv<'a> {
    /// UV.
    uv: &'a [f64],
    /// Reference information.
    reference_info: ReferenceInformation<'a>,
    /// Mapping mode.
    mapping_mode: MappingMode,
}

impl<'a> Uv<'a> {
    /// Creates a new `Uv`.
    fn new(handle: &LayerElementUvHandle<'a>) -> Result<Self, Error> {
        let uv = handle.uv_slice()?;
        let mapping_mode = handle.mapping_mode()?;
        let reference_info = match handle.reference_mode()? {
            ReferenceMode::Direct => ReferenceInformation::Direct,
            ReferenceMode::IndexToDirect => {
                let uv_index = handle.uv_index_slice()?;
                ReferenceInformation::IndexToDirect(uv_index)
            }
        };

        Ok(Self {
            uv,
            reference_info,
            mapping_mode,
        })
    }

    /// Returns `[f64; 2]` uv corresponding to the given triangle vertex index.
    pub fn uv(
        &self,
        tris: &TriangleVertices<'a>,
        tri_vi: TriangleVertexIndex,
    ) -> Result<Point2<f64>, Error> {
        let i = LayerContentIndex::control_point_data_from_triangle_vertices(
            self.reference_info,
            self.mapping_mode,
            tris,
            self.uv.len() / 2,
            tri_vi,
        )?;
        Ok(Point2::from_slice(&self.uv[(i.get() * 2)..]))
    }
}
