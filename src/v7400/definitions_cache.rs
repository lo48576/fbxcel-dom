//! Object properties template definitions cache.

use std::collections::HashMap;

use fbxcel::low::v7400::AttributeValue;
use fbxcel::tree::v7400::{NodeHandle, Tree};

use crate::v7400::document::LoadError;
use crate::v7400::properties::PropertiesNodeId;

/// Object properties template definitions cache.
#[derive(Default, Debug, Clone)]
pub(super) struct DefinitionsCache {
    /// Templates.
    ///
    /// The outer key is an object node name.
    /// The inner key is (maybe) the native node type name in FBX SDK.
    templates: HashMap<String, HashMap<String, PropertiesNodeId>>,
}

impl DefinitionsCache {
    /// Creates definitions from the given tree.
    pub(super) fn from_tree(tree: &Tree) -> Self {
        let mut builder = DefinitionsCacheLoader::default();
        builder.load_tree(tree);
        builder.build()
    }

    /// Returns the properties node ID of the template if available.
    #[must_use]
    pub(super) fn props_node_id(
        &self,
        node_name: &str,
        native_typename: &str,
    ) -> Option<PropertiesNodeId> {
        self.templates.get(node_name)?.get(native_typename).copied()
    }
}

/// `DefinitionsCache` loader.
#[derive(Default, Debug, Clone)]
pub(super) struct DefinitionsCacheLoader {
    /// Templates.
    ///
    /// See the documentation of [`DefinitionsCache`] for detail.
    templates: HashMap<String, HashMap<String, PropertiesNodeId>>,
}

impl DefinitionsCacheLoader {
    /// Builds a `DefinitionsCache`.
    #[inline]
    #[must_use]
    fn build(self) -> DefinitionsCache {
        DefinitionsCache {
            templates: self.templates,
        }
    }

    /// Loads definitions from the given tree.
    fn load_tree(&mut self, tree: &Tree) {
        let defs_node = match tree.root().first_child_by_name("Definitions") {
            Some(v) => v,
            None => return,
        };

        for obj_type_node in defs_node.children_by_name("ObjectType") {
            if let Err(e) = self.load_object_type(obj_type_node) {
                log::warn!(
                    "ignoring noncritical error: failed to load `ObjectType` \
                    node (node_id={:?}): {}",
                    obj_type_node.node_id(),
                    e
                );
            }
        }
    }

    /// Loads a `/Definitions/ObjectType` node.
    fn load_object_type(&mut self, obj_type_node: NodeHandle<'_>) -> Result<(), LoadError> {
        let obj_type = match obj_type_node.attributes().get(0) {
            Some(AttributeValue::String(v)) => v,
            Some(v) => {
                return Err(LoadError::from_msg(format!(
                    "expected a string attribute as object type, but got {:?}",
                    v.type_()
                )))
            }
            None => {
                return Err(LoadError::from_msg(
                    "expected a string attribute as object type, but no attributes found",
                ))
            }
        };

        for template_node in obj_type_node.children_by_name("PropertyTemplate") {
            if let Err(e) = self.load_property_template(template_node, obj_type) {
                log::warn!(
                    "ignoring noncritical error: failed to load properties \
                    template under an `ObjectType` node (node_id={:?}): {}",
                    obj_type_node.node_id(),
                    e
                );
            }
        }

        Ok(())
    }

    /// Loads a `/Definitions/ObjectType/PropertyTemplate` node.
    fn load_property_template(
        &mut self,
        template_node: NodeHandle<'_>,
        obj_type: &str,
    ) -> Result<(), LoadError> {
        let native_typename = match template_node.attributes().get(0) {
            Some(AttributeValue::String(v)) => v,
            Some(v) => {
                return Err(LoadError::from_msg(format!(
                    "expected a string attribute as a native typename, but got {:?}",
                    v.type_()
                )))
            }
            None => {
                return Err(LoadError::from_msg(
                    "expected a string attribute as a native typename, but no attributes found",
                ))
            }
        };
        let props_node_id = match template_node.first_child_by_name("Properties70") {
            Some(node) => PropertiesNodeId::new(node.node_id()),
            None => return Ok(()),
        };

        self.templates
            .entry(obj_type.to_owned())
            .or_insert_with(Default::default)
            .insert(native_typename.to_owned(), props_node_id);

        Ok(())
    }
}
