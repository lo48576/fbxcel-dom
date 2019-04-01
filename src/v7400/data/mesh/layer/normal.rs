//! Normal.

use crate::v7400::data::mesh::layer::LayerElementHandle;

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
}

impl<'a> std::ops::Deref for LayerElementNormalHandle<'a> {
    type Target = LayerElementHandle<'a>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}
