//! Mesh data.

pub use self::{
    control_point::{ControlPointIndex, ControlPoints},
    polygon_vertex_index::{PolygonVertex, PolygonVertexIndex, PolygonVertices},
    triangle_vertex_index::{TriangleVertexIndex, TriangleVertices},
};

mod control_point;
mod polygon_vertex_index;
mod triangle_vertex_index;
