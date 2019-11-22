//! Primitive types.

use std::convert::TryFrom;

use anyhow::{bail, Error};

use crate::v7400::object::property::{loaders::PrimitiveLoader, LoadProperty, PropertyHandle};

/// Texture wrap mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WrapMode {
    /// Repeat.
    Repeat,
    /// Clamp to edge.
    Clamp,
}

impl TryFrom<i32> for WrapMode {
    type Error = Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(WrapMode::Repeat),
            1 => Ok(WrapMode::Clamp),
            v => bail!("Unexpected `WrapMode` value: {:?}", v),
        }
    }
}

/// `WrapMode` property loader.
#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct WrapModeLoader;

impl<'a> LoadProperty<'a> for WrapModeLoader {
    type Value = WrapMode;
    type Error = Error;

    fn expecting(&self) -> String {
        "`i32` value as wrap mode".into()
    }

    fn load(self, node: &PropertyHandle<'a>) -> Result<Self::Value, Self::Error> {
        if node.data_type()? != "enum" {
            bail!(
                "Unexpected data type: expected \"enum\", but got {:?}",
                node.data_type()
            );
        }
        node.load_value(PrimitiveLoader::<i32>::new())
            .and_then(TryFrom::try_from)
    }
}

/// Texture blend mode.
///
/// See
/// <http://help.autodesk.com/cloudhelp/2019/ENU/FBX-Developer-Help/cpp_ref/class_fbx_texture.html#ae712bb955e55f00dc24eb98c3686dd5a>.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlendMode {
    /// Transparent depending on alpha settings.
    Translucent,
    /// Additive.
    Additive,
    /// Multiply.
    Modulate,
    /// Multiply 2.
    Modulate2,
    /// Opaque.
    Over,
}

impl TryFrom<i32> for BlendMode {
    type Error = Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(BlendMode::Translucent),
            1 => Ok(BlendMode::Additive),
            2 => Ok(BlendMode::Modulate),
            3 => Ok(BlendMode::Modulate2),
            4 => Ok(BlendMode::Over),
            v => bail!("Unexpected `BlendMode` value: {:?}", v),
        }
    }
}

/// `BlendMode` property loader.
#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct BlendModeLoader;

impl<'a> LoadProperty<'a> for BlendModeLoader {
    type Value = BlendMode;
    type Error = Error;

    fn expecting(&self) -> String {
        "`i32` value as blend mode".into()
    }

    fn load(self, node: &PropertyHandle<'a>) -> Result<Self::Value, Self::Error> {
        if node.data_type()? != "enum" {
            bail!(
                "Unexpected data type: expected \"enum\", but got {:?}",
                node.data_type()
            );
        }
        node.load_value(PrimitiveLoader::<i32>::new())
            .and_then(TryFrom::try_from)
    }
}
