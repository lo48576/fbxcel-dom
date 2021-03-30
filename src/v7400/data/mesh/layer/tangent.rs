//! Tangent.

use crate::v7400::data::mesh::layer::LayerElementHandle;

/// Tangent
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
}

impl<'a> std::ops::Deref for LayerElementTangentHandle<'a> {
    type Target = LayerElementHandle<'a>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}
