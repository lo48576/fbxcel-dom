//! Document metadata.

mod creation_timestamp;

use anyhow::anyhow;
use fbxcel::low::v7400::AttributeValue as A;
use fbxcel::tree::v7400::{NodeHandle, NodeId};

use crate::v7400::properties::{PropertiesNodeHandle, PropertiesNodeId};
use crate::v7400::property::loaders::BorrowedStringLoader;
use crate::v7400::{Document, Error, Result};

pub use self::creation_timestamp::{CreationTimestamp, RawCreationTimestamp};

/// The node name of the /FBXHeaderExtension node.
const NODENAME_FBX_HEADER_EXTENSION: &str = "FBXHeaderExtension";

/// Document metadata handle.
#[derive(Debug, Clone)]
pub struct DocumentMeta<'a> {
    /// Document.
    doc: &'a Document,
    /// Node ID of the `FBXHeaderExtension` node.
    // It is unlikely that `FBXHeaderExtension` does not exist, but the library
    // should be able to handle such broken documents.
    fbx_header_ext: Option<NodeId>,
    /// Properties node ID of the global info.
    global_props: Option<PropertiesNodeId>,
}

impl<'a> DocumentMeta<'a> {
    /// Creates a new `DocumentMeta` for the given document.
    #[inline]
    #[must_use]
    pub(super) fn new(doc: &'a Document) -> Self {
        let fbx_header_ext_node = doc
            .root_node()
            .first_child_by_name(NODENAME_FBX_HEADER_EXTENSION);
        let fbx_header_ext = fbx_header_ext_node.map(|node| node.node_id());

        let global_info =
            fbx_header_ext_node.and_then(|node| node.first_child_by_name("SceneInfo"));
        let global_props = global_info
            .and_then(|node| node.first_child_by_name("Properties70"))
            .map(|node| PropertiesNodeId::new(node.node_id()));

        Self {
            doc,
            fbx_header_ext,
            global_props,
        }
    }

