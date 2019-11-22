//! Material data.

use std::convert::{TryFrom, TryInto};

use anyhow::{bail, Error};

use crate::v7400::object::property::{loaders::BorrowedStringLoader, LoadProperty, PropertyHandle};

/// Shading model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShadingModel {
    /// Unknown.
    Unknown,
    /// Lambert.
    Lambert,
    /// Phong.
    Phong,
}

impl TryFrom<&str> for ShadingModel {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Unknown" => Ok(ShadingModel::Unknown),
            "Lambert" => Ok(ShadingModel::Lambert),
            "Phong" => Ok(ShadingModel::Phong),
            s => bail!("Unexpected `ShadingModel` value: {:?}", s),
        }
    }
}

impl std::str::FromStr for ShadingModel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.try_into()
    }
}

/// `ShadingModel` property loader.
#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct ShadingModelLoader;

impl<'a> LoadProperty<'a> for ShadingModelLoader {
    type Value = ShadingModel;
    type Error = Error;

    fn expecting(&self) -> String {
        "string value as shading model".into()
    }

    fn load(self, node: &PropertyHandle<'a>) -> Result<Self::Value, Self::Error> {
        node.load_value(BorrowedStringLoader::new())
            .and_then(str::parse)
    }
}
