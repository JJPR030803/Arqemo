//! Two-phase theme validation pipeline.
//!
//! Validation happens in two steps, each with its own error type:
//!
//! 1. **File validation** ([`file::validate_file`]) — checks that a path points to
//!    a readable `.toml` file and deserializes it into a [`ThemeConfig`](crate::schema::ThemeConfig).
//!    Produces [`FileError`](errors::FileError) on failure.
//!
//! 2. **Semantic validation** ([`semantic::validate_semantic`]) — checks rules that
//!    serde cannot express: wallpaper mode constraints, empty strings, color hex format.
//!    Produces [`SemanticError`](errors::SemanticError) on failure.
//!
//! Both phases return [`ValidationError`](errors::ValidationError), which wraps
//! either a `FileError` or `SemanticError`.
//!
//! # Typical usage
//!
//! ```rust,no_run
//! use std::path::Path;
//! use arqemo_core::validate::file::validate_file;
//! use arqemo_core::validate::semantic::validate_semantic;
//!
//! let path = Path::new("/home/user/.config/arqemo/themes/brutalist/theme.toml");
//! let config = validate_file(path)?;      // phase 1: file → ThemeConfig
//! validate_semantic(&config)?;             // phase 2: ThemeConfig → semantic checks
//! // config is now safe to use
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod errors;
pub mod file;
mod helpers;
pub mod semantic;
