//! Mesh data.

pub use self::{
    control_point::ControlPointIndex,
    polygon_vertex_index::{
        IntoCpiWithPolyVerts, PolygonIndex, PolygonVertex, PolygonVertexIndex, PolygonVertices,
    },
    triangle_vertex_index::{
        IntoCpiWithTriVerts, IntoPvWithTriVerts, TriangleIndex, TriangleVertexIndex,
        TriangleVertices,
    },
};
pub(crate) use self::{control_point::ControlPoints, polygon_vertex_index::RawPolygonVertices};

mod control_point;
pub mod layer;
mod polygon_vertex_index;
mod triangle_vertex_index;
