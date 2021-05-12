//! Primitive type value loader.

use std::marker::PhantomData;

use fbxcel::low::v7400::AttributeValue as A;

use crate::v7400::property::LoadPropertyValue;
use crate::v7400::{Error, PropertyHandle};

/// Primitive type value loader.
///
/// This does minimal checks about `typename` and `label`. If you want to check
/// property type precisely, you should implement another loader by purpose.
///
/// # Supported types
///
/// Supported types are: `bool`, `i16`, `i32`, `i64`, `f32`, and `f64`.
/// Note that `u16`, `u32`, and `u64` is not implemented, as there are multiple
/// ways to extend signed integer into unsigned larger integer (i.e. zero
/// extension or sign extension).
///
/// # Type conversions
///
/// **This does minimal lossy type conversions** listed below:
///
/// * To get boolean, an integer value (`i16`, `i32`, and `i64`) is converted to boolean.
/// * To get `f32`, an `f64` value is converted to `f32`, using `_ as f32` cast.
///     + This may change in future, for exmaple when lossy conversion trait is
///       introduced to std.
///
/// If you want the loader to avoid type conversion (for example avoid loading
/// `i32` property as `i64` integer), use [`StrictPrimitiveLoader`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrimitiveLoader<T>(PhantomData<fn() -> T>);

impl<T> PrimitiveLoader<T> {
    /// Creates a new `PrimitiveLoader`.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> Default for PrimitiveLoader<T> {
    #[inline]
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl LoadPropertyValue<'_> for PrimitiveLoader<bool> {
    type Value = bool;
    type Error = Error;

    fn load(self, node: &PropertyHandle<'_>) -> Result<Self::Value, Self::Error> {
        match node.value_raw()? {
            [A::Bool(v)] => Ok(*v),
            [A::I16(v)] => Ok(*v != 0),
            [A::I32(v)] => Ok(*v != 0),
            [A::I64(v)] => Ok(*v != 0),
            [v] => Err(error!(
                "expected a boolean or an integer, but got {:?}",
                v.type_()
            )),
            v => Err(error!(
                "expected a boolean or an integer, but got {:?} values",
                v.len()
            )),
        }
    }
}

impl LoadPropertyValue<'_> for PrimitiveLoader<i16> {
    type Value = i16;
    type Error = Error;

    fn load(self, node: &PropertyHandle<'_>) -> Result<Self::Value, Self::Error> {
        match node.value_raw()? {
            [A::I16(v)] => Ok(*v),
            [v] => Err(error!("expected an `i16` value, but got {:?}", v.type_())),
            v => Err(error!(
                "expected an `i16` value, but got {:?} values",
                v.len()
            )),
        }
    }
}

impl LoadPropertyValue<'_> for PrimitiveLoader<i32> {
    type Value = i32;
    type Error = Error;

    fn load(self, node: &PropertyHandle<'_>) -> Result<Self::Value, Self::Error> {
        match node.value_raw()? {
            [A::I16(v)] => Ok(i32::from(*v)),
            [A::I32(v)] => Ok(*v),
            [v] => Err(error!("expected an `i32` value, but got {:?}", v.type_())),
            v => Err(error!(
                "expected an `i32` value, but got {:?} values",
                v.len()
            )),
        }
    }
}

impl LoadPropertyValue<'_> for PrimitiveLoader<i64> {
    type Value = i64;
    type Error = Error;

    fn load(self, node: &PropertyHandle<'_>) -> Result<Self::Value, Self::Error> {
        match node.value_raw()? {
            [A::I16(v)] => Ok(i64::from(*v)),
            [A::I32(v)] => Ok(i64::from(*v)),
            [A::I64(v)] => Ok(*v),
            [v] => Err(error!("expected an `i64` value, but got {:?}", v.type_())),
            v => Err(error!(
                "expected an `i64` value, but got {:?} values",
                v.len()
            )),
        }
    }
}

impl LoadPropertyValue<'_> for PrimitiveLoader<f32> {
    type Value = f32;
    type Error = Error;

    fn load(self, node: &PropertyHandle<'_>) -> Result<Self::Value, Self::Error> {
        match node.value_raw()? {
            [A::F32(v)] => Ok(*v),
            [A::F64(v)] => Ok(*v as f32),
            [v] => Err(error!("expected an `f64` value, but got {:?}", v.type_())),
            v => Err(error!(
                "expected an `f32` value, but got {:?} values",
                v.len()
            )),
        }
    }
}

impl LoadPropertyValue<'_> for PrimitiveLoader<f64> {
    type Value = f64;
    type Error = Error;

    fn load(self, node: &PropertyHandle<'_>) -> Result<Self::Value, Self::Error> {
        match node.value_raw()? {
            [A::F32(v)] => Ok(f64::from(*v)),
            [A::F64(v)] => Ok(*v),
            [v] => Err(error!("expected an `f64` value, but got {:?}", v.type_())),
            v => Err(error!(
                "expected an `f64` value, but got {:?} values",
                v.len()
            )),
        }
    }
}

/// Strict primitive type value loader.
///
/// This does minimal checks about `typename` and `label`. If you want to check
/// property type precisely, you should implement another loader by purpose.
///
/// # Supported types
///
/// Supported types are: `bool`, `i16`, `i32`, `i64`, `f32`, and `f64`.
/// Note that `u16`, `u32`, and `u64` is not implemented, as they are not
/// supported natively by the FBX format.
///
/// # Type conversions
///
/// **This does no type conversions**.
/// If you want the loader to perform type conversion (for example loading `i32`
/// property as `i64` integer), use [`PrimitiveLoader`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StrictPrimitiveLoader<T>(PhantomData<fn() -> T>);

impl<T> StrictPrimitiveLoader<T> {
    /// Creates a new `StrictPrimitiveLoader`.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> Default for StrictPrimitiveLoader<T> {
    #[inline]
    fn default() -> Self {
        Self(PhantomData)
    }
}

macro_rules! impl_strict_primitive_loader {
    ($target:ty, $attr_variant:ident, $name:expr) => {
        impl LoadPropertyValue<'_> for StrictPrimitiveLoader<$target> {
            type Value = $target;
            type Error = Error;

            fn load(self, node: &PropertyHandle<'_>) -> Result<Self::Value, Self::Error> {
                match node.value_raw()? {
                    [A::$attr_variant(v)] => Ok(*v),
                    [v] => Err(error!(
                        "expected single `{}` value, but got {:?}",
                        $name,
                        v.type_()
                    )),
                    v => Err(error!(
                        "expected single `{}` value, but got {:?} values",
                        $name,
                        v.len()
                    )),
                }
            }
        }
    };
}

impl_strict_primitive_loader!(bool, Bool, "boolean");
impl_strict_primitive_loader!(i16, I16, "i16");
impl_strict_primitive_loader!(i32, I32, "i32");
impl_strict_primitive_loader!(i64, I64, "i64");
impl_strict_primitive_loader!(f32, F32, "f32");
impl_strict_primitive_loader!(f64, F64, "f64");
