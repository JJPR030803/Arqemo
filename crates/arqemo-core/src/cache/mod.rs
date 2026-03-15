//! Cache directory layout — validation and creation.
//!
//! The cache (`~/.cache/arqemo/`) holds rendered output: template files,
//! per-theme scratch space, and anything the engine writes at runtime.
//! It is never the source of truth — deleting it is always safe.
//!
//! # Typical usage
//!
//! `init` calls [`CacheLayout::ensure()`] to create dirs on first run.
//! A future `doctor` command calls [`CacheLayout::check()`] to validate
//! without creating anything.
//!
//! ```rust,no_run
//! use std::path::Path;
//! use arqemo_core::cache::CacheLayout;
//!
//! // init path: create if missing
//! let layout = CacheLayout::ensure(Path::new("/home/user/.cache/arqemo"))?;
//!
//! // doctor path: validate only
//! let layout = CacheLayout::check(Path::new("/home/user/.cache/arqemo"))?;
//!
//! println!("rendered output: {}", layout.rendered.display());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Module structure
//!
//! - [`CacheLayout`] — resolved paths and entry points
//! - [`error`] — typed errors for all cache operations

pub mod error;

use std::path::{Path, PathBuf};

use anyhow::Result;
use error::CacheError;

/// Returns the path to the arqemo cache directory (`~/.cache/arqemo/`).
///
/// # Errors
///
/// Returns an error if the cache directory cannot be determined.
pub fn search_cache_dir() -> Result<PathBuf> {
    let base =
        dirs::cache_dir().ok_or_else(|| anyhow::anyhow!("could not determine cache directory"))?;
    Ok(base.join("arqemo"))
}

/// Resolved paths for the arqemo cache directory.
///
/// Created by [`CacheLayout::check()`] (validates, no writes) or
/// [`CacheLayout::ensure()`] (creates missing dirs).
///
/// # Fields
///
/// - `root`     — `~/.cache/arqemo/`
/// - `rendered` — `~/.cache/arqemo/rendered/`  (template output: hyprland.conf, colors.css, …)
/// - `themes`   — `~/.cache/arqemo/themes/`    (per-theme scratch space)
///
/// # Examples
///
/// ```rust,no_run
/// use std::path::Path;
/// use arqemo_core::cache::CacheLayout;
///
/// let layout = CacheLayout::ensure(Path::new("/home/user/.cache/arqemo"))?;
/// assert!(layout.rendered.exists());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug)]
pub struct CacheLayout {
    /// The arqemo cache root, e.g. `~/.cache/arqemo/`.
    pub root: PathBuf,
    /// Rendered template output, e.g. `~/.cache/arqemo/rendered/`.
    pub rendered: PathBuf,
    /// Per-theme scratch space, e.g. `~/.cache/arqemo/themes/`.
    pub themes: PathBuf,
}

impl CacheLayout {
    /// Validate that the cache directory and all required subdirs exist and are writable.
    ///
    /// Does **not** create anything. Use [`ensure()`](Self::ensure) in `init`.
    /// Use this in a future `arqemo doctor` command.
    ///
    /// # Errors
    ///
    /// - [`CacheError::RootNotFound`]  — root does not exist
    /// - [`CacheError::SubdirMissing`] — a required subdir is absent
    /// - [`CacheError::NotWritable`]   — root exists but cannot be written to
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use std::path::Path;
    /// use arqemo_core::cache::CacheLayout;
    ///
    /// let layout = CacheLayout::check(Path::new("/home/user/.cache/arqemo"))?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn check(root: &Path) -> Result<Self, CacheError> {
        if !root.exists() {
            return Err(CacheError::RootNotFound(root.to_path_buf()));
        }

        Self::assert_writable(root)?;

        let layout = Self::from_root(root);

        for subdir in layout.subdirs() {
            if !subdir.exists() {
                return Err(CacheError::SubdirMissing(subdir.to_path_buf()));
            }
        }

        Ok(layout)
    }

    /// Ensure the cache directory and all required subdirs exist, creating them if needed.
    ///
    /// Idempotent — safe to call even if the dirs already exist.
    /// This is the entry point called by `arqemo init`.
    ///
    /// # Errors
    ///
    /// - [`CacheError::NotWritable`]  — root exists but cannot be written to
    /// - [`CacheError::CreateFailed`] — a directory could not be created
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use std::path::Path;
    /// use arqemo_core::cache::CacheLayout;
    ///
    /// let layout = CacheLayout::ensure(Path::new("/home/user/.cache/arqemo"))?;
    /// assert!(layout.themes.exists());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn ensure(root: &Path) -> Result<Self, CacheError> {
        std::fs::create_dir_all(root).map_err(|e| CacheError::CreateFailed {
            path: root.to_path_buf(),
            source: e,
        })?;

        Self::assert_writable(root)?;

        let layout = Self::from_root(root);

        for subdir in layout.subdirs() {
            std::fs::create_dir_all(&subdir).map_err(|e| CacheError::CreateFailed {
                path: subdir.clone(),
                source: e,
            })?;
        }

        Ok(layout)
    }

    // --- private helpers ---

    fn from_root(root: &Path) -> Self {
        CacheLayout {
            root: root.to_path_buf(),
            rendered: root.join("rendered"),
            themes: root.join("themes"),
        }
    }

    /// The subdirs `check` and `ensure` both operate on.
    /// Adding a new subdir here is the only change needed to extend the layout.
    fn subdirs(&self) -> [&PathBuf; 2] {
        [&self.rendered, &self.themes]
    }

    /// Probe writability by attempting to create (and immediately remove) a temp file.
    fn assert_writable(root: &Path) -> Result<(), CacheError> {
        let probe = root.join(".arqemo_write_probe");
        std::fs::write(&probe, b"").map_err(|e| CacheError::NotWritable {
            path: root.to_path_buf(),
            source: e,
        })?;
        let _ = std::fs::remove_file(&probe);
        Ok(())
    }
}