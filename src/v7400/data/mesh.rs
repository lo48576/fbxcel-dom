//! Mesh data.

pub use self::{
    control_point::{ControlPointIndex, ControlPoints},
    polygon_vertex_index::{PolygonVertex, PolygonVertexIndex, PolygonVertices},
};

mod control_point;
mod polygon_vertex_index;
