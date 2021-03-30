//! PolygonGroup

use crate::v7400::data::mesh::layer::LayerElementHandle;

/// PolygonGroup
#[derive(Debug, Clone, Copy)]
pub struct LayerElementPolygonGroupHandle<'a> {
    /// `LayerElementPolygonGroup` node.
    node: LayerElementHandle<'a>,
}

impl<'a> LayerElementPolygonGroupHandle<'a> {
    /// Creates a new `LayerElementPolygonGroupHandle`.
    pub fn new(node: LayerElementHandle<'a>) -> Self {
        Self { node }
    }
}

impl<'a> std::ops::Deref for LayerElementPolygonGroupHandle<'a> {
    type Target = LayerElementHandle<'a>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}
