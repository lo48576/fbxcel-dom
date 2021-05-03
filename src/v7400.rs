//! FBX DOM utils for FBX v7.4 or later.
//!
//! # How to load
//!
//! Usually [loading by `any::Document`][`crate::any`] is preferred, until you
//! want to customize the parser.
//!
//! 1. Open the source reader.
//! 2. Create a parser from the source reader.
//! 3. Pass the created parser to the lowlevel tree loader.
//! 4. Pass the loaded tree to the [`Document`] loader.
//!
//! ```rust,no_run
//! use fbxcel::pull_parser::any::AnyParser;
//! use fbxcel_dom::v7400::Document;
//!
//! let file = std::fs::File::open("sample.fbx")?;
//! // You can use the raw `file`, but buffering is recommended for more efficiency.
//! let reader = std::io::BufReader::new(file);
//!
//! // Create a parser.
//! let mut parser = match fbxcel::pull_parser::any::from_seekable_reader(reader)? {
//!     AnyParser::V7400(mut parser) => {
//!         // Now you can customize the parser here, for example you can call
//!         // `parser.set_warning_handler(/* ... */);`.
//!         parser
//!     },
//!     parser => {
//!         // This version of the FBX data is supported by the parser but not by this code.
//!         // In production code, you should return an error instead of panicking.
//!         panic!("unsupported FBX version {:?}", parser.fbx_version());
//!     }
//! };
//!
//! // Create a lowlevel tree loader.
//! let (tree, _footer) = fbxcel::tree::v7400::Loader::new().load(&mut parser)?;
//!
//! // Load a document from the lowlevel tree.
//! let doc = Document::loader().load_from_tree(tree)?;
//! // Now you got a document `doc`.
//! # Ok::<(), anyhow::Error>(())
//! ```

pub mod document;

pub use self::document::Document;
