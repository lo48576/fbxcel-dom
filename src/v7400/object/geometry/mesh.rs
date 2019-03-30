//! `Geometry` object (mesh).

use failure::{format_err, Error};

use crate::v7400::{
    data::mesh::{ControlPoints, PolygonVertices},
    object::{deformer, geometry::GeometryHandle, model, TypedObjectHandle},
};

define_object_subtype! {
    /// `Geometry` node handle (mesh).
    MeshHandle: GeometryHandle
}

impl<'a> MeshHandle<'a> {
    /// Returns an iterator of parent model objects.
    pub fn models(&self) -> impl Iterator<Item = model::MeshHandle<'a>> + 'a {
        self.destination_objects()
            .filter(|obj| obj.label().is_none())
            .filter_map(|obj| obj.object_handle())
            .filter_map(|obj| match obj.get_typed() {
                TypedObjectHandle::Model(model::TypedModelHandle::Mesh(o)) => Some(o),
                _ => None,
            })
    }

    /// Returns a child deformer skin if available.
    pub fn skins(&self) -> impl Iterator<Item = deformer::SkinHandle<'a>> {
        self.source_objects()
            .filter(|obj| obj.label().is_none())
            .filter_map(|obj| obj.object_handle())
            .filter_map(|obj| match obj.get_typed() {
                TypedObjectHandle::Deformer(deformer::TypedDeformerHandle::Skin(o)) => Some(o),
                _ => None,
            })
    }

    /// Returns a child deformer blendshapes if available.
    pub fn blendshapes(&self) -> impl Iterator<Item = deformer::BlendShapeHandle<'a>> {
        self.source_objects()
            .filter(|obj| obj.label().is_none())
            .filter_map(|obj| obj.object_handle())
            .filter_map(|obj| match obj.get_typed() {
                TypedObjectHandle::Deformer(deformer::TypedDeformerHandle::BlendShape(o)) => {
                    Some(o)
                }
                _ => None,
            })
    }

    /// Returns control points.
    pub fn control_points(&self) -> Result<ControlPoints<'a>, Error> {
        self.node()
            .children_by_name("Vertices")
            .next()
            .ok_or_else(|| format_err!("`Vertices` child node not found for geometry mesh"))?
            .attributes()
            .get(0)
            .ok_or_else(|| format_err!("`Vertices` node has no children"))?
            .get_arr_f64_or_type()
            .map(ControlPoints::new)
            .map_err(|ty| {
                format_err!(
                    "`Vertices` has wrong type attribute: expected `[f64]` but got {:?}`",
                    ty
                )
            })
    }

    /// Returns polygon vertex indices.
    pub fn polygon_vertex_indices(&self) -> Result<PolygonVertices<'a>, Error> {
        self.node()
            .children_by_name("PolygonVertexIndex")
            .next()
            .ok_or_else(|| {
                format_err!("`PolygonVertexIndex` child node not found for geometry mesh")
            })?
            .attributes()
            .get(0)
            .ok_or_else(|| format_err!("`PolygonVertexIndex` node has no children"))?
            .get_arr_i32_or_type()
            .map(PolygonVertices::new)
            .map_err(|ty| {
                format_err!(
                    "`PolygonVertexIndex` has wrong type attribute: expected `[i32]` but got {:?}`",
                    ty
                )
            })
    }
}
