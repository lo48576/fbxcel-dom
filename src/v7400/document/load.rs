//! Document loader.

use std::fmt;

use fbxcel::pull_parser::v7400::Parser;
use fbxcel::pull_parser::ParserSource;
use fbxcel::tree::v7400::{Loader as TreeLoader, Tree};
use thiserror::Error as ThisError;

use crate::v7400::document::{DefinitionsCache, ObjectsCache};
use crate::v7400::Document;

/// Document load error.
#[derive(Debug, ThisError)]
pub struct LoadError {
    /// Message.
    msg: String,
    /// Source error.
    source: Option<anyhow::Error>,
}

impl LoadError {
    /// Creates a new error.
    #[inline]
    #[must_use]
    pub(super) fn new(msg: impl Into<String>, source: impl Into<anyhow::Error>) -> Self {
        Self {
            msg: msg.into(),
            source: Some(source.into()),
        }
    }

    /// Creates a new error from a message.
    #[inline]
    #[must_use]
    // `pub(in crate::v7400)` since this is used in `v7400::objects_cache` and
    // `v7400::definitions_cache`.
    pub(in crate::v7400) fn from_msg(msg: impl Into<String>) -> Self {
        Self {
            msg: msg.into(),
            source: None,
        }
    }
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.msg)?;
        if let Some(source) = &self.source {
            f.write_str(": ")?;
            source.fmt(f)?;
        }
        Ok(())
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

        let objects_cache = ObjectsCache::from_tree(&tree)?;
        let definitions_cache = DefinitionsCache::from_tree(&tree);

        Ok(Document {
            tree,
            objects_cache,
            definitions_cache,
        })
    }
}
