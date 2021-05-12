//! Object properties.

use crate::v7400::properties::{PropertiesHandle, PropertiesNodeId};
use crate::v7400::{Document, PropertyHandle};

/// Object properties.
#[derive(Debug, Clone, Copy)]
pub struct ObjectProperties<'a> {
    /// Node ID of the direct properties.
    direct_props: Option<PropertiesNodeId>,
    /// Node ID of the default properties.
    default_props: Option<PropertiesNodeId>,
    /// Document.
    doc: &'a Document,
}

impl<'a> ObjectProperties<'a> {
    /// Creates a new object properties.
    #[inline]
    #[must_use]
    pub(super) fn new(
        direct_props: Option<PropertiesNodeId>,
        default_props: Option<PropertiesNodeId>,
        doc: &'a Document,
    ) -> Self {
        Self {
            direct_props,
            default_props,
            doc,
        }
    }

    /// Returns the property.
    ///
    /// First looks up the direct property. If not found, then falls back to the
    /// default property.
    pub fn get(&self, name: &str) -> Option<PropertyHandle<'a>> {
        self.get_direct(name).or_else(|| self.get_default(name))
    }

    /// Returns the direct property.
    pub fn get_direct(&self, name: &str) -> Option<PropertyHandle<'a>> {
        self.direct_props
            .map(|id| PropertiesHandle::new(id, self.doc))
            .and_then(|props| props.get(name))
    }

    /// Returns the default property.
    pub fn get_default(&self, name: &str) -> Option<PropertyHandle<'a>> {
        self.default_props
            .map(|id| PropertiesHandle::new(id, self.doc))
            .and_then(|props| props.get(name))
    }
}
