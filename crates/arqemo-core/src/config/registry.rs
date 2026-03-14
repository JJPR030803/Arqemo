//! Theme registry — scans the themes directory and provides name-based lookup.
//!
//! A theme is a subdirectory of `~/.config/arqemo/themes/` that contains
//! a `theme.toml` file. Directories without `theme.toml` are skipped with
//! a warning to stderr.

use std::collections::HashMap;
use std::path::PathBuf;

use crate::config::error::ConfigError;
use crate::config::root::ConfigRoot;

/// An index of available themes, keyed by directory name.
///
/// Built by scanning the filesystem via [`ThemeRegistry::scan()`].
/// Use [`theme_path()`](ThemeRegistry::theme_path) to resolve a theme name
/// to its directory, then pass `<dir>/theme.toml` to
/// [`validate_file()`](crate::validate::file::validate_file) for parsing.
///
/// # Examples
///
/// ```rust,no_run
/// use arqemo_core::config::root::ConfigRoot;
/// use arqemo_core::config::registry::ThemeRegistry;
///
/// let root = ConfigRoot::locate()?;
/// let registry = ThemeRegistry::scan(&root)?;
///
/// for name in registry.available_names() {
///     println!("  {name}");
/// }
///
/// let brutalist_dir = registry.theme_path("brutalist")?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug)]
pub struct ThemeRegistry {
    entries: HashMap<String, PathBuf>,
}

impl ThemeRegistry {
    /// Scan the themes directory and build a registry of available themes.
    ///
    /// A valid entry is a subdirectory containing `theme.toml`.
    /// Directories without `theme.toml` are skipped (warning printed to stderr).
    ///
    /// # Errors
    ///
    /// - [`ConfigError::ThemesDirUnreadable`] — I/O error reading the themes directory
    /// - [`ConfigError::NoThemesFound`] — no valid theme directories found
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use arqemo_core::config::root::ConfigRoot;
    /// use arqemo_core::config::registry::ThemeRegistry;
    ///
    /// let root = ConfigRoot::locate()?;
    /// let registry = ThemeRegistry::scan(&root)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn scan(root: &ConfigRoot) -> Result<Self, ConfigError> {
        let mut entries = HashMap::new();

        let read_dir =
            std::fs::read_dir(&root.themes_dir).map_err(|e| ConfigError::ThemesDirUnreadable {
                path: root.themes_dir.clone(),
                source: e,
            })?;

        for entry in read_dir.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            let toml_path = path.join("theme.toml");
            if !toml_path.exists() {
                eprintln!("warn: '{}' has no theme.toml, skipping", path.display());
                continue;
            }

            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                entries.insert(name.to_string(), path);
            }
        }

        if entries.is_empty() {
            return Err(ConfigError::NoThemesFound(root.themes_dir.clone()));
        }

        Ok(ThemeRegistry { entries })
    }

    /// Look up a theme's directory path by name.
    ///
    /// Returns the full path to the theme directory (e.g. `~/.config/arqemo/themes/brutalist/`).
    /// To get the theme.toml path, join with `"theme.toml"`.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::ThemeNotFound`] if the name doesn't match any scanned theme.
    /// The error includes the list of available theme names.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use arqemo_core::config::root::ConfigRoot;
    /// # use arqemo_core::config::registry::ThemeRegistry;
    /// # let root = ConfigRoot::locate()?;
    /// # let registry = ThemeRegistry::scan(&root)?;
    /// let dir = registry.theme_path("brutalist")?;
    /// let toml = dir.join("theme.toml");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn theme_path(&self, name: &str) -> Result<&PathBuf, ConfigError> {
        self.entries
            .get(name)
            .ok_or_else(|| ConfigError::ThemeNotFound {
                name: name.to_string(),
                available: self.available_names(),
            })
    }

    /// Return a sorted list of all available theme names.
    ///
    /// Useful for display in `arqemo list` or in error messages.
    #[must_use]
    pub fn available_names(&self) -> Vec<String> {
        let mut names: Vec<_> = self.entries.keys().cloned().collect();
        names.sort();
        names
    }
}
