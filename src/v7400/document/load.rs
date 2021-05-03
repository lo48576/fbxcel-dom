//! Document loader.

use fbxcel::pull_parser::v7400::Parser;
use fbxcel::pull_parser::ParserSource;
use fbxcel::tree::v7400::{Loader as TreeLoader, Tree};
use thiserror::Error as ThisError;

use crate::v7400::Document;

/// Document load error.
#[derive(Debug, ThisError)]
#[error("{msg}: {source}")]
pub struct LoadError {
    /// Message.
    msg: String,
    /// Source error.
    source: anyhow::Error,
}

impl LoadError {
    /// Creates a new error.
    #[inline]
    #[must_use]
    fn new(msg: impl Into<String>, source: impl Into<anyhow::Error>) -> Self {
        Self {
            msg: msg.into(),
            source: source.into(),
        }
    }
}

/// FBX document loader.
#[derive(Default, Debug, Clone)]
pub struct Loader(());

impl Loader {
    /// Creates a new loader.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads a document from the given FBX parser.
    pub fn load_from_parser<R: ParserSource>(
        self,
        parser: &mut Parser<R>,
    ) -> Result<Document, LoadError> {
        log::trace!("Loading FBX document from a parser");
        let (tree, _footer) = TreeLoader::new()
            .load(parser)
            .map_err(|e| LoadError::new("failed to load lowlevel document tree", e))?;
        self.load_from_tree(tree)
    }

    /// Loads a document from the given lowlevel FBX tree.
    pub fn load_from_tree(self, tree: Tree) -> Result<Document, LoadError> {
        log::trace!("Loading FBX document from a lowlevel tree");
        log::trace!("Successfully loaded FBX document from a lowlevel tree");

        Ok(Document { tree })
    }
}
