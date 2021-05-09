//! Object property loaders.

mod array;
mod binstr;
mod primitive;

pub use self::array::{F32ArrayLoader, F64ArrayLoader, FloatArrayLoader};
pub use self::binstr::{
    BorrowedBinaryLoader, BorrowedStringLoader, OwnedBinaryLoader, OwnedStringLoader,
};
pub use self::primitive::{PrimitiveLoader, StrictPrimitiveLoader};
