//! The Global Settings for the FBX file. See struct `GlobalSettings`.

use crate::v7400::object::property::PropertiesHandle;
use fbxcel::low::v7400::AttributeValue;

/// The Global Settings for the FBX file.
/// Similar to http://docs.autodesk.com/FBX/2014/ENU/FBX-SDK-Documentation/index.html?url=cpp_ref/class_fbx_global_settings.html,topicNumber=cpp_ref_class_fbx_global_settings_html121c7acd-33fd-4411-8710-deeff384f0f4
pub struct GlobalSettings<'a> {
    pub(crate) properties: PropertiesHandle<'a>,
}

impl<'a> GlobalSettings<'a> {
    /// Returns the unit scale of the file. This is relative to 1 centimeter: A file
    /// with scale in meters will have a UnitScaleFactor of "100.0".
    pub fn unit_scale_factor(&self) -> f64 {
        match self.unit_scale_factor_raw() {
            Some(unit) => unit,
            None => 1.0, // The default unit is assumed to be centimeters.
        }
    }

    /// Returns the raw UnitScaleFactor of the file. This is relative to 1 centimeter. ie. A file
    /// with scale in meters will have a UnitScaleFactor of "100.0"
    ///
    /// This function will return None if no unit is specified, however an FBX with no units
    /// is assumed to be in centimeters by default.
    ///
    /// In most cases, you should use `unit_scale_factor` instead.
    pub fn unit_scale_factor_raw(&self) -> Option<f64> {
        match self
            .properties
            .get_property("UnitScaleFactor")?
            .value_part()
            .get(0)?
        {
            AttributeValue::F64(unit) => Some(*unit),
            _ => None,
        }
    }

    /// Returns a property accessor handle that can be used to query properties using the string name.
    ///
    /// This is an escape hatch for properties that are not yet exposed as functions on this object.
    pub fn raw_properties(&self) -> &PropertiesHandle<'a> {
        &self.properties
    }
}
