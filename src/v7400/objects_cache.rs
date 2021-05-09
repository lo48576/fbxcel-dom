//! Objects cache.

use std::collections::HashMap;
use std::sync::Arc;

use fbxcel::low::v7400::AttributeValue;
use fbxcel::tree::v7400::{NodeHandle, Tree};
use lasso::{MiniSpur, Rodeo, RodeoReader};

use crate::v7400::document::LoadError;
use crate::v7400::object::{ObjectId, ObjectNodeId};

/// A symbol of an interned string.
// This may be exported in future to enable users to compare classes and subclasses
// in faster way, but note that resolving user-provided symbol will be unsupported.
// Symbols not tied to a document will become the source of bugs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct ObjectClassSym(MiniSpur);

/// Cached metadata of an object.
#[derive(Debug, Clone)]
pub(super) struct ObjectMeta {
    /// Object ID.
    id: ObjectId,
    /// Name (if exists).
    ///
    /// Note that `Some("")` and `None` is distinguished.
    name: Option<String>,
    /// Class.
    class: ObjectClassSym,
    /// Subclass.
    subclass: ObjectClassSym,
}

impl ObjectMeta {
    /// Creates a new object meta.
    #[inline]
    #[must_use]
    fn new(
        id: ObjectId,
        name: Option<String>,
        class: ObjectClassSym,
        subclass: ObjectClassSym,
    ) -> Self {
        Self {
            id,
            name,
            class,
            subclass,
        }
    }

    /// Returns the object ID.
    #[inline]
    #[must_use]
    pub(super) fn id(&self) -> ObjectId {
        self.id
    }

    /// Returns the object name.
    ///
    /// Note that `Some("")` and `None` is distinguished.
    #[inline]
    #[must_use]
    pub(super) fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the object class.
    #[inline]
    #[must_use]
    pub(super) fn class<'a>(&self, objects_cache: &'a ObjectsCache) -> &'a str {
        objects_cache.resolve_class_string(self.class)
    }

    /// Returns the object class.
    #[inline]
    #[must_use]
    pub(super) fn subclass<'a>(&self, objects_cache: &'a ObjectsCache) -> &'a str {
        objects_cache.resolve_class_string(self.subclass)
    }
}

/// Objects cache.
#[derive(Debug, Clone)]
pub(super) struct ObjectsCache {
    /// A map from object ID to node ID.
    obj_id_to_node_id: HashMap<ObjectId, ObjectNodeId>,
    /// Object metadata store.
    // Using `ObjectNodeId` as a key since `ObjectMeta` contains an object ID.
    meta: HashMap<ObjectNodeId, ObjectMeta>,
    /// Interned object classes and subclasses.
    class_strings: Arc<RodeoReader<MiniSpur>>,
}

impl ObjectsCache {
    /// Creates an objects cache from the given tree.
    #[inline]
    pub(super) fn from_tree(tree: &Tree) -> Result<Self, LoadError> {
        ObjectsCacheBuilder::default().load(tree)
    }

    /// Returns the object node ID for the node with the given node ID.
    #[must_use]
    pub(super) fn node_id(&self, obj_id: ObjectId) -> Option<ObjectNodeId> {
        self.obj_id_to_node_id.get(&obj_id).cloned()
    }

    /// Returns a reference to the object metadata.
    #[must_use]
    pub(super) fn meta_from_node_id(&self, node_id: ObjectNodeId) -> Option<&ObjectMeta> {
        self.meta.get(&node_id)
    }

    /// Resolves object class and subclass to string.
    ///
    /// # Panics
    ///
    /// Panics if the given symbol is not registered in the internal table.
    /// This may happen when the given symbol is generated for another `ObjectsCache`,
    /// and it is completely the fault of this crate if it happens.
    #[must_use]
    pub(super) fn resolve_class_string(&self, sym: ObjectClassSym) -> &str {
        self.class_strings.try_resolve(&sym.0).unwrap_or_else(|| {
            panic!("bug: the given object class symbol is not a valid key of the string table")
        })
    }
}

