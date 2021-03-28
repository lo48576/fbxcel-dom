//! VertexCrease

use crate::v7400::data::mesh::layer::LayerElementHandle;

/// VertexCrease
#[derive(Debug, Clone, Copy)]
pub struct LayerElementVertexCreaseHandle<'a> {
    /// `LayerElementVertexCrease` node.
    node: LayerElementHandle<'a>,
}

impl<'a> LayerElementVertexCreaseHandle<'a> {
    /// Creates a new `LayerElementVertexCreaseHandle`.
    pub fn new(node: LayerElementHandle<'a>) -> Self {
        Self { node }
    }
}

impl<'a> std::ops::Deref for LayerElementVertexCreaseHandle<'a> {
    type Target = LayerElementHandle<'a>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}
