//! Functions and types for properties (`Properties70`) node.
//!
//! # Properties
//!
//! In FBX 7.4, object properties and some global properties are represented as
//! a key-value map. A whole map is `Properties70` node, and entries in it are
//! `P` nodes.
//!
//! The key is a string, and the value is a multiple fields: typename, label,
//! flags, and the typed value.
//!
//! ```text
//! ; Example of properties node.
//! Properties70:  {
//!     ; ...
//!     P: "DiffuseColor", "Color", "", "A", 0.8, 0.8, 0.8
//!     P: "DiffuseFactor", "Number", "", "A", 1
//!     P: "Bump", "Vector3D", "Vector", "", 0, 0, 0
//!     P: "DisplacementColor", "ColorRGB", "Color", "", 0, 0, 0
//!     ; ...
//! }
//! ```
//!
//! Note that the keys in a map are expected to be unique, but this crate should
//! be able to handle malicious or broken data.
//! If there are multiple keys with the same key string in a map, the first
//! entry with that key (in order of appearance in the document) will be chosen
//! by this crate.
//!
//! Also note that objects can have default properties. This means that users
//! who attempt to get a property value for some object will also want to access
//! default properties. For this access pattern, a dedicated type for object
//! properties will be provided (not yet implemented).
//!
//! ## `PropertiesNodeHandle`
//!
//! [`PropertiesNodeHandle`] type is a proxy with convenience functions to
//! `Properties70` node. This works as a simple map.
//!
//! ## `PropertyHandle`
//!
//! [`PropertyNodeHandle`] type is a proxy with convenience functions to `P` node.

use fbxcel::tree::v7400::{ChildrenByName, NodeHandle, NodeId};

use crate::v7400::{Document, PropertyNodeHandle, PropertyNodeId};

/// Node ID of a properties node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PropertiesNodeId(NodeId);

impl PropertiesNodeId {
    /// Creates a new `PropertiesNodeId`.
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
pub struct PropertiesNodeHandle<'a> {
    /// Node ID.
    node_id: PropertiesNodeId,
    /// Document.
    doc: &'a Document,
}

impl<'a> PropertiesNodeHandle<'a> {
    /// Creates a new node handle for a properties node.
    #[inline]
    #[must_use]
    pub(super) fn new(node_id: PropertiesNodeId, doc: &'a Document) -> Self {
        debug_assert_eq!(
            node_id.tree_node_id().to_handle(doc.tree()).name(),
            "Properties70",
            "expected properties node `Properties70` but got different node",
        );
        Self { node_id, doc }
    }

    /// Returns a lowlevel node handle for the properties node.
    #[inline]
    #[must_use]
    pub(super) fn tree_node(&self) -> NodeHandle<'a> {
        self.node_id.tree_node_id().to_handle(self.doc.tree())
    }

    /// Returns an iterator of the property nodes.
    #[must_use]
    pub fn iter(&self) -> Iter<'a> {
        Iter {
            p_node_iter: self.tree_node().children_by_name("P"),
            doc: self.doc,
        }
    }

    /// Returns the property handle with the given name.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<PropertyNodeHandle<'a>> {
        self.tree_node()
            .children_by_name("P")
            .map(|node| PropertyNodeId::new(node.node_id()))
            .map(|node_id| PropertyNodeHandle::new(node_id, self.doc))
            .filter_map(|prop| match prop.name() {
                Ok(name) => Some((prop, name)),
                Err(e) => {
                    log::warn!("ignoring a property with invalid name: {}", e);
                    None
                }
            })
            .find_map(|(prop, pname)| if pname == name { Some(prop) } else { None })
    }
}

impl<'a> IntoIterator for PropertiesNodeHandle<'a> {
    type IntoIter = Iter<'a>;
    type Item = PropertyNodeHandle<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'_ PropertiesNodeHandle<'a> {
    type IntoIter = Iter<'a>;
    type Item = PropertyNodeHandle<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Iterator of property nodes [`PropertyNodeHandle`] under a [`PropertiesNodeHandle`].
#[derive(Debug, Clone)]
pub struct Iter<'a> {
    /// An iterator of `P` nodes.
    p_node_iter: ChildrenByName<'a>,
    /// FBX document.
    doc: &'a Document,
}

impl<'a> Iterator for Iter<'a> {
    type Item = PropertyNodeHandle<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.p_node_iter
            .next()
            .map(|tree_node_handle| PropertyNodeId::new(tree_node_handle.node_id()))
            .map(|node_id| PropertyNodeHandle::new(node_id, self.doc))
    }
}