/// Objcets cache builder.
#[derive(Debug)]
struct ObjectsCacheBuilder {
    /// A map from object ID to node ID.
    obj_id_to_node_id: HashMap<ObjectId, ObjectNodeId>,
    /// Object metadata store.
    meta: HashMap<ObjectNodeId, ObjectMeta>,
    /// Interned object classes and subclasses.
    class_strings: Rodeo<MiniSpur>,
}

// Workaround for lasso-0.5.0. See <https://github.com/Kixiron/lasso/issues/26>.
impl Default for ObjectsCacheBuilder {
    fn default() -> Self {
        Self {
            obj_id_to_node_id: Default::default(),
            meta: Default::default(),
            class_strings: Rodeo::new(),
        }
    }
}

impl ObjectsCacheBuilder {
    /// Creates an objects cache from the given tree.
    fn load(mut self, tree: &Tree) -> Result<ObjectsCache, LoadError> {
        self.load_objects(tree)?;

        Ok(self.build())
    }

    /// Builds the objects cache.
    fn build(self) -> ObjectsCache {
        ObjectsCache {
            obj_id_to_node_id: self.obj_id_to_node_id,
            meta: self.meta,
            class_strings: Arc::new(self.class_strings.into_reader()),
        }
    }

    /// Loads objects.
    fn load_objects(&mut self, tree: &Tree) -> Result<(), LoadError> {
        let objects_node = tree.root().first_child_by_name("Objects").ok_or_else(|| {
            LoadError::from_msg("expected toplevel `Objects` node to exist but not found")
        })?;

        for obj_node in objects_node.children() {
            self.load_object(obj_node)?;
        }

        Ok(())
    }

    /// Loads an object.
    fn load_object(&mut self, node: NodeHandle<'_>) -> Result<(), LoadError> {
        assert!(
            !self.meta.contains_key(&ObjectNodeId::new(node.node_id())),
            "should never fail: the same object node (node_id={:?}), should not loaded twice",
            node.node_id()
        );

        let (obj_id, name_class, subclass): (i64, &str, &str) = match node.attributes() {
            [AttributeValue::I64(obj_id), AttributeValue::String(name_class), AttributeValue::String(subclass)] => {
                (*obj_id, name_class, subclass)
            }
            [a0, a1, a2] => {
                return Err(LoadError::from_msg(format!(
                    "invalid node attributes: expected `(i64, String, String)` attributes, \
                    but got `({:?}, {:?}, {:?})`",
                    a0.type_(),
                    a1.type_(),
                    a2.type_()
                )))
            }
            _ => {
                return Err(LoadError::from_msg(format!(
                    "invalid object node attributes: expected three attributes but got {}",
                    node.attributes().len()
                )))
            }
        };
        let obj_id = ObjectId::new(obj_id);
        // NOTE: FBX ASCII format is not supported.
        // To support ASCII format, document loader should be able to know the
        // source format of the document (i.e. binary or ASCII).
        // For now, fbxcel-0.7.0 (or `develop` branch at 2021-05-09) does not
        // support FBX ASCII format loading, so it is safe to assume that the
        // source document is FBX binary.
        let (name, class) = decompose_name_class_bin(name_class);
        let meta = ObjectMeta::new(
            obj_id,
            name.map(ToOwned::to_owned),
            ObjectClassSym(self.class_strings.get_or_intern(class)),
            ObjectClassSym(self.class_strings.get_or_intern(subclass)),
        );
        let node_id = ObjectNodeId::new(node.node_id());

        self.obj_id_to_node_id.insert(obj_id, node_id);
        self.meta.insert(node_id, meta);

        Ok(())
    }
}

/// Decomposes the object name and class.
///
/// In FBX binary format, the object name and the class is placed together at
/// the second attribute (`attrs[1]`) of the object node, in the
/// `name\x00\x01class` or `class` format.
/// This method decomposes the object into name and class.
#[must_use]
fn decompose_name_class_bin(name_class: &str) -> (Option<&str>, &str) {
    // NOTE: This (`name\x00\x01class` format) is only for FBX binary format.
    name_class.find("\u{0}\u{1}").map_or((None, ""), |sep_pos| {
        (Some(&name_class[0..sep_pos]), &name_class[(sep_pos + 2)..])
    })
}