    /// Returns the `FBXHeaderExtension` node.
    // The document without `FBXHeaderExtension` can be considered broken, so
    // returning `Result` here instead of `Option`.
    #[inline]
    fn fbx_header_ext(&self) -> Result<NodeHandle<'a>> {
        self.fbx_header_ext
            .map(|id| id.to_handle(self.doc.tree()))
            .ok_or_else(|| error!("`FBXHeaderExtension` node is expected to exist but not found"))
    }

    /// Returns the global properties node handle.
    fn global_props(&self) -> Result<PropertiesNodeHandle<'a>> {
        let global_props = self
            .global_props
            .ok_or_else(|| error!("global properties not found"))?;
        Ok(PropertiesNodeHandle::new(global_props, self.doc))
    }

    /// Returns the creation timestamp if they are valid.
    ///
    /// Time offset is local timezone of the machine where the file is created.
    ///
    /// # Failures
    ///
    /// Returns an error if the timestamp is not found or the value is invalid.
    pub fn creation_timestamp(&self) -> Result<Option<CreationTimestamp>> {
        match self.creation_timestamp_raw() {
            Ok(Some(v)) => CreationTimestamp::from_raw(v).map(Some),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Returns the raw creation timestamp.
    ///
    /// "Raw" means that the value might be invalid since it is not strictly
    /// validated as a datetime.
    ///
    /// Time offset is local timezone of the machine where the file is created.
    ///
    /// # Failures
    ///
    /// Returns an error if the timestamp is not found or the value is clearly invalid.
    pub fn creation_timestamp_raw(&self) -> Result<Option<RawCreationTimestamp>> {
        /// The node name of the /FBXHeaderExtension/CreationTimeStamp node.
        const NODENAME_CREATION_TIMESTAMP: &str = "CreationTimeStamp";

        let ts_node = self
            .fbx_header_ext()?
            .first_child_by_name(NODENAME_CREATION_TIMESTAMP);
        let ts_node = match ts_node {
            Some(v) => v,
            None => return Ok(None),
        };

        let version = ts_node
            .first_child_by_name("Version")
            .and_then(|node| node.attributes().get(0));
        if version.and_then(|v| v.get_i32()) != Some(1000) {
            log::warn!("unknown creation timestamp version {:?}", version);
        }

        let year = ts_node
            .first_child_by_name("Year")
            .and_then(get_i32_first)
            .ok_or_else(|| Error::new(anyhow!("failed to get creation year")))?;
        let year: u16 = if (0..=9999).contains(&year) {
            year as u16
        } else {
            return Err(Error::new(anyhow!(
                "year ({}) is out of supported range",
                year
            )));
        };
        let month1 = ts_node
            .first_child_by_name("Month")
            .and_then(get_i32_first)
            .ok_or_else(|| Error::new(anyhow!("failed to get creation month")))?;
        let month1: u8 = if (1..=12).contains(&month1) {
            month1 as u8
        } else {
            return Err(Error::new(anyhow!(
                "month1 ({}) is out of supported range",
                month1
            )));
        };
        let mday1 = ts_node
            .first_child_by_name("Day")
            .and_then(get_i32_first)
            .ok_or_else(|| Error::new(anyhow!("failed to get creation mday")))?;
        let mday1: u8 = if (1..=31).contains(&mday1) {
            mday1 as u8
        } else {
            return Err(Error::new(anyhow!(
                "mday1 ({}) is out of supported range",
                mday1
            )));
        };
        let hour = ts_node
            .first_child_by_name("Hour")
            .and_then(get_i32_first)
            .ok_or_else(|| Error::new(anyhow!("failed to get creation hour")))?;
        let hour: u8 = if (0..=23).contains(&hour) {
            hour as u8
        } else {
            return Err(Error::new(anyhow!(
                "hour ({}) is out of supported range",
                hour
            )));
        };
        let minute = ts_node
            .first_child_by_name("Minute")
            .and_then(get_i32_first)
            .ok_or_else(|| Error::new(anyhow!("failed to get creation minute")))?;
        let minute: u8 = if (0..=59).contains(&minute) {
            minute as u8
        } else {
            return Err(Error::new(anyhow!(
                "minute ({}) is out of supported range",
                minute
            )));
        };
        let second = ts_node
            .first_child_by_name("Second")
            .and_then(get_i32_first)
            .ok_or_else(|| Error::new(anyhow!("failed to get creation second")))?;
        let second: u8 = if (0..=60).contains(&second) {
            second as u8
        } else {
            return Err(Error::new(anyhow!(
                "second ({}) is out of supported range",
                second
            )));
        };
        let millisecond = ts_node
            .first_child_by_name("Millisecond")
            .and_then(get_i32_first)
            .ok_or_else(|| Error::new(anyhow!("failed to get creation millisecond")))?;
        let millisecond: u16 = if (0..=1999).contains(&millisecond) {
            millisecond as u16
        } else {
            return Err(Error::new(anyhow!(
                "millisecond ({}) is out of supported range",
                millisecond
            )));
        };

        Ok(Some(RawCreationTimestamp::new(
            year,
            month1,
            mday1,
            hour,
            minute,
            second,
            millisecond,
        )))
    }

    /// Returns the "creator" of the document.
    ///
    /// Note that the "creator" seems to be an application or library name,
    /// rather than a person or an organization.
    pub fn creator(&self) -> Result<Option<&'a str>> {
        /// The node name of the /Creator node.
        const NODENAME_CREATOR: &str = "Creator";

        let creator_node = self.doc.root_node().first_child_by_name(NODENAME_CREATOR);
        let creator_node = match creator_node {
            Some(v) => v,
            None => return Ok(None),
        };

        get_str_first(creator_node)
            .map(Some)
            .ok_or_else(|| Error::new(anyhow!("failed to get creator of the document")))
    }

    /// Returns the "original filename".
    ///
    /// This is not documented officially, but it seems to be a path where the
    /// file is (first) created.
    ///
    /// The path separator is a slash, even for a document created on Windows.
    pub fn original_filename(&self) -> Result<Option<&'a str>> {
        match self.global_props()?.get("Original|FileName") {
            Some(prop) => prop.value(BorrowedStringLoader::new()).map(Some),
            None => Ok(None),
        }
    }

    /// Returns the "application vendor" at the time the document is created.
    pub fn original_application_vendor(&self) -> Result<Option<&'a str>> {
        match self.global_props()?.get("Original|ApplicationVendor") {
            Some(prop) => prop.value(BorrowedStringLoader::new()).map(Some),
            None => Ok(None),
        }
    }

    /// Returns the "application name" at the time the document is created.
    pub fn original_application_name(&self) -> Result<Option<&'a str>> {
        match self.global_props()?.get("Original|ApplicationName") {
            Some(prop) => prop.value(BorrowedStringLoader::new()).map(Some),
            None => Ok(None),
        }
    }

    /// Returns the "application version" at the time the document is created.
    pub fn original_application_version(&self) -> Result<Option<&'a str>> {
        match self.global_props()?.get("Original|ApplicationVersion") {
            Some(prop) => prop.value(BorrowedStringLoader::new()).map(Some),
            None => Ok(None),
        }
    }

    /// Returns the "application vendor" at the time the document is last saved.
    pub fn last_saved_application_vendor(&self) -> Result<Option<&'a str>> {
        match self.global_props()?.get("LastSaved|ApplicationVendor") {
            Some(prop) => prop.value(BorrowedStringLoader::new()).map(Some),
            None => Ok(None),
        }
    }

    /// Returns the "application name" at the time the document is last saved.
    pub fn last_saved_application_name(&self) -> Result<Option<&'a str>> {
        match self.global_props()?.get("LastSaved|ApplicationName") {
            Some(prop) => prop.value(BorrowedStringLoader::new()).map(Some),
            None => Ok(None),
        }
    }

    /// Returns the "application version" at the time the document is last saved.
    pub fn last_saved_application_version(&self) -> Result<Option<&'a str>> {
        match self.global_props()?.get("LastSaved|ApplicationVersion") {
            Some(prop) => prop.value(BorrowedStringLoader::new()).map(Some),
            None => Ok(None),
        }
    }

    /// Returns the file ID.
    ///
    /// This seems to be always 16 bytes binary, but the official specification
    /// is not published.
    pub fn file_id(&self) -> Result<Option<&'a [u8]>> {
        let node = match self.doc.root_node().first_child_by_name("FileId") {
            Some(v) => v,
            None => return Ok(None),
        };
        match node.attributes() {
            [A::Binary(v)] => Ok(Some(v)),
            [v] => Err(error!(
                "expected a binary attribute but got {:?}",
                v.type_()
            )),
            v => Err(error!(
                "expected single binary attribute but got {} attributes",
                v.len()
            )),
        }
    }
}

/// Returns the `i32` value at the first attribute, if available.
#[must_use]
fn get_i32_first(node: NodeHandle<'_>) -> Option<i32> {
    node.attributes().get(0).and_then(|v| v.get_i32())
}

/// Returns the `&str` value at the first attribute, if available.
#[must_use]
fn get_str_first(node: NodeHandle<'_>) -> Option<&'_ str> {
    node.attributes().get(0).and_then(|v| v.get_string())
}
