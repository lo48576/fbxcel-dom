//! Material.

use crate::v7400::data::mesh::layer::LayerElementHandle;

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
}

impl<'a> std::ops::Deref for LayerElementMaterialHandle<'a> {
    type Target = LayerElementHandle<'a>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}
