//! Error types for the two-phase validation pipeline.
//!
//! [`ValidationError`] is the top-level error returned by both
//! [`validate_file()`](super::file::validate_file) and
//! [`validate_semantic()`](super::semantic::validate_semantic).
//! It wraps either a [`FileError`] (phase 1) or a [`SemanticError`] (phase 2).

use thiserror::Error;

/// Top-level validation error wrapping file or semantic errors.
///
/// Pattern-match on the inner variant to determine which phase failed:
///
/// ```rust,no_run
/// use arqemo_core::validate::errors::{ValidationError, FileError, SemanticError};
///
/// # fn example(err: ValidationError) {
/// match err {
///     ValidationError::File(e) => eprintln!("file problem: {e}"),
///     ValidationError::Semantic(e) => eprintln!("semantic problem: {e}"),
/// }
/// # }
/// ```
#[derive(Error, Debug)]
pub enum ValidationError {
    /// A file-level problem (path, read, parse).
    #[error(transparent)]
    File(#[from] FileError),

    /// A semantic rule violation (wallpaper mode, empty field, color format).
    #[error(transparent)]
    Semantic(#[from] SemanticError),
}

/// Errors from phase 1: file existence, readability, and TOML parsing.
///
/// Produced by [`validate_file()`](super::file::validate_file).
/// Each variant corresponds to one check in the validation pipeline.
#[derive(Error, Debug)]
pub enum FileError {
    /// The given path does not exist on disk.
    #[error("Path does not exist: {0}")]
    PathDoesNotExist(std::path::PathBuf),

    /// The path exists but points to a directory, not a file.
    #[error("Path is not a file: {0}")]
    PathIsNotFile(std::path::PathBuf),

    /// The file has an extension other than `.toml`.
    #[error("Wrong extension: {0}")]
    WrongExtension(String),

    /// The file exists and is a `.toml` file, but has zero bytes.
    #[error("File is empty: {0}")]
    FileIsEmpty(std::path::PathBuf),

    /// The file could not be opened (permission denied, etc.).
    #[error("failed to open file: {0}")]
    OpenFileError(std::io::Error),

    /// The file could not be read into a string (encoding error, I/O error).
    #[error("failed to read file: {0}")]
    ReadFileError(std::io::Error),

    /// The file contents are not valid TOML or do not match the
    /// [`ThemeConfig`](crate::schema::ThemeConfig) schema.
    #[error("failed to parse file: {0}")]
    ParseError(#[from] toml::de::Error),
}

/// Errors from phase 2: semantic rules that serde cannot express.
///
/// Produced by [`validate_semantic()`](super::semantic::validate_semantic).
/// Covers wallpaper mode constraints, empty required fields, and color format.
#[derive(Error, Debug)]
pub enum SemanticError {
    /// `[wallpaper] mode = "renderer"` but no `[renderer]` section exists.
    #[error("[wallpaper] mode = \"renderer\" requires a [renderer] section")]
    MissingRendererSection,

    /// `[wallpaper] mode = "image"` but `path` key is missing.
    #[error("[wallpaper] mode = \"image\" requires a path key")]
    MissingWallpaperPath,

    /// `[wallpaper] mode = "solid"` but `color` key is missing.
    #[error("[wallpaper] mode = \"solid\" requires a color key")]
    MissingWallpaperColor,

    /// `[wallpaper] mode = "glsl"` but `shader` key is missing.
    #[error("[wallpaper] mode = \"glsl\" requires a shader key")]
    MissingWallpaperShader,

    /// A wallpaper key is present that is forbidden for the current mode.
    /// See the validation table in `.claude/schema.md`.
    #[error("[wallpaper] forbidden key for mode \"{mode}\": {key}")]
    ForbiddenWallpaperKey {
        /// The wallpaper mode (e.g. `"image"`, `"solid"`).
        mode: String,
        /// The forbidden key name (e.g. `"color"`, `"shader"`).
        key: String,
    },

    /// The `wallpaper.mode` value is not one of: `image`, `solid`, `glsl`, `renderer`.
    #[error("unknown wallpaper mode: {0}")]
    UnknownWallpaperMode(String),

    /// A required string field is present but empty (`""`).
    /// Checked for meta, typography, wallpaper.mode, and all color fields.
    #[error("required field is empty: {section}.{field}")]
    EmptyField {
        /// The TOML section (e.g. `"meta"`, `"colors"`).
        section: String,
        /// The field name within that section.
        field: String,
    },

    /// A color field does not match the expected `#RRGGBB` hex format.
    #[error("invalid color format for {field}: \"{value}\" (expected #RRGGBB)")]
    InvalidColorFormat {
        /// The color field name (e.g. `"red"`, `"accent"`).
        field: String,
        /// The actual value found.
        value: String,
    },
}
