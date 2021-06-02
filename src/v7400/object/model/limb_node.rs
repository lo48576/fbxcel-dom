//! Objects with `Model` class and `LimbNode` subclass.

use crate::v7400::connection::ConnectionsForObject;
use crate::v7400::object::model::{ChildSkeletonNodes, ModelHandle, SkeletonHierarchyNode};
use crate::v7400::object::subdeformer::SubDeformerClusterHandle;
use crate::v7400::object::{ObjectHandle, ObjectId, ObjectNodeId, ObjectSubtypeHandle};
use crate::v7400::Result;

/// Node ID for a model object with subclass `LimbNode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelLimbNodeNodeId(ObjectNodeId);

/// Object handle for a model object with subclass `LimbNode`.
#[derive(Debug, Clone, Copy)]
pub struct ModelLimbNodeHandle<'a> {
    /// Model handle.
    object: ModelHandle<'a>,
}

impl<'a> ModelLimbNodeHandle<'a> {
    /// Creates a model (limb node) handle from the given model handle.
    pub(super) fn from_model(object: &ModelHandle<'a>) -> Result<Self> {
        let subclass = object.as_object().subclass();
        if subclass != "LimbNode" {
            return Err(error!(
                "not a `Model(LimbNode)` object: expected \"LimbNode\" subclass \
                but got {:?} subclass",
                subclass
            ));
        }

        Ok(Self { object: *object })
    }

    /// Returns the object ID.
    #[inline]
    #[must_use]
    pub fn object_id(&self) -> ObjectId {
        self.as_object().id()
    }
}

impl<'a> ModelLimbNodeHandle<'a> {
    /// Returns the parent model node.
    ///
    /// If there are two or more parent models, one of them is returned.
    /// If you want to get all of them, use [`ObjectHandle::destination_objects`]
    /// and filter by yourself.
    #[must_use]
    pub fn parent_skeleton_node(&self) -> Option<SkeletonHierarchyNode<'a>> {
        self.as_object()
            .destination_objects()
            .filter(|conn| !conn.has_label())
            .filter_map(|conn| conn.destination())
            .find_map(|obj| SkeletonHierarchyNode::from_object(&obj).ok())
    }

    /// Returns the parent clusters.
    // Memo: a limb node can have multiple parent clusters.
    // For example in the wild, see `Model::Head_bind`
    // (subclass=`LimbNode`, ID=943273919888) of `naka.fbx` in
    // <https://web.archive.org/web/20180902221702/http://nakasis.com/data/NakanoSisters_1_2_FBX.zip>.
    // It has three parent cluster objects (IDs are 943347533968, 943135892928,
    // and 943135894432).
    #[inline]
    #[must_use]
    pub fn parent_clusters(&self) -> ParentClusters<'a> {
        ParentClusters {
            destinations: self.as_object().destination_objects(),
        }
    }

    /// Returns an iterator of the child limb nodes.
    #[inline]
    #[must_use]
    pub fn child_skeleton_nodes(&self) -> ChildSkeletonNodes<'a> {
        ChildSkeletonNodes::from_parent(self.as_object())
    }
}

impl<'a> ObjectSubtypeHandle<'a> for ModelLimbNodeHandle<'a> {
    type NodeId = ModelLimbNodeNodeId;

    #[inline]
    fn from_object(object: &ObjectHandle<'a>) -> Result<Self> {
        ModelHandle::from_object(object).and_then(|model| Self::from_model(&model))
    }

    #[inline]
    fn as_object(&self) -> &ObjectHandle<'a> {
        &self.object.as_object()
    }

    #[inline]
    fn node_id(&self) -> Self::NodeId {
        ModelLimbNodeNodeId(self.as_object().node_id())
    }
}

/// Iterator of `SubDeformer`(`Cluster`) nodes which are children of a `Model`(`LimbNode`) node.
#[derive(Debug, Clone)]
pub struct ParentClusters<'a> {
    /// Destination objects.
    destinations: ConnectionsForObject<'a>,
}

impl<'a> Iterator for ParentClusters<'a> {
    type Item = SubDeformerClusterHandle<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.destinations
            .by_ref()
            .filter(|conn| !conn.has_label())
            .filter_map(|conn| conn.destination())
            .find_map(|obj| SubDeformerClusterHandle::from_object(&obj).ok())
    }
}
