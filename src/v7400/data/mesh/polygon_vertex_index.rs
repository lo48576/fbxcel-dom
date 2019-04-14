//! Polygon vertex index.

use failure::{bail, Error};

use crate::v7400::data::mesh::{ControlPointIndex, ControlPoints, TriangleVertices};

/// Polygon vertex index.
///
/// This is index of control point index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolygonVertexIndex(usize);

impl PolygonVertexIndex {
    /// Creates a new `PolygonVertexIndex`.
    pub(crate) fn new(v: usize) -> Self {
        Self(v)
    }

    /// Returns the raw index.
    pub(crate) fn to_usize(self) -> usize {
        self.0
    }
}

/// Raw polygon vertices (control point indices) data.
#[derive(Debug, Clone, Copy)]
pub struct RawPolygonVertices<'a> {
    /// Polygon vertices (control point indices).
    data: &'a [i32],
}

impl<'a> RawPolygonVertices<'a> {
    /// Creates a new `RawPolygonVertices`.
    pub(crate) fn new(data: &'a [i32]) -> Self {
        Self { data }
    }

    /// Returns a polygon vertex at the given index.
    pub(crate) fn get(&self, pvi: PolygonVertexIndex) -> Option<PolygonVertex> {
        self.data
            .get(pvi.to_usize())
            .cloned()
            .map(PolygonVertex::new)
    }
}

/// Polygon vertices and control points data.
#[derive(Debug, Clone, Copy)]
pub struct PolygonVertices<'a> {
    /// Control points.
    control_points: ControlPoints<'a>,
    /// Polygon vertices (control point indices).
    polygon_vertices: RawPolygonVertices<'a>,
}

impl<'a> PolygonVertices<'a> {
    /// Creates a new `PolygonVertices`.
    pub(crate) fn new(
        control_points: ControlPoints<'a>,
        polygon_vertices: RawPolygonVertices<'a>,
    ) -> Self {
        Self {
            control_points,
            polygon_vertices,
        }
    }

    /// Returns a polygon vertex at the given index.
    pub fn polygon_vertex(&self, pvi: PolygonVertexIndex) -> Option<PolygonVertex> {
        self.polygon_vertices.get(pvi)
    }

    /// Returns a control point at the given index.
    pub fn control_point_by_pvi(&self, pvi: PolygonVertexIndex) -> Option<[f64; 3]> {
        self.polygon_vertex(pvi)
            .and_then(|pv| self.control_point_by_pv(pv))
    }

    /// Returns a control point at the given index.
    pub fn control_point_by_pv(&self, pv: PolygonVertex) -> Option<[f64; 3]> {
        self.control_point_by_cpi(pv.into())
    }

    /// Returns a control point at the given index.
    pub fn control_point_by_cpi(&self, cpi: ControlPointIndex) -> Option<[f64; 3]> {
        self.control_points.get(cpi)
    }

    /// Triangulates the polygons and returns indices map.
    pub fn triangulate_each<F>(&self, mut triangulator: F) -> Result<TriangleVertices<'a>, Error>
    where
        F: FnMut(
                &Self,
                &[PolygonVertexIndex],
                &mut Vec<[PolygonVertexIndex; 3]>,
            ) -> Result<(), Error>
            + Copy,
    {
        let len = self.polygon_vertices.data.len();
        let mut tri_pv_indices = Vec::new();
        let mut tri_poly_indices = Vec::new();

        let mut current_poly_index = 0;
        let mut current_poly_pvis = Vec::new();
        let mut pv_index_start = 0;
        let mut tri_results = Vec::new();
        while pv_index_start < len {
            current_poly_pvis.clear();
            tri_results.clear();

            let pv_index_next_start = match self.polygon_vertices.data[pv_index_start..]
                .iter()
                .cloned()
                .map(PolygonVertex::new)
                .position(PolygonVertex::is_end)
            {
                Some(v) => pv_index_start + v + 1,
                None => bail!(
                    "Incomplete polygon found: pv_index_start={:?}, len={}",
                    pv_index_start,
                    len
                ),
            };
            current_poly_pvis
                .extend((pv_index_start..pv_index_next_start).map(PolygonVertexIndex::new));
            triangulator(self, &current_poly_pvis, &mut tri_results)?;
            tri_pv_indices.extend(tri_results.iter().flat_map(|tri| tri));
            tri_poly_indices
                .extend((0..tri_results.len()).map(|_| PolygonIndex::new(current_poly_index)));

            pv_index_start = pv_index_next_start;
            current_poly_index += 1;
        }

        Ok(TriangleVertices::new(
            *self,
            tri_pv_indices,
            tri_poly_indices,
        ))
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
    pub fn to_u32(self) -> u32 {
        if self.0 < 0 {
            !self.0 as u32
        } else {
            self.0 as u32
        }
    }

    /// Returns the polygon vertex, i.e. index of control point, in `u32`.
    #[deprecated(since = "0.0.3", note = "Renamed to `to_u32`")]
    pub fn get_u32(self) -> u32 {
        self.to_u32()
    }
}

impl From<PolygonVertex> for ControlPointIndex {
    fn from(pv: PolygonVertex) -> Self {
        Self::new(pv.to_u32())
    }
}

impl From<&PolygonVertex> for ControlPointIndex {
    fn from(pv: &PolygonVertex) -> Self {
        Self::new(pv.to_u32())
    }
}

/// Polygon index.
#[derive(Debug, Clone, Copy)]
pub struct PolygonIndex(usize);

impl PolygonIndex {
    /// Creates a new `PolygonIndex`.
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
