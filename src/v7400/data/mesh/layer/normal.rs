//! Normal.

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
pub struct LayerElementNormalHandle<'a> {
    /// `LayerElementNormal` node.
    node: LayerElementHandle<'a>,
}

impl<'a> LayerElementNormalHandle<'a> {
    /// Creates a new `LayerElementNormalHandle`.
    pub fn new(node: LayerElementHandle<'a>) -> Self {
        Self { node }
    }

    /// Returns `Normals` data.
    pub fn normals(&self) -> Result<Normals<'a>, Error> {
        Normals::new(self)
    }

    /// Returns reference to the normals (xyz) slice.
    fn normals_vec3_slice(&self) -> Result<&'a [f64], Error> {
        self.children_by_name("Normals")
            .next()
            .ok_or_else(|| format_err!("No `Normals` found for `LayerElementNormal` node"))?
            .attributes()
            .get(0)
            .ok_or_else(|| format_err!("No attributes found for `Normals` node"))?
            .get_arr_f64_or_type()
            .map_err(|ty| format_err!("Expected `[f64]` as normals, but got {:?}", ty))
    }

    /// Returns reference to the normals norms (w = `sqrt(x*x + y*y + z*z)`)
    /// slice.
    ///
    /// It is not guaranteed to be correct value.
    /// Use with care, especially if you are using untrusted data.
    fn normals_norm_slice(&self) -> Result<Option<&'a [f64]>, Error> {
        let normals_w_node = match self.children_by_name("NormalsW").next() {
            Some(v) => v,
            None => return Ok(None),
        };
        normals_w_node
            .attributes()
            .get(0)
            .ok_or_else(|| format_err!("No attributes found for `NormalsW` node"))?
            .get_arr_f64_or_type()
            .map(Some)
            .map_err(|ty| format_err!("Expected `[f64]` as normals W, but got {:?}", ty))
    }
}

impl<'a> std::ops::Deref for LayerElementNormalHandle<'a> {
    type Target = LayerElementHandle<'a>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

/// Normals.
#[derive(Debug, Clone, Copy)]
pub struct Normals<'a> {
    /// Normals.
    normals: &'a [f64],
    /// Normals W.
    normals_w: Option<&'a [f64]>,
    /// Mapping mode.
    mapping_mode: MappingMode,
}

impl<'a> Normals<'a> {
    /// Creates a new `Normals`.
    fn new(handle: &LayerElementNormalHandle<'a>) -> Result<Self, Error> {
        let normals = handle.normals_vec3_slice()?;
        let normals_w = handle.normals_norm_slice()?;
        let mapping_mode = handle.mapping_mode()?;
        let reference_mode = handle.reference_mode()?;
        if reference_mode != ReferenceMode::Direct {
            bail!(
                "Unsupported reference mode for normals: {:?}",
                reference_mode
            );
        }
        Ok(Self {
            normals,
            normals_w,
            mapping_mode,
        })
    }

    /// Returns `[f64; 3]` normal corresponding to the given triangle vertex
    /// index.
    pub fn normal(
        &self,
        tris: &TriangleVertices<'a>,
        tri_vi: TriangleVertexIndex,
    ) -> Result<Vector3<f64>, Error> {
        let i = LayerContentIndex::control_point_data_from_triangle_vertices(
            ReferenceInformation::Direct,
            self.mapping_mode,
            tris,
            self.normals.len() / 3,
            tri_vi,
        )?;
        Ok(Vector3::from_slice(&self.normals[(i.get() * 3)..]))
    }
}
