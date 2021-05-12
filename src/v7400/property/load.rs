//! Property loaders.

use crate::v7400::property::PropertyHandle;

/// A trait for property node value loader.
pub trait LoadPropertyValue<'a> {
    /// Value type.
    type Value;
    /// Error type.
    type Error;

    /// Loads a value from the property node handle.
    fn load(self, node: &PropertyHandle<'a>) -> Result<Self::Value, Self::Error>;
}

impl<'a, F, T, E> LoadPropertyValue<'a> for F
where
    F: FnOnce(&PropertyHandle<'a>) -> Result<T, E>,
{
    type Value = T;
    type Error = E;

    #[inline]
    fn load(self, node: &PropertyHandle<'a>) -> Result<Self::Value, Self::Error> {
        self(node)
    }
}
