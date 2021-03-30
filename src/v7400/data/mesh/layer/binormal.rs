//! Binormal

use crate::v7400::data::mesh::layer::LayerElementHandle;

/// Binormal
#[derive(Debug, Clone, Copy)]
pub struct LayerElementBinormalHandle<'a> {
    /// `LayerElementBinormal` node.
    node: LayerElementHandle<'a>,
}

impl<'a> LayerElementBinormalHandle<'a> {
    /// Creates a new `LayerElementBinormalHandle`.
    pub fn new(node: LayerElementHandle<'a>) -> Self {
        Self { node }
    }
}

impl<'a> std::ops::Deref for LayerElementBinormalHandle<'a> {
    type Target = LayerElementHandle<'a>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}
