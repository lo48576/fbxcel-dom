//! Common stuff for layer elements.

use failure::{bail, format_err, Error};

use crate::{fbxcel::tree::v7400::NodeHandle, v7400::data::mesh::layer::LayerElementIndex};

/// Layer element node handle.
#[derive(Debug, Clone, Copy)]
pub struct LayerElementHandle<'a> {
    /// `LayerElement*` node under `Geometry`.
    node: NodeHandle<'a>,
}

impl<'a> LayerElementHandle<'a> {
    /// Returns a reference to the node handle.
    pub fn node(&self) -> &NodeHandle<'a> {
        &self.node
    }

    /// Returns type-local layer element index.
    pub fn typed_index(&self) -> Result<LayerElementIndex, Error> {
        let raw = self
            .node()
            .attributes()
            .get(0)
            .ok_or_else(|| format_err!("No attributes found for `{}` node", self.node().name()))?
            .get_i32_or_type()
            .map_err(|ty| format_err!("Expected `i32` as layer element index, but got {:?}", ty))?;
        if raw < 0 {
            bail!(
                "Expected non-negative integer as layer element index, but got {:?}",
                raw
            );
        }

        Ok(LayerElementIndex::new(raw as u32))
    }

    /// Retuns layer element name.
    ///
    /// This conflicts with `fbxcel::tree::v7400::NodeHandle::name()`.
    /// If you want to get node name, do `obj.node().name()` instead of
    /// `obj.name()`.
    pub fn name(&self) -> Result<&'a str, Error> {
        self.children_by_name("Name")
            .next()
            .ok_or_else(|| {
                format_err!(
                    "Child node `Name` not found for `{}` node",
                    self.node().name()
                )
            })?
            .attributes()
            .get(0)
            .ok_or_else(|| format_err!("No attributes found for `{}` node", self.node().name()))?
            .get_string_or_type()
            .map_err(|ty| format_err!("Expected string as layer element name, but got {:?}", ty))
    }
}

impl<'a> std::ops::Deref for LayerElementHandle<'a> {
    type Target = NodeHandle<'a>;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}
