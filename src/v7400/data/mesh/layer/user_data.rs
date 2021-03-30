//! UserData

use crate::v7400::data::mesh::layer::LayerElementHandle;

/// UserData
#[derive(Debug, Clone, Copy)]
pub struct LayerElementUserDataHandle<'a> {
    /// `LayerElementUserData` node.
    node: LayerElementHandle<'a>,
}

impl<'a> LayerElementUserDataHandle<'a> {
    /// Creates a new `LayerElementUserDataHandle`.
    pub fn new(node: LayerElementHandle<'a>) -> Self {
        Self { node }
    }
}

impl<'a> std::ops::Deref for LayerElementUserDataHandle<'a> {
    type Target = LayerElementHandle<'a>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}
