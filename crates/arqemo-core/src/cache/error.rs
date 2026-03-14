//! Error types for cache directory operations.
//!
//! Separate from [`ConfigError`](crate::config::error::ConfigError) — config
//! errors are about finding source files; cache errors are about writing
//! rendered output. The domains don't overlap.

use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during cache directory validation or creation.
///
/// Used by [`CacheLayout::check()`](super::CacheLayout::check) and
/// [`CacheLayout::ensure()`](super::CacheLayout::ensure).
#[derive(Debug, Error)]
pub enum CacheError {
    /// `~/.cache/arqemo/` does not exist and was not created.
    /// Surfaces when `check()` is called and the root is absent.
    #[error("arqemo cache not found at '{0}'\n  hint: run `arqemo init`")]
    RootNotFound(PathBuf),

    /// A required subdirectory is missing inside the cache root.
    #[error("cache subdirectory missing: '{0}'\n  hint: run `arqemo init`")]
    SubdirMissing(PathBuf),

    /// The cache root exists but cannot be written to.
    #[error("cache directory '{path}' is not writable: {source}")]
    NotWritable {
        path: PathBuf,
        source: std::io::Error,
    },

    /// A directory could not be created during `ensure()`.
    #[error("failed to create cache directory '{path}': {source}")]
    CreateFailed {
        path: PathBuf,
        source: std::io::Error,
    },
}