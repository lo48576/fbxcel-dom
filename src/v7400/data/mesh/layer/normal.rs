//! Normal.

use failure::{bail, format_err, Error};

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
    fn normals_norm_slice(&self) -> Result<&'a [f64], Error> {
        self.children_by_name("NormalsW")
            .next()
            .ok_or_else(|| format_err!("No `NormalsW` not found for `LayerElementNormal` node"))?
            .attributes()
            .get(0)
            .ok_or_else(|| format_err!("No attributes found for `NormalsW` node"))?
            .get_arr_f64_or_type()
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
    normals_w: &'a [f64],
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
    pub fn get_xyz_f64_by_tri_vi(
        &self,
        tris: &TriangleVertices<'a>,
        tri_vi: TriangleVertexIndex,
    ) -> Result<[f64; 3], Error> {
        let i = LayerContentIndex::control_ponint_data_from_triangle_vertices(
            ReferenceInformation::Direct,
            self.mapping_mode,
            tris,
            self.normals.len() / 3,
            tri_vi,
        )?;
        let i3 = i.get() * 3;
        Ok([self.normals[i3], self.normals[i3 + 1], self.normals[i3 + 2]])
    }

    /// Returns `[f32; 3]` normal corresponding to the given triangle vertex
    /// index.
    pub fn get_xyz_f32_by_tri_vi(
        &self,
        tris: &TriangleVertices<'a>,
        tri_vi: TriangleVertexIndex,
    ) -> Result<[f32; 3], Error> {
        self.get_xyz_f64_by_tri_vi(tris, tri_vi)
            .map(|[x, y, z]| [x as f32, y as f32, z as f32])
    }
}
