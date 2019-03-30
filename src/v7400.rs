//! FBX DOM utils for FBX v7.4 or later.
//!
//! # Concepts
//!
//! ## Tree
//!
//! FBX data has tree structure at low level.
//! This low-level structure is represented by `fbxcel::tree::v7400::Tree`.
//!
//! Some nodes in FBX (including geometries, bones, textures, etc) has directed
//! acyclic graph structures.
//! These are called "objects".
//!
//! [`Document`] type interprets low-level tree and provides access to
//! high-level objects graph structure.
//!
//! ### Node ID
//!
//! Each node (in low-level tree structure) is assigned internal indentifier by
//! `fbxcel` crate.
//! This does not appear in source FBX data.
//!
//! The node ID for low-level nodes are represented by
//! `fbxcel::tree::v7400::NodeId`.
//!
//! `fbxcel_dom` provides specialized node ID types for some types of nodes,
//! including objects.
//! ([`object::ObjectNodeId`] is one example of them.)
//! They have dedicated node ID types, and they can be converted into low-level
//! node ID.
//!
//! ### Node handle
//!
//! Node IDs cannot be used without the tree the node belongs to.
//! Node handle is a struct which contains both a tree and a node ID.
//!
//! The low-level node handles are represented by
//! `fbxcel::tree::v7400::NodeHandle`.
//!
//! `fbxcel_dom` provides specialized node handle types for some types of nodes,
//! including objects.
//! [`object::ObjectHandle`] is one example of them.
//!
//! ## Object
//!
//! Many useful data are represented as objects in FBX data.
//!
//! Some subsets of object types make tree structures, but the rules are not
//! completely revealed, because FBX is proprietary format.
//!
//! Sometimes users should see multiple connected objects to render 3D content.
//!
//! For detail, see [module documentation of `object`](object/index.html).
//!
//! [`Document`]: struct.Document.html
//! [`object::ObjectHandle`]: object/struct.ObjectHandle.html
//! [`object::ObjectNodeId`]: object/struct.ObjectNodeId.html

pub use self::{
    document::{Document, Loader},
    error::LoadError,
};

pub(crate) mod connection;
pub mod data;
mod definition;
mod document;
pub(crate) mod error;
pub mod object;
