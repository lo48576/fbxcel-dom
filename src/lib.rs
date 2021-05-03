//! FBX DOM library.
//!
//! To see how to load an FBX document, see the module documentation for [`any`].
#![forbid(unsafe_code)]
#![warn(rust_2018_idioms)]
// `clippy::missing_docs_in_private_items` implies `missing_docs`.
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::unwrap_used)]

pub mod any;
pub mod v7400;

pub use fbxcel;
