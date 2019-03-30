//! Polygon vertex index.

use crate::v7400::data::mesh::{ControlPointIndex, ControlPoints};

/// Polygon vertex index.
///
/// This is index of control point index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolygonVertexIndex(usize);

impl PolygonVertexIndex {
    /// Returns the raw index.
    pub(crate) fn get(self) -> usize {
        self.0
    }

    /// Returns the vertex.
    pub fn get_vertex(
        self,
        cps: &ControlPoints<'_>,
        pvs: &PolygonVertices<'_>,
    ) -> Option<[f64; 3]> {
        cps.get_cp_f64(pvs.get_pv(self)?.into())
    }
}

/// Polygon vertex indices.
#[derive(Debug, Clone, Copy)]
pub struct PolygonVertices<'a> {
    /// Polygon vertex indices.
    data: &'a [i32],
}

impl<'a> PolygonVertices<'a> {
    /// Creates a new `PolygonVertices`.
    pub(crate) fn new(data: &'a [i32]) -> Self {
        Self { data }
    }

    /// Returns a polygon vertex at the given index.
    pub fn get_pv(&self, pvi_i: PolygonVertexIndex) -> Option<PolygonVertex> {
        self.data.get(pvi_i.get()).cloned().map(PolygonVertex::new)
    }
}

/// Polygon vertex.
///
/// `PolygonVertex` = control point index + polygon end marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolygonVertex(i32);

impl PolygonVertex {
    /// Creates a new `PolygonVertex`.
    pub(crate) fn new(i: i32) -> Self {
        Self(i)
    }

    /// Returns whether the polygon vertex index is the end of a polygon.
    pub fn is_end(self) -> bool {
        self.0 < 0
    }

    /// Returns the polygon vertex, i.e. index of control point, in `u32`.
    pub fn get_u32(self) -> u32 {
        if self.0 < 0 {
            !self.0 as u32
        } else {
            self.0 as u32
        }
    }
}

impl From<PolygonVertex> for ControlPointIndex {
    fn from(pv: PolygonVertex) -> Self {
        Self::new(pv.get_u32())
    }
}
