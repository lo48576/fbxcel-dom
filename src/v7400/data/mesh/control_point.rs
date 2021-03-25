//! Control point.

use mint::Point3;

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
}

/// Control points.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ControlPoints<'a> {
    /// Control points.
    data: &'a [f64],
}

impl<'a> ControlPoints<'a> {
    /// Creates a new `ControlPoints`.
    pub(crate) fn new(data: &'a [f64]) -> Self {
        Self { data }
    }

    /// Returns a control point at the given index.
    pub(crate) fn get(&self, index: ControlPointIndex) -> Option<Point3<f64>> {
        let i3 = index.to_u32() as usize * 3;
        if self.data.len() < i3 + 2 {
            return None;
        }
        Some(Point3::from_slice(&self.data[i3..]))
    }

    /// Returns an iterator through the control points
    pub(crate) fn iter(&self) -> anyhow::Result<impl Iterator<Item = Point3<f64>> + 'a> {
        if self.data.len() % 3 != 0 {
            return Err(anyhow::format_err!(
                "Mesh did not have valid vertex array size."
            ));
        }

        Ok(self.data.chunks(3).map(|chunk| Point3::from_slice(chunk)))
    }
}
