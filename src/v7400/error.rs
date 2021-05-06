//! Data access error.

use std::fmt;

use thiserror::Error as ThisError;

/// Result of a data access.
pub type Result<T> = std::result::Result<T, Error>;

/// Constructs `Error::new(anyhow!(...))`.
macro_rules! error {
    ($msg:literal $(,)?) => {
        crate::v7400::Error::new(anyhow::anyhow!($msg))
    };
    ($err:expr $(,)?) => {
        crate::v7400::Error::new(anyhow::anyhow!($err))
    };
    ($fmt:expr, $($arg:tt)*) => {
        crate::v7400::Error::new(anyhow::anyhow!($fmt, $($arg)*))
    };
}

/// Data access error.
#[derive(Debug, ThisError)]
#[error(transparent)]
pub struct Error {
    /// Inner error.
    inner: anyhow::Error,
}

impl Error {
    /// Creates a new error.
    #[must_use]
    pub(super) fn new(e: impl Into<anyhow::Error>) -> Self {
        Self { inner: e.into() }
    }

    /// Adds the given context to the error.
    #[must_use]
    #[allow(dead_code)] // TODO: Remove when this attr becomes unnecessary.
    pub(super) fn context<C>(self, context: C) -> Self
    where
        C: fmt::Display + Send + Sync + 'static,
    {
        Self {
            inner: self.inner.context(context),
        }
    }

    /// Adds a context returned by the given funciton to the error.
    #[must_use]
    #[allow(dead_code)] // TODO: Remove when this attr becomes unnecessary.
    pub(super) fn with_context<C, F>(self, f: F) -> Self
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        Self {
            inner: self.inner.context(f()),
        }
    }
}
