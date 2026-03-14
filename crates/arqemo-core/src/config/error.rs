//! Error types for configuration discovery and theme registry operations.
//!
//! Every error variant includes enough context for the user to fix the problem.
//! Several variants include `hint:` messages suggesting `arqemo init`.

use std::path::PathBuf;

use thiserror::Error;

/// Errors that can occur during config directory resolution or theme scanning.
///
/// Used by [`ConfigRoot::locate()`](super::root::ConfigRoot::locate) and
/// [`ThemeRegistry::scan()`](super::registry::ThemeRegistry::scan).
#[derive(Debug, Error)]
pub enum ConfigError {
    /// XDG config directory could not be determined.
    /// Typically means `$HOME` is unset on this system.
    #[error("could not determine config directory (XDG_CONFIG_HOME not set?)")]
    NoConfigDir,

    /// XDG cache directory could not be determined.
    #[error("could not determine cache directory")]
    NoCacheDir,

    /// `~/.config/arqemo/` does not exist.
    /// The user needs to run `arqemo init` or create it manually.
    #[error("arqemo config not found at '{0}'\n  hint: run `arqemo init`")]
    RootNotFound(PathBuf),

    /// `~/.config/arqemo/themes/` does not exist inside the config root.
    #[error("themes directory not found at '{0}'\n  hint: run `arqemo init`")]
    MissingThemesDir(PathBuf),

    /// The themes directory exists but cannot be read (permission error, etc.).
    #[error("cannot read themes directory '{path}': {source}")]
    ThemesDirUnreadable {
        /// The path that could not be read.
        path: PathBuf,
        /// The underlying I/O error.
        source: std::io::Error,
    },

    /// The themes directory is readable but contains zero valid theme directories.
    /// A valid theme directory contains a `theme.toml` file.
    #[error("no themes found in '{0}'")]
    NoThemesFound(PathBuf),

    /// The requested theme name does not match any scanned theme directory.
    /// The `available` field lists all themes that were found during scanning.
    #[error("theme '{name}' not found\n  available: {}", available.join(", "))]
    ThemeNotFound {
        /// The name the user requested.
        name: String,
        /// Sorted list of theme names that do exist.
        available: Vec<String>,
    },
}
