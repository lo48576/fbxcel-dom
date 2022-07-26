//! The Global Settings for the FBX file. See struct `GlobalSettings`.

use crate::v7400::object::property::PropertiesHandle;

/// The Global Settings for the FBX file.
/// Similar to http://docs.autodesk.com/FBX/2014/ENU/FBX-SDK-Documentation/index.html?url=cpp_ref/class_fbx_global_settings.html,topicNumber=cpp_ref_class_fbx_global_settings_html121c7acd-33fd-4411-8710-deeff384f0f4
pub struct GlobalSettings<'a> {
    pub(crate) properties: PropertiesHandle<'a>,
}

impl<'a> GlobalSettings<'a> {
    /// Returns a property accessor handle that can be used to query properties using the string name.
    pub fn raw_properties(&self) -> &PropertiesHandle<'a> {
        &self.properties
    }
}
