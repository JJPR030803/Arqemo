//! Runtime configuration discovery and theme registry.
//!
//! This module resolves XDG directory paths (`~/.config/arqemo/`, `~/.cache/arqemo/`)
//! and scans the themes directory to build a registry of available themes.
//!
//! # Typical usage
//!
//! ```rust,no_run
//! use arqemo_core::config::root::ConfigRoot;
//! use arqemo_core::config::registry::ThemeRegistry;
//!
//! let root = ConfigRoot::locate()?;
//! let registry = ThemeRegistry::scan(&root)?;
//! let theme_dir = registry.theme_path("brutalist")?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Module structure
//!
//! - [`root`] — XDG path resolution and existence checks
//! - [`registry`] — theme directory scanning and name lookup
//! - [`error`] — typed errors for all config operations

pub mod error;
pub mod registry;
pub mod root;
