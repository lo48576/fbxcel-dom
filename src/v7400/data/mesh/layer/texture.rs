//! Texture

use crate::v7400::data::mesh::layer::LayerElementHandle;

/// Texture
#[derive(Debug, Clone, Copy)]
pub struct LayerElementTextureHandle<'a> {
    /// `LayerElementTexture` node.
    node: LayerElementHandle<'a>,
}

impl<'a> LayerElementTextureHandle<'a> {
    /// Creates a new `LayerElementTextureHandle`.
    pub fn new(node: LayerElementHandle<'a>) -> Self {
        Self { node }
    }
}

impl<'a> std::ops::Deref for LayerElementTextureHandle<'a> {
    type Target = LayerElementHandle<'a>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}
