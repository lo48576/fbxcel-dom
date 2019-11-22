//! `Geometry` object (mesh).

use anyhow::{format_err, Error};

use crate::v7400::{
    data::mesh::{layer::LayerHandle, ControlPoints, PolygonVertices, RawPolygonVertices},
    object::{deformer, geometry::GeometryHandle, model, TypedObjectHandle},
};

define_object_subtype! {
    /// `Geometry` node handle (mesh).
    MeshHandle: GeometryHandle
}

impl<'a> MeshHandle<'a> {
    /// Returns an iterator of parent model objects.
    pub fn models(&self) -> impl Iterator<Item = model::MeshHandle<'a>> {
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
    pub(crate) fn control_points(&self) -> Result<ControlPoints<'a>, Error> {
        self.node()
            .children_by_name("Vertices")
            .next()
            .ok_or_else(|| format_err!("`Vertices` child node not found for geometry mesh"))?
            .attributes()
            .get(0)
            .ok_or_else(|| format_err!("`Vertices` node has no attributes"))?
            .get_arr_f64_or_type()
            .map(ControlPoints::new)
            .map_err(|ty| {
                format_err!(
                    "`Vertices` has wrong type attribute: expected `[f64]` but got {:?}`",
                    ty
                )
            })
    }

    /// Returns polygon vertices without control points.
    pub(crate) fn raw_polygon_vertices(&self) -> Result<RawPolygonVertices<'a>, Error> {
        self.node()
            .children_by_name("PolygonVertexIndex")
            .next()
            .ok_or_else(|| {
                format_err!("`PolygonVertexIndex` child node not found for geometry mesh")
            })?
            .attributes()
            .get(0)
            .ok_or_else(|| format_err!("`PolygonVertexIndex` node has no attributes"))?
            .get_arr_i32_or_type()
            .map(RawPolygonVertices::new)
            .map_err(|ty| {
                format_err!(
                    "`PolygonVertexIndex` has wrong type attribute: expected `[i32]` but got {:?}`",
                    ty
                )
            })
    }

    /// Returns polygon vertices (control point indices) and control points.
    pub fn polygon_vertices(&self) -> Result<PolygonVertices<'a>, Error> {
        let control_points = self.control_points()?;
        let raw_polygon_vertices = self.raw_polygon_vertices()?;
        Ok(PolygonVertices::new(control_points, raw_polygon_vertices))
    }

    /// Returns layers.
    pub fn layers(&self) -> impl Iterator<Item = LayerHandle<'a>> {
        self.node().children_by_name("Layer").map(LayerHandle::new)
    }
}
