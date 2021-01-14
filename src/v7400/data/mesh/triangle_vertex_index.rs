//! Triangle vertex index.

use mint::Point3;

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

    /// Returns underlying polygon vertices.
    pub fn polygon_vertices(&self) -> PolygonVertices<'a> {
        self.polygon_vertices
    }

    /// Returns polygon vertex index corresponding to the given triangle vertex.
    pub fn polygon_vertex_index(&self, tri_vi: TriangleVertexIndex) -> Option<PolygonVertexIndex> {
        self.tri_pv_indices.get(tri_vi.to_usize()).cloned()
    }

    /// Returns polygon vertex corresponding to the given triangle vertex.
    pub fn polygon_vertex(&self, i: impl Into<IntoPvWithTriVerts>) -> Option<PolygonVertex> {
        i.into().polygon_vertex(self)
    }

    /// Returns control point index corresponding to the given triangle vertex.
    pub fn control_point_index(
        &self,
        i: impl Into<IntoCpiWithTriVerts>,
    ) -> Option<ControlPointIndex> {
        i.into().control_point_index(self)
    }

    /// Returns control point corresponding to the given triangle vertex.
    pub fn control_point(&self, i: impl Into<IntoCpiWithTriVerts>) -> Option<Point3<f64>> {
        self.control_point_index(i.into())
            .and_then(|cpi| self.polygon_vertices.control_point(cpi))
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
    pub fn iter_control_point_indices(
        &self,
    ) -> impl Iterator<Item = Option<ControlPointIndex>> + '_ {
        (0..self.len())
            .map(TriangleVertexIndex::new)
            .map(move |tri_vi| self.control_point_index(tri_vi))
    }

    /// Returns polygon index for the given triangle index.
    pub fn polygon_index(&self, tri_i: TriangleIndex) -> Option<PolygonIndex> {
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

/// A type to contain a value convertible into polygon vertex.
///
/// This is used for [`TriangleVertices::polygon_vertex`], but not intended to
/// be used directly by users.
///
/// [`TriangleVertices::polygon_vertex`]:
/// struct.TriangleVertices.html#method.polygon_vertex
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum IntoPvWithTriVerts {
    /// Polygon vertex.
    PolygonVertex(PolygonVertex),
    /// Polygon vertex index.
    PolygonVertexIndex(PolygonVertexIndex),
    /// Triangle vertex index.
    TriangleVertexIndex(TriangleVertexIndex),
}

impl IntoPvWithTriVerts {
    /// Returns polygon vertex.
    fn polygon_vertex(&self, triangle_vertices: &TriangleVertices<'_>) -> Option<PolygonVertex> {
        match *self {
            IntoPvWithTriVerts::PolygonVertex(pv) => Some(pv),
            IntoPvWithTriVerts::PolygonVertexIndex(pvi) => {
                triangle_vertices.polygon_vertices.polygon_vertex(pvi)
            }
            IntoPvWithTriVerts::TriangleVertexIndex(tri_vi) => triangle_vertices
                .polygon_vertex_index(tri_vi)
                .and_then(|pvi| triangle_vertices.polygon_vertices.polygon_vertex(pvi)),
        }
    }
}

impl From<PolygonVertex> for IntoPvWithTriVerts {
    fn from(i: PolygonVertex) -> Self {
        IntoPvWithTriVerts::PolygonVertex(i)
    }
}

impl From<&PolygonVertex> for IntoPvWithTriVerts {
    fn from(i: &PolygonVertex) -> Self {
        IntoPvWithTriVerts::PolygonVertex(*i)
    }
}

impl From<PolygonVertexIndex> for IntoPvWithTriVerts {
    fn from(i: PolygonVertexIndex) -> Self {
        IntoPvWithTriVerts::PolygonVertexIndex(i)
    }
}

impl From<&PolygonVertexIndex> for IntoPvWithTriVerts {
    fn from(i: &PolygonVertexIndex) -> Self {
        IntoPvWithTriVerts::PolygonVertexIndex(*i)
    }
}

impl From<TriangleVertexIndex> for IntoPvWithTriVerts {
    fn from(i: TriangleVertexIndex) -> Self {
        IntoPvWithTriVerts::TriangleVertexIndex(i)
    }
}

impl From<&TriangleVertexIndex> for IntoPvWithTriVerts {
    fn from(i: &TriangleVertexIndex) -> Self {
        IntoPvWithTriVerts::TriangleVertexIndex(*i)
    }
}

/// A type to contain a value convertible into control point index.
///
/// This is used for [`TriangleVertices::control_point_index`] and
/// [`TriangleVertices::control_point`], but not intended to be used directly by
/// users.
///
/// [`TriangleVertices::control_point_index`]:
/// struct.TriangleVertices.html#method.control_point_index
/// [`TriangleVertices::control_point`]:
/// struct.TriangleVertices.html#method.control_point
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum IntoCpiWithTriVerts {
    /// Control point index.
    ControlPointIndex(ControlPointIndex),
    /// A value which is convertible into polygon vertex.
    IntoPolygonVertex(IntoPvWithTriVerts),
}

impl IntoCpiWithTriVerts {
    /// Returns control point index.
    fn control_point_index(
        &self,
        triangle_vertices: &TriangleVertices<'_>,
    ) -> Option<ControlPointIndex> {
        match *self {
            IntoCpiWithTriVerts::ControlPointIndex(cpi) => Some(cpi),
            IntoCpiWithTriVerts::IntoPolygonVertex(into_pv) => {
                into_pv.polygon_vertex(triangle_vertices).map(Into::into)
            }
        }
    }
}

impl<T: Into<IntoPvWithTriVerts>> From<T> for IntoCpiWithTriVerts {
    fn from(i: T) -> Self {
        IntoCpiWithTriVerts::IntoPolygonVertex(i.into())
    }
}

impl From<ControlPointIndex> for IntoCpiWithTriVerts {
    fn from(i: ControlPointIndex) -> Self {
        IntoCpiWithTriVerts::ControlPointIndex(i)
    }
}

impl From<&ControlPointIndex> for IntoCpiWithTriVerts {
    fn from(i: &ControlPointIndex) -> Self {
        IntoCpiWithTriVerts::ControlPointIndex(*i)
    }
}
