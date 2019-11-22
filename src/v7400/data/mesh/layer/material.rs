//! Material.

use anyhow::{bail, format_err, Error};

use crate::v7400::data::mesh::{
    layer::{
        LayerContentIndex, LayerElementHandle, MappingMode, ReferenceInformation, ReferenceMode,
    },
    TriangleVertexIndex, TriangleVertices,
};

/// Layer element node handle.
#[derive(Debug, Clone, Copy)]
pub struct LayerElementMaterialHandle<'a> {
    /// `LayerElementMaterial` node.
    node: LayerElementHandle<'a>,
}

impl<'a> LayerElementMaterialHandle<'a> {
    /// Creates a new `LayerElementMaterialHandle`.
    pub fn new(node: LayerElementHandle<'a>) -> Self {
        Self { node }
    }

    /// Returns `Materials` data.
    pub fn materials(&self) -> Result<Materials<'a>, Error> {
        Materials::new(self)
    }

    /// Returns material indices slice.
    fn material_indices_slice(&self) -> Result<&'a [i32], Error> {
        self.children_by_name("Materials")
            .next()
            .ok_or_else(|| format_err!("No `Materials` found for `LayerElementMaterial` node"))?
            .attributes()
            .get(0)
            .ok_or_else(|| format_err!("No attributes found for `Materials` node"))?
            .get_arr_i32_or_type()
            .map_err(|ty| format_err!("Expected `[i32]` as material indices, but got {:?}", ty))
    }
}

impl<'a> std::ops::Deref for LayerElementMaterialHandle<'a> {
    type Target = LayerElementHandle<'a>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

/// Materials.
#[derive(Debug, Clone, Copy)]
pub struct Materials<'a> {
    /// Mapping mode.
    mapping_mode: MappingMode,
    /// Reference information.
    indices: &'a [i32],
}

impl<'a> Materials<'a> {
    /// Creates a new `Materials`.
    fn new(handle: &LayerElementMaterialHandle<'a>) -> Result<Self, Error> {
        let mapping_mode = handle.mapping_mode()?;
        let reference_mode = handle.reference_mode()?;
        if reference_mode != ReferenceMode::IndexToDirect {
            bail!(
                "Unsupported reference mode for material: {:?}",
                reference_mode
            );
        }
        let indices = handle.material_indices_slice()?;

        Ok(Self {
            mapping_mode,
            indices,
        })
    }

    /// Returns material index corresponding to the given triangle vertex index.
    pub fn material_index(
        &self,
        tris: &TriangleVertices<'a>,
        tri_vi: TriangleVertexIndex,
    ) -> Result<MaterialIndex, Error> {
        let i = LayerContentIndex::control_point_data_from_triangle_vertices(
            ReferenceInformation::Direct,
            self.mapping_mode,
            tris,
            self.indices.len(),
            tri_vi,
        )?;
        let material_index_index = self.indices[i.get()];
        if material_index_index < 0 {
            bail!(
                "Negative index is not allowed: material_index_index={:?}",
                material_index_index
            );
        }

        Ok(MaterialIndex::new(material_index_index as u32))
    }
}

/// Material index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MaterialIndex(u32);

impl MaterialIndex {
    /// Creates a new `MaterialIndex`.
    fn new(i: u32) -> Self {
        Self(i)
    }

    /// Returns the material index.
    pub fn to_u32(self) -> u32 {
        self.0
    }

    /// Returns the material index.
    #[deprecated(since = "0.0.3", note = "Renamed to `to_u32`")]
    pub fn get_u32(self) -> u32 {
        self.to_u32()
    }
}
