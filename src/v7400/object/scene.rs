//! Dummy root object.

use fbxcel::low::v7400::AttributeValue as A;
use fbxcel::tree::v7400::NodeHandle;

use crate::v7400::connection::ConnectionsForObjectByLabel;
use crate::v7400::{Document, ObjectHandle, ObjectId, ObjectNodeId, Result};

/// Node handle of the root object node.
///
/// This dummy root object has no corresponding object node. Only the object ID
/// appears in the objects graph.
///
/// # `Document` node
///
/// Internally, this is `/Documents/Document` node, and it has almost the same
/// structure as other object nodes.
/// It has object ID as the first attribute, and strings as second and third attributes.
/// (Third attribute is the string `Scene`, as if it has `Scene` subclass.)
///
/// However, it has some difference from usual objects.
/// Object nodes are usually under `/Objects`, but `Document` nodes are not.
/// Object nodes have classes, but `Document` nodes do not.
/// Object nodes are usually connected to other objects, but `Document` nodes are not.
///
/// # Root object
///
/// `Document` node has a data of the root object of the scene.
/// Such root objects may only have an object ID, and may not have any
/// corresponding lowlevel nodes.
///
/// ```text
/// Relevant part of the raw FBX tree structure:
///
/// +-- Documents
/// |   `-- Document  // A virtual node corresponding to a scene.
/// |       `-- RootNode
/// |           `-- (attr[0])  // This has root object ID of the scene.
/// |-- Objects
/// |   |-- (objects...)
/// |   |-- (Object)  // Some objects may be sources (children) of the root object.
/// |   |-- (Object)  // Note that the root object may have multiple sources (children).
/// |   `-- (other objects...)
/// `-- Connections
///     |-- (connections...)
///     |-- C  // A connection between two objects.
///     |      // Connected objects may have no corresponding object nodes, and
///     |      // the root object may be one of such "virtual" object nodes.
///     `-- (other connections...)
/// ```
#[derive(Debug, Clone, Copy)]
pub struct SceneHandle<'a> {
    /// Object ID of the dummy scene object (`Document` node).
    id: ObjectId,
    /// Node ID of the dummy scene object (`Document` node).
    node_id: ObjectNodeId,
    /// Document.
    doc: &'a Document,
}

impl<'a> SceneHandle<'a> {
    /// Returns the object ID for the (virtual) scene object.
    #[inline]
    #[must_use]
    pub fn scene_object_id(&self) -> ObjectId {
        self.id
    }

    /// Returns the object handle for the (virtual) scene object.
    #[inline]
    #[must_use]
    fn object_handle(&self) -> ObjectHandle<'a> {
        self.node_id.to_handle(self.doc).expect(
            "should never fail: `Document` nodes cached at \
            `ObjectsCache::document_nodes` must be also registered as an object",
        )
    }

    /// Returns the lowlevel node handle for the `Document` node.
    #[inline]
    #[must_use]
    fn tree_node(&self) -> NodeHandle<'a> {
        self.object_handle().tree_node()
    }

    /// Returns the object ID of the (virtual) root object of the scene.
    pub fn root_object_id(&self) -> Result<ObjectId> {
        let root_info_node = self
            .tree_node()
            .first_child_by_name("RootNode")
            .ok_or_else(|| error!("expected `RootNode` node under `Document` but not found"))?;
        match root_info_node.attributes() {
            [A::I64(id)] => Ok(ObjectId::new(*id)),
            [v] => Err(error!("expected `i64` attribute but got {:?}", v.type_())),
            v => Err(error!(
                "expected single `i64` attribute but got {} attributes",
                v.len()
            )),
        }
    }

    /// Returns an iterator of the children of the scene root.
    #[inline]
    pub fn children(&self) -> Result<SceneRootChildren<'a>> {
        let root_object_id = self.root_object_id()?;
        Ok(SceneRootChildren {
            iter: self.doc.source_objects_by_label(root_object_id, None),
            doc: self.doc,
        })
    }
}

/// Iterator of the scene proxies.
#[derive(Debug, Clone)]
pub struct SceneIter<'a> {
    /// Iterator of `/Documents/Document` nodes.
    iter: std::slice::Iter<'a, ObjectNodeId>,
    /// Document.
    doc: &'a Document,
}

impl<'a> SceneIter<'a> {
    /// Creates a new scenes iterator for the document.
    #[inline]
    #[must_use]
    pub(in crate::v7400) fn new(doc: &'a Document) -> Self {
        Self {
            iter: doc.objects_cache().document_node_ids(),
            doc,
        }
    }
}

impl<'a> Iterator for SceneIter<'a> {
    type Item = SceneHandle<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|&node_id| {
            let id = self
                .doc
                .objects_cache()
                .meta_from_node_id(node_id)
                .expect(
                    "should never fail: `Document` nodes cached at `ObjectsCache::document_nodes` \
                    must be also registered as an object",
                )
                .id();
            SceneHandle {
                id,
                node_id,
                doc: self.doc,
            }
        })
    }
}

impl std::iter::FusedIterator for SceneIter<'_> {}

/// An iterator of the children of a scene root.
#[derive(Debug, Clone)]
pub struct SceneRootChildren<'a> {
    /// Iterator of connections.
    iter: ConnectionsForObjectByLabel<'a>,
    /// Document.
    doc: &'a Document,
}

impl<'a> Iterator for SceneRootChildren<'a> {
    type Item = ObjectHandle<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let doc = self.doc;
        self.iter
            .find_map(|conn| doc.get_object_by_id(conn.source_id()))
    }
}

impl std::iter::FusedIterator for SceneRootChildren<'_> {}
