//! Binary and string type value loader.

use fbxcel::low::v7400::AttributeValue as A;

use crate::v7400::property::LoadPropertyNodeValue;
use crate::v7400::{Error, PropertyNodeHandle};

/// Generates impls for a loader of an owned type.
macro_rules! impl_owned_loader {
    ($loader:ty, $target:ty, $attr_variant:ident, $name:expr) => {
        impl $loader {
            /// Creates a new loader.
            #[inline]
            #[must_use]
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl LoadPropertyNodeValue<'_> for $loader {
            type Value = $target;
            type Error = Error;

            fn load(self, node: &PropertyNodeHandle<'_>) -> Result<Self::Value, Self::Error> {
                match node.value_raw()? {
                    [A::$attr_variant(v)] => Ok(v.clone()),
                    [v] => Err(error!("expected {} but got {:?}", $name, v.type_())),
                    v => Err(error!(
                        "expected single {} but got {} values",
                        $name,
                        v.len()
                    )),
                }
            }
        }
    };
}

/// Owned string loader returning `String`.
///
/// This does minimal checks about `typename` and `label`. If you want to check
/// property type precisely, you should implement another loader by purpose.
///
/// This does not load binary property.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct OwnedStringLoader(());

impl_owned_loader!(OwnedStringLoader, String, String, "string");

/// Owned binary loader returning `Vec<u8>`.
///
/// This does minimal checks about `typename` and `label`. If you want to check
/// property type precisely, you should implement another loader by purpose.
///
/// This does not load string property.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct OwnedBinaryLoader(());

impl_owned_loader!(OwnedBinaryLoader, Vec<u8>, Binary, "binary");

/// Generates impls for a loader of a borrowed type.
macro_rules! impl_borrowed_loader {
    ($loader:ty, $target:ty, $attr_variant:ident, $name:expr) => {
        impl $loader {
            /// Creates a new loader.
            #[inline]
            #[must_use]
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl<'a> LoadPropertyNodeValue<'a> for $loader {
            type Value = $target;
            type Error = Error;

            fn load(self, node: &PropertyNodeHandle<'a>) -> Result<Self::Value, Self::Error> {
                match node.value_raw()? {
                    [A::$attr_variant(v)] => Ok(v),
                    [v] => Err(error!("expected {} but got {:?}", $name, v.type_())),
                    v => Err(error!(
                        "expected single {} but got {} values",
                        $name,
                        v.len()
                    )),
                }
            }
        }
    };
}

/// Borrowed string loader returning `&str`.
///
/// This does minimal checks about `typename` and `label`. If you want to check
/// property type precisely, you should implement another loader by purpose.
///
/// This does not load binary property.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct BorrowedStringLoader(());

impl_borrowed_loader!(BorrowedStringLoader, &'a str, String, "string");

/// Borrowed binary loader returning `&[u8]`.
///
/// This does minimal checks about `typename` and `label`. If you want to check
/// property type precisely, you should implement another loader by purpose.
///
/// This does not load string property.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct BorrowedBinaryLoader(());

impl_borrowed_loader!(BorrowedBinaryLoader, &'a [u8], Binary, "binary");
