//! Document-level structures.

mod load;

use fbxcel::tree::v7400::Tree;

pub use self::load::{LoadError, Loader};

/// FBX document.
// This is intended to be a read-only structure as of writing this.
#[derive(Debug, Clone)]
pub struct Document {
    /// Low level tree.
    tree: Tree,
}

impl Document {
    /// Creates a new loader.
    #[inline]
    #[must_use]
    pub fn loader() -> Loader {
        Loader::new()
    }
}
