//! Mesh data.

pub use self::{
    control_point::{ControlPointIndex, ControlPoints},
    polygon_vertex_index::{PolygonIndex, PolygonVertex, PolygonVertexIndex, PolygonVertices},
    triangle_vertex_index::{TriangleIndex, TriangleVertexIndex, TriangleVertices},
};

mod control_point;
pub mod layer;
mod polygon_vertex_index;
mod triangle_vertex_index;
