//! Array type value loader.

use fbxcel::low::v7400::AttributeValue as A;

use crate::v7400::property::LoadPropertyValue;
use crate::v7400::{Error, PropertyHandle};

/// Generates impls for an array loader type.
macro_rules! impl_fxx_arr_loader {
    ($loader:ident, $component:ty, $attr_variant:ident) => {
        impl<const N: usize> $loader<N> {
            /// Creates a new loader.
            #[inline]
            #[must_use]
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl<const N: usize> LoadPropertyValue<'_> for $loader<N>
        where
            [$component; N]: Default,
        {
            type Value = [$component; N];
            type Error = Error;

            fn load(self, node: &PropertyHandle<'_>) -> Result<Self::Value, Self::Error> {
                let raw = node.value_raw()?;
                if raw.len() != N {
                    return Err(error!(
                        "expected `[{}; {}]` but got {} values",
                        stringify!($component),
                        N,
                        raw.len()
                    ));
                }

                let mut arr: [$component; N] = Default::default();
                for (i, component) in raw.iter().enumerate() {
                    match component {
                        A::$attr_variant(v) => arr[i] = *v,
                        v => {
                            return Err(error!(
                                "expected an `f32` at `attrs.value_raw()[{}]`, but got {:?}",
                                i,
                                v.type_()
                            ))
                        }
                    }
                }

                Ok(arr)
            }
        }
    };
}

/// `f32` array type value loader returning `[f32; N]`.
///
/// This does minimal checks about `typename` and `label`. If you want to check
/// property type precisely, you should implement another loader by purpose.
///
/// Note that this loads not single `[f64]` property but multiple `f64`
/// properties. This is because many values such as vectors and matrices are
/// represented in this way.
///
/// This does not load `f64` components.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct F32ArrayLoader<const N: usize>(());

impl_fxx_arr_loader!(F32ArrayLoader, f32, F32);

/// `f64` array type value loader returning `[f64; N]`.
///
/// This does minimal checks about `typename` and `label`. If you want to check
/// property type precisely, you should implement another loader by purpose.
///
/// Note that this loads not single `[f64]` property but multiple `f64`
/// properties. This is because many values such as vectors and matrices are
/// represented in this way.
///
/// This does not load `f32` attributes.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct F64ArrayLoader<const N: usize>(());

impl_fxx_arr_loader!(F64ArrayLoader, f64, F64);

/// `f32` or `f64` array type value loader returning `[f64; N]`.
///
/// This does minimal checks about `typename` and `label`. If you want to check
/// property type precisely, you should implement another loader by purpose.
///
/// Note that this loads not single `[fN]` property but multiple `f32` or `f64`
/// properties. This is because many values such as vectors and matrices are
/// represented in this way.
///
/// This loads an array of `f32` or `f64`. Heterogeneous array can also be loaded.
/// `f32` components are converted to `f64`.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct FloatArrayLoader<const N: usize>(());

impl<const N: usize> FloatArrayLoader<N> {
    /// Creates a new loader.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<const N: usize> LoadPropertyValue<'_> for FloatArrayLoader<N>
where
    [f64; N]: Default,
{
    type Value = [f64; N];
    type Error = Error;

    fn load(self, node: &PropertyHandle<'_>) -> Result<Self::Value, Self::Error> {
        let raw = node.value_raw()?;
        if raw.len() != N {
            return Err(error!(
                "expected array of length {} with `f32` or `f64` but got {} values",
                N,
                raw.len()
            ));
        }

        let mut arr: [f64; N] = Default::default();
        for (i, component) in raw.iter().enumerate() {
            match component {
                A::F32(v) => arr[i] = f64::from(*v),
                A::F64(v) => arr[i] = *v,
                v => {
                    return Err(error!(
                        "expected an `f32` at `attrs.value_raw()[{}]`, but got {:?}",
                        i,
                        v.type_()
                    ))
                }
            }
        }

        Ok(arr)
    }
}
