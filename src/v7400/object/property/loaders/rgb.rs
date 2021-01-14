//! `rgb` integration.

use std::marker::PhantomData;

use crate::v7400::object::property::{loaders::check_attrs_len, LoadProperty, PropertyHandle};

/// `rgb` crate color type loader.
///
/// This does minimal checks about `data_type` and `label`.
/// If you want to check property type precisely, you should make another
/// loader type by purpose.
///
/// Note that `f32` and `f64` is **NOT** converted automatically by this loader.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RgbLoader<T>(PhantomData<fn() -> T>);

impl<T> RgbLoader<T> {
    /// Creates a new `RgbLoader`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> Default for RgbLoader<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Clone for RgbLoader<T> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}

impl<T> Copy for RgbLoader<T> {}

macro_rules! read_nth_value {
    ($node:expr, $value_part:expr, $getter:ident, $target_name:expr, $index:expr) => {
        $value_part[$index]
            .$getter()
            .map_err(|ty| prop_type_err!($target_name, ty, $node))?
    };
}

macro_rules! load_rgb_value {
    (rgb, $node:expr, $value_part:expr, $getter:ident, $target_name:expr) => {
        rgb::RGB {
            r: read_nth_value!($node, $value_part, $getter, $target_name, 0),
            g: read_nth_value!($node, $value_part, $getter, $target_name, 1),
            b: read_nth_value!($node, $value_part, $getter, $target_name, 2),
        }
    };
    (rgba, $node:expr, $value_part:expr, $getter:ident, $target_name:expr) => {
        rgb::RGBA {
            r: read_nth_value!($node, $value_part, $getter, $target_name, 0),
            g: read_nth_value!($node, $value_part, $getter, $target_name, 1),
            b: read_nth_value!($node, $value_part, $getter, $target_name, 2),
            a: read_nth_value!($node, $value_part, $getter, $target_name, 3),
        }
    };
}

macro_rules! impl_loader {
    ($ty_elem:ty, $getter:ident, $kind:tt, $base:ident, $len:tt) => {
        impl_loader! {
            @impl,
            $ty_elem,
            $getter,
            $kind,
            $base,
            $len,
            concat!(
                "`rgb::",
                stringify!($base),
                "<",
                stringify!($ty_target),
                ">`"
            )
        }
    };
    (@impl, $ty_elem:ty, $getter:ident, $kind:tt, $base:ident, $len:tt, $target_name:expr) => {
        impl LoadProperty<'_> for RgbLoader<rgb::$base<$ty_elem>> {
            type Value = rgb::$base<$ty_elem>;
            type Error = anyhow::Error;

            fn expecting(&self) -> String {
                $target_name.into()
            }

            fn load(self, node: &PropertyHandle<'_>) -> Result<Self::Value, Self::Error> {
                let value_part = check_attrs_len(node, $len, $target_name)?;
                Ok(load_rgb_value!(
                    $kind,
                    node,
                    value_part,
                    $getter,
                    $target_name
                ))
            }
        }
    };
}

impl_loader! { f32, get_f32_or_type, rgb, RGB, 3 }
impl_loader! { f64, get_f64_or_type, rgb, RGB, 3 }
impl_loader! { f32, get_f32_or_type, rgba, RGBA, 4 }
impl_loader! { f64, get_f64_or_type, rgba, RGBA, 4 }
