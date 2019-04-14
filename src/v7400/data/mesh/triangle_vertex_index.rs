//! Triangle vertex index.

use crate::v7400::data::mesh::{
    ControlPointIndex, PolygonIndex, PolygonVertex, PolygonVertexIndex, PolygonVertices,
};

/// Triange vertex index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TriangleVertexIndex(usize);

impl TriangleVertexIndex {
    /// Creates a new `TriangleVertexIndex`.
    pub(crate) fn new(i: usize) -> Self {
        Self(i)
    }

    /// Returns the triange vertex index.
    pub fn to_usize(self) -> usize {
        self.0
    }

    /// Returns the triange vertex index.
    #[deprecated(since = "0.0.3", note = "Renamed to `to_usize`")]
    pub fn get(self) -> usize {
        self.to_usize()
    }

    /// Returns triangle index.
    pub fn triangle_index(self) -> TriangleIndex {
        TriangleIndex::new(self.0 / 3)
    }
}

/// Triangle vertices (this is arary of control point indices).
///
/// "Triangle vertex" means "index of control point".
#[derive(Debug, Clone)]
pub struct TriangleVertices<'a> {
    /// Source polygon vertices which can contain non-triangles.
    polygon_vertices: PolygonVertices<'a>,
    /// A map from triangle vertex to polygon vertex index.
    tri_pv_indices: Vec<PolygonVertexIndex>,
    /// A map from triangle index to polygon index.
    tri_poly_indices: Vec<PolygonIndex>,
}

impl<'a> TriangleVertices<'a> {
    /// Creates a new `TriangleVertices`.
    pub(crate) fn new(
        polygon_vertices: PolygonVertices<'a>,
        tri_pv_indices: Vec<PolygonVertexIndex>,
        tri_poly_indices: Vec<PolygonIndex>,
    ) -> Self {
        Self {
            polygon_vertices,
            tri_pv_indices,
            tri_poly_indices,
        }
    }

    /// Returns polygon vertex index corresponding to the given triangle vertex.
    pub fn get_pvi(&self, tri_vi: TriangleVertexIndex) -> Option<PolygonVertexIndex> {
        self.tri_pv_indices.get(tri_vi.to_usize()).cloned()
    }

    /// Returns polygon vertex corresponding to the given triangle vertex.
    pub(crate) fn get_pv(&self, tri_vi: TriangleVertexIndex) -> Option<PolygonVertex> {
        self.get_pvi(tri_vi)
            .and_then(|pvi| self.polygon_vertices.polygon_vertex(pvi))
    }

    /// Returns control point index corresponding to the given triangle vertex.
    pub fn get_control_point(&self, tri_vi: TriangleVertexIndex) -> Option<ControlPointIndex> {
        self.get_pv(tri_vi).map(ControlPointIndex::from_pv)
    }

    /// Returns the number of triangle vertices.
    pub fn len(&self) -> usize {
        self.tri_pv_indices.len()
    }

    /// Returns whether or not there are no triangle vertices.
    pub fn is_empty(&self) -> bool {
        self.tri_pv_indices.is_empty()
    }

    /// Returns an iterator of control point indices.
    pub fn iter_control_point_indices<'b>(
        &'b self,
    ) -> impl Iterator<Item = Option<ControlPointIndex>> + 'b {
        (0..self.len())
            .map(TriangleVertexIndex::new)
            .map(move |tri_vi| self.get_control_point(tri_vi))
    }

    /// Returns polygon index for the given triangle index.
    pub fn get_polygon_index(&self, tri_i: TriangleIndex) -> Option<PolygonIndex> {
        self.tri_poly_indices.get(tri_i.to_usize()).cloned()
    }

    /// Returns an iterator of triangle vertex indices.
    pub fn triangle_vertex_indices(&self) -> impl Iterator<Item = TriangleVertexIndex> {
        (0..self.len()).map(TriangleVertexIndex::new)
    }
}

/// Triangle index.
#[derive(Debug, Clone, Copy)]
pub struct TriangleIndex(usize);

impl TriangleIndex {
    /// Creates a new `TriangleIndex`.
    fn new(v: usize) -> Self {
        Self(v)
    }

    /// Returns the index.
    pub fn to_usize(self) -> usize {
        self.0
    }

    /// Returns the index.
    #[deprecated(since = "0.0.3", note = "Renamed to `to_usize`")]
    pub fn get(self) -> usize {
        self.to_usize()
    }
}
