//! XDG-based config root resolution.
//!
//! Resolves `~/.config/arqemo/` and `~/.cache/arqemo/` from XDG base directories
//! and validates that the config root and themes directory exist on disk.

use std::path::PathBuf;

use crate::config::error::ConfigError;

/// Resolved runtime directory paths for arqemo.
///
/// Created by [`ConfigRoot::locate()`], which reads XDG environment variables
/// and checks that the expected directories exist.
///
/// # Fields
///
/// - `base` — `~/.config/arqemo/` (or equivalent XDG path)
/// - `themes_dir` — `~/.config/arqemo/themes/`
/// - `cache_dir` — `~/.cache/arqemo/`
///
/// # Examples
///
/// ```rust,no_run
/// use arqemo_core::config::root::ConfigRoot;
///
/// let root = ConfigRoot::locate()?;
/// println!("config:  {}", root.base.display());
/// println!("themes:  {}", root.themes_dir.display());
/// println!("cache:   {}", root.cache_dir.display());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug)]
pub struct ConfigRoot {
    /// The arqemo config root, e.g. `~/.config/arqemo/`.
    pub base: PathBuf,
    /// The themes directory, e.g. `~/.config/arqemo/themes/`.
    pub themes_dir: PathBuf,
    /// The cache directory, e.g. `~/.cache/arqemo/`.
    pub cache_dir: PathBuf,
}

impl ConfigRoot {
    /// Ensure the config root and all required subdirs exist, creating them if needed.
    ///
    /// Idempotent — safe to call even if the dirs already exist.
    /// This is the entry point called by `arqemo init`.
    ///
    /// # Errors
    ///
    /// - [`ConfigError::NoConfigDir`] — XDG config dir cannot be determined
    /// - [`ConfigError::NoCacheDir`]  — XDG cache dir cannot be determined
    /// - [`ConfigError::CreateFailed`] — a directory could not be created
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use arqemo_core::config::root::ConfigRoot;
    ///
    /// let root = ConfigRoot::ensure()?;
    /// assert!(root.themes_dir.exists());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn ensure() -> Result<Self, ConfigError> {
        let base = dirs::config_dir()
            .ok_or(ConfigError::NoConfigDir)?
            .join("arqemo");

        let cache_dir = dirs::cache_dir()
            .ok_or(ConfigError::NoCacheDir)?
            .join("arqemo");

        let themes_dir = base.join("themes");

        for dir in [&base, &themes_dir] {
            std::fs::create_dir_all(dir).map_err(|e| ConfigError::CreateFailed {
                path: dir.clone(),
                source: e,
            })?;
        }

        Ok(ConfigRoot {
            base,
            themes_dir,
            cache_dir,
        })
    }

    /// Locate the arqemo config root by resolving XDG directories.
    ///
    /// Checks that `~/.config/arqemo/` and `~/.config/arqemo/themes/` exist.
    /// The cache directory is resolved but not required to exist yet
    /// (it is created on first write by [`cache::write()`](crate::cache::write)).
    ///
    /// # Errors
    ///
    /// - [`ConfigError::NoConfigDir`] — XDG config dir cannot be determined
    /// - [`ConfigError::NoCacheDir`] — XDG cache dir cannot be determined
    /// - [`ConfigError::RootNotFound`] — `~/.config/arqemo/` does not exist
    /// - [`ConfigError::MissingThemesDir`] — `themes/` subdirectory missing
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use arqemo_core::config::root::ConfigRoot;
    ///
    /// let root = ConfigRoot::locate()?;
    /// assert!(root.themes_dir.exists());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn locate() -> Result<Self, ConfigError> {
        let base = dirs::config_dir()
            .ok_or(ConfigError::NoConfigDir)?
            .join("arqemo");

        let cache_dir = dirs::cache_dir()
            .ok_or(ConfigError::NoCacheDir)?
            .join("arqemo");

        if !base.exists() {
            return Err(ConfigError::RootNotFound(base));
        }

        let themes_dir = base.join("themes");
        if !themes_dir.exists() {
            return Err(ConfigError::MissingThemesDir(themes_dir));
        }

        Ok(ConfigRoot {
            base,
            themes_dir,
            cache_dir,
        })
    }
}
