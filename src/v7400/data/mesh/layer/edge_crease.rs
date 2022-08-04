//! EdgeCrease

use crate::v7400::data::mesh::layer::LayerElementHandle;

/// EdgeCrease
#[derive(Debug, Clone, Copy)]
pub struct LayerElementEdgeCreaseHandle<'a> {
    /// `LayerElementEdgeCrease` node.
    node: LayerElementHandle<'a>,
}

impl<'a> LayerElementEdgeCreaseHandle<'a> {
    /// Creates a new `LayerElementEdgeCreaseHandle`.
    pub fn new(node: LayerElementHandle<'a>) -> Self {
        Self { node }
    }
}

impl<'a> std::ops::Deref for LayerElementEdgeCreaseHandle<'a> {
    type Target = LayerElementHandle<'a>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}
