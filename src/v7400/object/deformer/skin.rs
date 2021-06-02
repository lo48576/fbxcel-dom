//! Objects with `Deformer` class and `Skin` subclass.

use crate::v7400::connection::ConnectionsForObject;
use crate::v7400::object::deformer::DeformerHandle;
use crate::v7400::object::geometry::GeometryMeshHandle;
use crate::v7400::object::subdeformer::SubDeformerClusterHandle;
use crate::v7400::object::{ObjectHandle, ObjectId, ObjectNodeId, ObjectSubtypeHandle};
use crate::v7400::Result;

/// Node ID for a deformer object with subclass `Skin`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeformerSkinNodeId(ObjectNodeId);

/// Object handle for a deformer object with subclass `Skin`.
#[derive(Debug, Clone, Copy)]
pub struct DeformerSkinHandle<'a> {
    /// Deformer handle.
    object: DeformerHandle<'a>,
}

impl<'a> DeformerSkinHandle<'a> {
    /// Creates a deformer (skin) handle from the given deformer handle.
    fn from_deformer(object: &DeformerHandle<'a>) -> Result<Self> {
        let subclass = object.as_object().subclass();
        if subclass != "Skin" {
            return Err(error!(
                "not a `Deformer(Skin)` object: expected \"Skin\" subclass \
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

    /// Returns the reference to the more generic deformer handle.
    #[inline]
    #[must_use]
    pub fn as_deformer(&self) -> &DeformerHandle<'a> {
        &self.object
    }
}

impl<'a> DeformerSkinHandle<'a> {
    /// Returns the parent geometry mesh node.
    ///
    /// If there are two or more parent models, one of them is returned.
    /// If you want to get all of them, use [`ObjectHandle::destination_objects`]
    /// and filter by yourself.
    // NOTE: I (the author) am not sure the parent `Geometry`(`Mesh`) object
    // is just one.
    pub fn parent_geometry_mesh(&self) -> Result<GeometryMeshHandle<'a>> {
        self.as_object()
            .destination_objects()
            .filter(|conn| !conn.has_label())
            .filter_map(|conn| conn.destination())
            .find_map(|obj| GeometryMeshHandle::from_object(&obj).ok())
            .ok_or_else(|| {
                error!(
                    "`Deformer(Skin)` object is expected to have \
                    a parent `Geometry(Mesh)` object, but not found"
                )
            })
    }

    /// Returns an iterator of the child clusters.
    #[inline]
    #[must_use]
    pub fn child_clusters(&self) -> ChildClusters<'a> {
        ChildClusters {
            sources: self.as_object().source_objects(),
        }
    }
}

impl<'a> ObjectSubtypeHandle<'a> for DeformerSkinHandle<'a> {
    type NodeId = DeformerSkinNodeId;

    #[inline]
    fn from_object(object: &ObjectHandle<'a>) -> Result<Self> {
        DeformerHandle::from_object(object).and_then(|deformer| Self::from_deformer(&deformer))
    }

    #[inline]
    fn as_object(&self) -> &ObjectHandle<'a> {
        &self.object.as_object()
    }

    #[inline]
    fn node_id(&self) -> Self::NodeId {
        DeformerSkinNodeId(self.as_object().node_id())
    }
}

impl<'a> AsRef<ObjectHandle<'a>> for DeformerSkinHandle<'a> {
    #[inline]
    fn as_ref(&self) -> &ObjectHandle<'a> {
        self.as_object()
    }
}

impl<'a> AsRef<DeformerHandle<'a>> for DeformerSkinHandle<'a> {
    #[inline]
    fn as_ref(&self) -> &DeformerHandle<'a> {
        self.as_deformer()
    }
}

/// Iterator of `SubDeformer`(`Cluster`) nodes which are children of a `Deformer`(`Skin`) node.
#[derive(Debug, Clone)]
pub struct ChildClusters<'a> {
    /// Source objects.
    sources: ConnectionsForObject<'a>,
}

impl<'a> Iterator for ChildClusters<'a> {
    type Item = SubDeformerClusterHandle<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.sources
            .by_ref()
            .filter(|conn| !conn.has_label())
            .filter_map(|conn| conn.source())
            .find_map(|obj| SubDeformerClusterHandle::from_object(&obj).ok())
    }
}
