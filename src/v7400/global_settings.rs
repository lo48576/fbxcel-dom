//! Document-wide settings.

use crate::v7400::axis::{AxisSystem, SignedAxis};
use crate::v7400::properties::PropertiesNodeId;
use crate::v7400::property::loaders::PrimitiveLoader;
use crate::v7400::{Document, ObjectProperties, Result};

/// A proxy to document-wide settings.
#[derive(Debug, Clone, Copy)]
pub struct GlobalSettings<'a> {
    /// Objects properties of `/GlobalSettings` node.
    props: ObjectProperties<'a>,
}

impl<'a> GlobalSettings<'a> {
    /// Creates a new proxy to `GlobalSettings` props.
    pub(super) fn new(doc: &'a Document) -> Result<Self> {
        let global_settings_node = doc
            .tree()
            .root()
            .first_child_by_name("GlobalSettings")
            .ok_or_else(|| error!("expected `/GlobalSettings` node but not found"))?;
        let direct_props = global_settings_node
            .first_child_by_name("Properties70")
            .map(|node| PropertiesNodeId::new(node.node_id()));

        // I am not confident about native typename being `FbxGlobalSettings`,
        // but it seems likely.
        // Documents I investigated has no default properties (`PropertyTemplate`) for any FB
        let default_props = doc
            .definitions_cache()
            .props_node_id("GlobalSettings", "FbxGlobalSettings");
        Ok(Self {
            props: ObjectProperties::new(direct_props, default_props, doc),
        })
    }

    /// Returns the axis system.
    pub fn axis_system(&self) -> Result<AxisSystem> {
        let up = self.up_axis()?;
        let front = self.front_axis()?;
        let right = self.right_axis()?;

        AxisSystem::from_up_front_right(up, front, right).ok_or_else(|| {
            error!(
                "invalid axis system: (up, front, right) = ({}, {}, {})",
                up, front, right
            )
        })
    }

    /// Returns the up axis.
    pub fn up_axis(&self) -> Result<SignedAxis> {
        load_axis_from_prop("Up", self.up_axis_raw()?, self.up_axis_sign_raw()?)
    }

    /// Returns the front axis.
    pub fn front_axis(&self) -> Result<SignedAxis> {
        load_axis_from_prop("Front", self.front_axis_raw()?, self.front_axis_sign_raw()?)
    }

    /// Returns the "coord axis" (i.e. rightward axis).
    pub fn right_axis(&self) -> Result<SignedAxis> {
        load_axis_from_prop("Coord", self.coord_axis_raw()?, self.coord_axis_sign_raw()?)
    }

    /// Returns the raw `UpAxis` value.
    fn up_axis_raw(&self) -> Result<i32> {
        self.props
            .get("UpAxis")
            .ok_or_else(|| error!("expected `UpAxis` property but not found"))?
            .value(PrimitiveLoader::<i32>::new())
    }

    /// Returns the raw `UpAxisSign` value.
    fn up_axis_sign_raw(&self) -> Result<i32> {
        self.props
            .get("UpAxisSign")
            .ok_or_else(|| error!("expected `UpAxisSign` property but not found"))?
            .value(PrimitiveLoader::<i32>::new())
    }

    /// Return the raws `FrontAxis` value.
    fn front_axis_raw(&self) -> Result<i32> {
        self.props
            .get("FrontAxis")
            .ok_or_else(|| error!("expected `FrontAxis` property but not found"))?
            .value(PrimitiveLoader::<i32>::new())
    }

    /// Returns the raw `FrontAxisSign` value.
    fn front_axis_sign_raw(&self) -> Result<i32> {
        self.props
            .get("FrontAxisSign")
            .ok_or_else(|| error!("expected `FrontAxisSign` property but not found"))?
            .value(PrimitiveLoader::<i32>::new())
    }

    /// Returns the raw `CoordAxis` value.
    fn coord_axis_raw(&self) -> Result<i32> {
        self.props
            .get("CoordAxis")
            .ok_or_else(|| error!("expected `CoordAxis` property but not found"))?
            .value(PrimitiveLoader::<i32>::new())
    }

    /// Returns the raw `CoordAxisSign` value.
    fn coord_axis_sign_raw(&self) -> Result<i32> {
        self.props
            .get("CoordAxisSign")
            .ok_or_else(|| error!("expected `CoordAxisSign` property but not found"))?
            .value(PrimitiveLoader::<i32>::new())
    }
}

/// Loads a signed axis from the given property values for axis and axis sign.
#[inline]
fn load_axis_from_prop(axis_name: &str, axis: i32, axis_sign: i32) -> Result<SignedAxis> {
    match (axis, axis_sign) {
        (0, 1) => Ok(SignedAxis::PosX),
        (0, -1) => Ok(SignedAxis::NegX),
        (1, 1) => Ok(SignedAxis::PosY),
        (1, -1) => Ok(SignedAxis::NegY),
        (2, 1) => Ok(SignedAxis::PosZ),
        (2, -1) => Ok(SignedAxis::NegZ),
        _ => {
            if !(0..=2).contains(&axis) {
                return Err(error!(
                    "invalid `{}Axis` property value: expected 0, 1, or 2 but got {}",
                    axis_name, axis
                ));
            }
            if (axis_sign == 1) || (axis_sign == -1) {
                return Err(error!(
                    "invalid `{}AxisSign` property value: expected 1 or -1, but got {}",
                    axis_name, axis_sign
                ));
            }

            unreachable!(
                "at least one of axis or axis sign must be invalid: \
                axis_name={:?}, axis={:?}, axis_sign={:?}",
                axis_name, axis, axis_sign
            );
        }
    }
}
