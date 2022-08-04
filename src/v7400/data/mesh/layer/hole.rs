//! Hole

use crate::v7400::data::mesh::layer::LayerElementHandle;

/// Hole
#[derive(Debug, Clone, Copy)]
pub struct LayerElementHoleHandle<'a> {
    /// `LayerElementHole` node.
    node: LayerElementHandle<'a>,
}

impl<'a> LayerElementHoleHandle<'a> {
    /// Creates a new `LayerElementHoleHandle`.
    pub fn new(node: LayerElementHandle<'a>) -> Self {
        Self { node }
    }
}

impl<'a> std::ops::Deref for LayerElementHoleHandle<'a> {
    type Target = LayerElementHandle<'a>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}
