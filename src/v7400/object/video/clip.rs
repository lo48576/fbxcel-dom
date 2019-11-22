//! `Video` object (clip).

use anyhow::{format_err, Error};

use crate::v7400::object::video::VideoHandle;

define_object_subtype! {
    /// `Video` node handle (clip).
    ClipHandle: VideoHandle
}

impl<'a> ClipHandle<'a> {
    /// Returns relative filename.
    ///
    /// Note that this returns raw value, and the path separator might be a
    /// slash or a backslash.
    pub fn relative_filename(&self) -> Result<&'a str, Error> {
        // "n" of "Filename" is lower.
        self.node()
            .children_by_name("RelativeFilename")
            .next()
            .ok_or_else(|| {
                format_err!("`RelativeFilename` child node not found for video clip object")
            })?
            .attributes()
            .get(0)
            .ok_or_else(|| format_err!("`RelativeFilename` node has no attributes"))?
            .get_string_or_type()
            .map_err(|ty| {
                format_err!(
                    "Expected string as `RelativeFilename` value, but got {:?}",
                    ty
                )
            })
    }
    /// Returns content.
    pub fn content(&self) -> Option<&'a [u8]> {
        self.node()
            .children_by_name("Content")
            .next()?
            .attributes()
            .get(0)?
            .get_binary()
    }
}
