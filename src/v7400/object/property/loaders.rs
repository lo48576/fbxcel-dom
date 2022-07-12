//! Property loaders.

use anyhow::bail;
use fbxcel::low::v7400::AttributeValue;

use crate::v7400::object::property::PropertyHandle;

pub use self::{
    array::{F64Arr16Loader, F64Arr2Loader, F64Arr3Loader, F64Arr4Loader},
    binstr::{BorrowedBinaryLoader, BorrowedStringLoader, OwnedBinaryLoader, OwnedStringLoader},
    mint::MintLoader,
    primitive::PrimitiveLoader,
    rgb::RgbLoader,
    strict_primitive::{StrictF32Loader, StrictF64Loader},
};

/// Returns an object node property type error.
macro_rules! prop_type_err {
    ($v:expr, $ty:expr, $node:expr) => {
        anyhow::format_err!(
            "Unexpected attribute value type for boolean property: \
             expected {} but got {:?}, node_id={:?}",
            $v,
            $ty,
            $node.node_id()
        )
    };
}

mod array;
mod binstr;
mod mint;
mod primitive;
mod rgb;
mod strict_primitive;

/// Returns `Ok(value_part)` if the value part has expected length.
fn check_attrs_len<'a>(
    node: &PropertyHandle<'a>,
    expected_len: usize,
    target_name: &str,
) -> Result<&'a [AttributeValue], anyhow::Error> {
    use std::cmp::Ordering;

    let value_part = node.value_part();
    let len = value_part.len();
    match len.cmp(&expected_len) {
        Ordering::Less => bail!(
            "Not enough node attributes for {} property: node_id={:?}, expected {} but got {}",
            target_name,
            node.node_id(),
            expected_len,
            len
        ),
        Ordering::Greater => bail!(
            "Too many node attributes for {} property: node_id={:?}, expected {} but got {}",
            target_name,
            node.node_id(),
            expected_len,
            len
        ),
        Ordering::Equal => {}
    }

    Ok(value_part)
}
