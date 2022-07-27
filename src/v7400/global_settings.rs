//! The Global Settings for the FBX file. See struct `GlobalSettings`.

use crate::v7400::document::Document;
use crate::v7400::object::property::PropertiesHandle;

/// The Global Settings for the FBX file.
///
/// Similar to
/// <http://docs.autodesk.com/FBX/2014/ENU/FBX-SDK-Documentation/cpp_ref/class_fbx_global_settings.html>.
pub struct GlobalSettings<'a> {
    /// Properties.
    properties: PropertiesHandle<'a>,
}

impl<'a> GlobalSettings<'a> {
    /// Creates a new `GlobalSettings` value from the document.
    #[must_use]
    pub(super) fn new(doc: &'a Document) -> Option<Self> {
        let settings_node = doc.tree().root().first_child_by_name("GlobalSettings")?;
        let properties = PropertiesHandle::from_node(settings_node, doc)?;
        Some(Self { properties })
    }

    /// Returns a property accessor handle that can be used to query properties using the string name.
    #[inline]
    #[must_use]
    pub fn raw_properties(&self) -> PropertiesHandle<'a> {
        self.properties
    }
}
