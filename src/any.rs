//! Types and functions for all supported FBX versions.
//!
//! # How to load
//!
//! ## Easy setup (recommended)
//!
//! 1. Open the source reader.
//! 2. Pass the reader to [`AnyDocument::from_reader`] method or
//!    [`AnyDocument::from_seekable_reader`] method.
//!
//! ```rust,no_run
//! use fbxcel_dom::any::AnyDocument;
//!
//! let file = std::fs::File::open("sample.fbx")?;
//! // You can use the raw `file`, but buffering is recommended for more efficiency.
//! let reader = std::io::BufReader::new(file);
//!
//! // Use `from_seekable_reader` for readers implementing `std::io::Seek`.
//! // To use readers without `std::io::Seek` implementation, use `from_reader` method instead.
//! match AnyDocument::from_seekable_reader(reader)? {
//!     AnyDocument::V7400(fbx_version, doc) => {
//!         // FBX 7.4 or compatible.
//!         // Now you got a document `doc`.
//!         println!("FBX version {:?}", fbx_version);
//!         let _ = doc;
//!     }
//!     doc => {
//!         // FBX version is known to `fbxcel-dom` crate, but unknown to the user.
//!         panic!("Unknown FBX version {:?}", doc.fbx_version());
//!     }
//! }
//! # Ok::<(), anyhow::Error>(())
//! ```
//!
//! ## Manual setup
//!
//! 1. Open the source reader.
//! 2. Create a parser from the source reader.
//! 3. Pass it to a loader for the correct FBX version.
//!     + See [`v7400`][`crate::v7400`] module document for detail.

use std::io::{Read, Seek};

use fbxcel::low::FbxVersion;
use fbxcel::tree::any::AnyTree;
use thiserror::Error as ThisError;

/// FBX document load error.
#[derive(Debug, ThisError)]
pub enum LoadError {
    /// Unsupported (or unknown) FBX version.
    #[error("unsupported FBX version {}.{}", .0.major(), .0.minor())]
    UnsupportedVersion(FbxVersion),
    /// Syntax error.
    ///
    /// This error indicates that the lowlevel tree data has syntactic errors.
    #[error("invalid FBX syntax: {0}")]
    InvalidSyntax(#[from] fbxcel::tree::any::Error),
    /// Semantic error.
    ///
    /// This error indicates that the lowlevel tree data is OK but cannot interpret
    /// the data as a valid FBX structure.
    #[error("invalid FBX semantics: {0}")]
    InvalidSemantics(SemanticError),
}

/// Semantic error while loading FBX of a specific version.
#[derive(Debug, ThisError)]
#[non_exhaustive]
#[error(transparent)]
pub enum SemanticError {
    /// FBX 7.4 or later.
    V7400(crate::v7400::document::LoadError),
}

/// FBX document of any supported version.
#[derive(Debug)]
#[non_exhaustive]
// Box the backend document types to make this type small.
pub enum AnyDocument {
    /// FBX 7.4 or compatible.
    V7400(FbxVersion, Box<crate::v7400::Document>),
}

impl AnyDocument {
    /// Loads a document from the given reader.
    ///
    /// Though this works for seekable readers (which implement [`std::io::Seek`]),
    /// [`from_seekable_reader`][`Self::from_seekable_reader`] method should be
    /// used for them, since it is more efficent.
    #[inline]
    pub fn from_reader<R>(reader: R) -> Result<Self, LoadError>
    where
        R: Read,
    {
        Self::from_tree(AnyTree::from_reader(reader)?)
    }

    /// Loads a document form the given seekable reader.
    ///
    /// For non-seekable readers, use [`from_reader`][`Self::from_reader`] method.
    #[inline]
    pub fn from_seekable_reader<R>(reader: R) -> Result<Self, LoadError>
    where
        R: Read + Seek,
    {
        Self::from_tree(AnyTree::from_seekable_reader(reader)?)
    }

    /// Loads a document from the given lowlevel tree.
    fn from_tree(tree: fbxcel::tree::any::AnyTree) -> Result<Self, LoadError> {
        match tree {
            AnyTree::V7400(fbx_version, tree, _footer) => {
                let doc = crate::v7400::Document::loader()
                    .load_from_tree(tree)
                    .map_err(|e| LoadError::InvalidSemantics(SemanticError::V7400(e)))?;
                Ok(Self::V7400(fbx_version, Box::new(doc)))
            }
            tree => Err(LoadError::UnsupportedVersion(tree.fbx_version())),
        }
    }

    /// Returns the FBX version of the loaded document.
    pub fn fbx_version(&self) -> FbxVersion {
        match self {
            Self::V7400(ver, _doc) => *ver,
        }
    }
}
