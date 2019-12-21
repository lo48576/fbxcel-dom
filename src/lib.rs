//! FBX DOM library.
//!
//! # Creating DOM
//!
//! ## Easy setup (recommended)
//!
//! If you don't care about low-level features (such as precise FBX version and
//! parser warning handling), you can use easy setup using [`any`] module.
//!
//! ```no_run
//! use fbxcel_dom::any::AnyDocument;
//!
//! let file = std::fs::File::open("sample.fbx").expect("Failed to open file");
//! // You can also use raw `file`, but do buffering for better efficiency.
//! let reader = std::io::BufReader::new(file);
//!
//! // Use `from_seekable_reader` for readers implementing `std::io::Seek`.
//! // To use readers without `std::io::Seek` implementation, use `from_reader`
//! // instead.
//! match AnyDocument::from_seekable_reader(reader).expect("Failed to load document") {
//!     AnyDocument::V7400(fbx_ver, doc) => {
//!         // You got a document. You can do what you want.
//!     }
//!     // `AnyDocument` is nonexhaustive.
//!     // You should handle unknown document versions case.
//!     _ => panic!("Got FBX document of unsupported version"),
//! }
//! ```
//!
//! ## Manual setup
//!
//! You can create a parser or a tree by yourself, and use appropriate loader to
//! load the document from it.
//!
//! For example:
//!
//! * From `tree: fbxcel::tree::v7400::Tree`, you can create the document by
//!   `fbxcel_dom::v7400::Loader::load_from_tree(tree)`.
//! * From `parser: fbxcel::pull_parser::v7400::Parser`, you can create the
//!   document by `fbxcel_dom::v7400::Loader::load_from_parser(&mut parser)`.
//!
//! For detail, see documents of loaders.
//!
//! [`any`]: any/index.html
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

pub use fbxcel;

pub mod any;
pub mod v7400;
