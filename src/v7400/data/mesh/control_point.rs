//! Control point.

use std::convert::TryInto;

use crate::v7400::data::mesh::PolygonVertex;

/// Control point index (in other words, polygon vertex).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ControlPointIndex(u32);

impl ControlPointIndex {
    /// Creates a new `ControlPointIndex`.
    pub(crate) fn new(v: u32) -> Self {
        Self(v)
    }

    /// Returns the raw index.
    pub fn to_u32(self) -> u32 {
        self.0
    }

    /// Returns the raw index.
    #[deprecated(since = "0.0.3", note = "Renamed to `to_u32`")]
    pub fn get_u32(self) -> u32 {
        self.to_u32()
    }

    /// Creates a new `ControlPointIndex` from the given `PolygonVertex`.
    pub(crate) fn from_pv(pv: PolygonVertex) -> Self {
        Self(pv.to_u32())
    }
}

/// Control points.
#[derive(Debug, Clone, Copy)]
pub struct ControlPoints<'a> {
    /// Control points.
    data: &'a [f64],
}

impl<'a> ControlPoints<'a> {
    /// Creates a new `ControlPoints`.
    pub(crate) fn new(data: &'a [f64]) -> Self {
        Self { data }
    }

    /// Returns a control point at the given index.
    pub fn get(&self, index: ControlPointIndex) -> Option<[f64; 3]> {
        let i3 = index.to_u32() as usize * 3;
        if self.data.len() < i3 + 2 {
            return None;
        }
        Some([self.data[i3], self.data[i3 + 1], self.data[i3 + 2]])
    }

    /// Returns an iterator of control points.
    pub fn iter(&self) -> impl Iterator<Item = [f64; 3]> + 'a {
        self.data
            .chunks_exact(3)
            .map(|arr| [arr[0], arr[1], arr[2]])
    }
}

impl std::ops::Index<ControlPointIndex> for ControlPoints<'_> {
    type Output = [f64; 3];

    /// Returns a control point at the given index.
    ///
    /// # Panics
    ///
    /// Panics if the given index is out of range.
    fn index(&self, i: ControlPointIndex) -> &Self::Output {
        self.data[(i.to_u32() as usize * 3)..]
            .try_into()
            .unwrap_or_else(|_| panic!("Index out of range: index={:?}", i))
    }
}
