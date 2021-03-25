//! Smoothing.

use crate::v7400::data::mesh::layer::LayerElementHandle;

/// Smoothing
#[derive(Debug, Clone, Copy)]
pub struct LayerElementSmoothingHandle<'a> {
    /// `LayerElementSmoothing` node.
    node: LayerElementHandle<'a>,
}

impl<'a> LayerElementSmoothingHandle<'a> {
    /// Creates a new `LayerElementSmoothingHandle`.
    pub fn new(node: LayerElementHandle<'a>) -> Self {
        Self { node }
    }
}

impl<'a> std::ops::Deref for LayerElementSmoothingHandle<'a> {
    type Target = LayerElementHandle<'a>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}
