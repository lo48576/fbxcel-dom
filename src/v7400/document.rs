//! Document-level structures.

mod load;
pub mod meta;

use fbxcel::tree::v7400::{NodeHandle, Tree};

pub use self::load::{LoadError, Loader};
pub use self::meta::DocumentMeta;

/// FBX document.
// This is intended to be a read-only structure as of writing this.
#[derive(Debug, Clone)]
pub struct Document {
    /// Low level tree.
    tree: Tree,
}

impl Document {
    /// Returns a reference to the lowlevel tree.
    #[inline]
    #[must_use]
    pub fn tree(&self) -> &Tree {
        &self.tree
    }

    /// Returns the root node.
    #[inline]
    #[must_use]
    fn root_node(&self) -> NodeHandle<'_> {
        self.tree.root()
    }

    /// Returns a proxy to the document metadata.
    #[inline]
    #[must_use]
    pub fn meta(&self) -> DocumentMeta<'_> {
        DocumentMeta::new(self)
    }
}

impl Document {
    /// Creates a new loader.
    #[inline]
    #[must_use]
    pub fn loader() -> Loader {
        Loader::new()
    }
}
