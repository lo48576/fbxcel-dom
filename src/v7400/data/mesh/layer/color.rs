//! Color.

use anyhow::{format_err, Error};

use crate::v7400::data::mesh::{
    layer::{
        LayerContentIndex, LayerElementHandle, MappingMode, ReferenceInformation, ReferenceMode,
    },
    TriangleVertexIndex, TriangleVertices,
};

/// Layer element node handle.
#[derive(Debug, Clone, Copy)]
pub struct LayerElementColorHandle<'a> {
    /// `LayerElementColor` node.
    node: LayerElementHandle<'a>,
}

impl<'a> LayerElementColorHandle<'a> {
    /// Creates a new `LayerElementColorHandle`.
    pub fn new(node: LayerElementHandle<'a>) -> Self {
        Self { node }
    }

    /// Returns `Color` data.
    pub fn color(&self) -> Result<Colors<'a>, Error> {
        Colors::new(self)
    }

    /// Returns reference to the colors slice.
    fn colors_slice(&self) -> Result<&'a [f64], Error> {
        self.children_by_name("Colors")
            .next()
            .ok_or_else(|| format_err!("No `Colors` found for `LayerElementColor` node"))?
            .attributes()
            .get(0)
            .ok_or_else(|| format_err!("No attributes found for `Colors` node"))?
            .get_arr_f64_or_type()
            .map_err(|ty| format_err!("Expected `[f64]` as colors, but got {:?}", ty))
    }

    /// Returns reference to the colors index slice.
    // NOTE: I (the author) am not sure `ColorsIndex` node really exists, but
    // it would be better to implement this rather than rejecting `ColorsIndex`
    // (if it exists).
    fn colors_index_slice(&self) -> Result<&'a [i32], Error> {
        self.children_by_name("ColorsIndex")
            .next()
            .ok_or_else(|| format_err!("No `ColorsIndex` found for `LayerElementColor` node"))?
            .attributes()
            .get(0)
            .ok_or_else(|| format_err!("No attributes found for `ColorsIndex` node"))?
            .get_arr_i32_or_type()
            .map_err(|ty| format_err!("Expected `[i32]` as color indices, but got {:?}", ty))
    }
}

impl<'a> std::ops::Deref for LayerElementColorHandle<'a> {
    type Target = LayerElementHandle<'a>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

/// Colors.
#[derive(Debug, Clone, Copy)]
pub struct Colors<'a> {
    /// Colors.
    colors: &'a [f64],
    /// Reference information.
    reference_info: ReferenceInformation<'a>,
    /// Mapping mode.
    mapping_mode: MappingMode,
}

impl<'a> Colors<'a> {
    /// Creates a new `Colors`.
    fn new(handle: &LayerElementColorHandle<'a>) -> Result<Self, Error> {
        let colors = handle.colors_slice()?;
        let mapping_mode = handle.mapping_mode()?;
        let reference_info = match handle.reference_mode()? {
            ReferenceMode::Direct => ReferenceInformation::Direct,
            ReferenceMode::IndexToDirect => {
                let colors_index = handle.colors_index_slice()?;
                ReferenceInformation::IndexToDirect(colors_index)
            }
        };

        Ok(Self {
            colors,
            reference_info,
            mapping_mode,
        })
    }

    /// Returns `[f64; 4]` color corresponding to the given triangle vertex
    /// index.
    pub fn color(
        &self,
        tris: &TriangleVertices<'a>,
        tri_vi: TriangleVertexIndex,
    ) -> Result<[f64; 4], Error> {
        let i = LayerContentIndex::control_point_data_from_triangle_vertices(
            self.reference_info,
            self.mapping_mode,
            tris,
            self.colors.len() / 4,
            tri_vi,
        )?;
        let i4 = i.get() * 4;
        Ok([
            self.colors[i4],
            self.colors[i4 + 1],
            self.colors[i4 + 2],
            self.colors[i4 + 3],
        ])
    }
}
