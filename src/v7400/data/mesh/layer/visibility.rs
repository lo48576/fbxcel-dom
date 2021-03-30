//! Visibility

use crate::v7400::data::mesh::layer::LayerElementHandle;

/// Visibility
#[derive(Debug, Clone, Copy)]
pub struct LayerElementVisibilityHandle<'a> {
    /// `LayerElementVisibility` node.
    node: LayerElementHandle<'a>,
}

impl<'a> LayerElementVisibilityHandle<'a> {
    /// Creates a new `LayerElementVisibilityHandle`.
    pub fn new(node: LayerElementHandle<'a>) -> Self {
        Self { node }
    }
}

impl<'a> std::ops::Deref for LayerElementVisibilityHandle<'a> {
    type Target = LayerElementHandle<'a>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}
