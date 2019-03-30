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

    /// Triangulates the polygons and returns indices map.
    pub fn triangulate_each<F>(
        &self,
        control_points: &ControlPoints<'a>,
        mut triangulator: F,
    ) -> Result<TriangleVertices<'a>, Error>
    where
        F: FnMut(
                &ControlPoints<'a>,
                &PolygonVertices<'a>,
                &[PolygonVertexIndex],
                &mut Vec<[PolygonVertexIndex; 3]>,
            ) -> Result<(), Error>
            + Copy,
    {
        let len = self.data.len();
        let mut tri_pv_indices = Vec::new();

        let mut current_poly_pvis = Vec::new();
        let mut pv_index_start = 0;
        let mut tri_results = Vec::new();
        while pv_index_start < len {
            current_poly_pvis.clear();
            tri_results.clear();

            let pv_index_next_start = match self.data[pv_index_start..]
                .iter()
                .cloned()
                .map(PolygonVertex::new)
                .position(|pv| pv.is_end())
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
            triangulator(control_points, self, &current_poly_pvis, &mut tri_results)?;
            tri_pv_indices.extend(tri_results.iter().flat_map(|tri| tri));

            pv_index_start = pv_index_next_start;
        }
        Ok(TriangleVertices::new(*self, tri_pv_indices))
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
