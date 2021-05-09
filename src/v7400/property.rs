//! Functions and types for property (`P` under `Properties70`) node.

mod load;
pub mod loaders;

use fbxcel::low::v7400::AttributeValue;
use fbxcel::tree::v7400::{NodeHandle, NodeId};

use crate::v7400::{Document, Result};

pub use self::load::LoadPropertyNodeValue;

/// Node ID of a property node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PropertyNodeId(NodeId);

impl PropertyNodeId {
    /// Creates a new `PropertyNodeId`.
    #[inline]
    #[must_use]
    pub(super) fn new(node_id: NodeId) -> Self {
        Self(node_id)
    }

    /// Returns the internal node ID.
    #[inline]
    #[must_use]
    pub(super) fn tree_node_id(self) -> NodeId {
        self.0
    }
}

/// Node handle of a property node.
#[derive(Debug, Clone)]
pub struct PropertyNodeHandle<'a> {
    /// Node ID.
    node_id: PropertyNodeId,
    /// Document.
    doc: &'a Document,
}

impl<'a> PropertyNodeHandle<'a> {
    /// Creates a new node handle for a property node.
    #[inline]
    #[must_use]
    pub(super) fn new(node_id: PropertyNodeId, doc: &'a Document) -> Self {
        debug_assert_eq!(
            node_id.tree_node_id().to_handle(doc.tree()).name(),
            "P",
            "expected property node `P` but got different node",
        );
        Self { node_id, doc }
    }

    /// Returns a lowlevel node handle for the properties node.
    #[inline]
    #[must_use]
    pub(super) fn tree_node(&self) -> NodeHandle<'a> {
        self.node_id.tree_node_id().to_handle(self.doc.tree())
    }

    /// Returns a lowlevel node ID for the properties node.
    #[inline]
    #[must_use]
    fn tree_node_id(&self) -> NodeId {
        self.node_id.tree_node_id()
    }

    /// Returns the property name.
    #[inline]
    pub fn name(&self) -> Result<&'a str> {
        self.get_string_attr(0, "property name")
    }

    /// Returns the property type name.
    #[inline]
    pub fn typename(&self) -> Result<&'a str> {
        self.get_string_attr(1, "property typename")
    }

    /// Returns proprety label.
    #[inline]
    pub fn label(&self) -> Result<&'a str> {
        self.get_string_attr(2, "property label")
    }

    /// Loads the property value using the given loader.
    #[inline]
    pub fn value<L>(&self, loader: L) -> std::result::Result<L::Value, L::Error>
    where
        L: LoadPropertyNodeValue<'a>,
    {
        loader.load(self)
    }

    /// Returns property value part of node attributes.
    pub fn value_raw(&self) -> Result<&'a [AttributeValue]> {
        self.tree_node().attributes().get(4..).ok_or_else(|| {
            error!(
                "not enough node attributes for the property node: \
                expected 4 or more, but found {} (tree_node_id={:?})",
                self.tree_node().attributes().len(),
                self.tree_node_id()
            )
        })
    }

    /// Returns the property name.
    fn get_string_attr(&self, index: usize, target: &str) -> Result<&'a str> {
        self.tree_node()
            .attributes()
            .get(index)
            .ok_or_else(|| {
                error!(
                    "{} is not found (tree_node_id={:?})",
                    target,
                    self.tree_node_id()
                )
            })?
            .get_string_or_type()
            .map_err(|ty| {
                error!(
                    "invalid {} type: expected string but got {:?} (tree_node_id={:?})",
                    target,
                    ty,
                    self.tree_node_id()
                )
            })
    }
}
