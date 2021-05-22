//! Document-wide settings.

use crate::v7400::properties::PropertiesNodeId;
use crate::v7400::{Document, ObjectProperties, Result};

/// A proxy to document-wide settings.
#[derive(Debug, Clone, Copy)]
pub struct GlobalSettings<'a> {
    /// Objects properties of `/GlobalSettings` node.
    props: ObjectProperties<'a>,
}

impl<'a> GlobalSettings<'a> {
    /// Creates a new proxy to `GlobalSettings` props.
    pub(super) fn new(doc: &'a Document) -> Result<Self> {
        let global_settings_node = doc
            .tree()
            .root()
            .first_child_by_name("GlobalSettings")
            .ok_or_else(|| error!("expected `/GlobalSettings` node but not found"))?;
        let direct_props = global_settings_node
            .first_child_by_name("Properties70")
            .map(|node| PropertiesNodeId::new(node.node_id()));

        // I am not confident about native typename being `FbxGlobalSettings`,
        // but it seems likely.
        // Documents I investigated has no default properties (`PropertyTemplate`) for any FB
        let default_props = doc
            .definitions_cache()
            .props_node_id("GlobalSettings", "FbxGlobalSettings");
        Ok(Self {
            props: ObjectProperties::new(direct_props, default_props, doc),
        })
    }
}
