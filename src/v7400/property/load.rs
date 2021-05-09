//! Property loaders.

use crate::v7400::property::PropertyNodeHandle;

/// A trait for property node value loader.
pub trait LoadPropertyNodeValue<'a> {
    /// Value type.
    type Value;
    /// Error type.
    type Error;

    /// Loads a value from the property node handle.
    fn load(self, node: &PropertyNodeHandle<'a>) -> Result<Self::Value, Self::Error>;
}

impl<'a, F, T, E> LoadPropertyNodeValue<'a> for F
where
    F: FnOnce(&PropertyNodeHandle<'a>) -> Result<T, E>,
{
    type Value = T;
    type Error = E;

    #[inline]
    fn load(self, node: &PropertyNodeHandle<'a>) -> Result<Self::Value, Self::Error> {
        self(node)
    }
}
