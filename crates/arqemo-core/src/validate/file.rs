use crate::schema::ThemeConfig;
use crate::validate::errors::{FileError, ValidationError};
use std::path::Path;

/// Validate that a path points to a readable, parseable theme.toml
/// and return the parsed [`ThemeConfig`].
///
/// Runs checks in order: exists → is file → `.toml` extension → not empty → parses.
/// Stops at the first failure. On success, returns the deserialized config.
///
/// This is phase 1 of validation. Follow with
/// [`validate_semantic()`](super::semantic::validate_semantic) for phase 2.
///
/// # Errors
///
/// Returns a [`ValidationError::File`](super::errors::ValidationError::File)
/// variant describing what went wrong. See [`FileError`](super::errors::FileError)
/// for the full list of variants.
///
/// # Examples
///
/// ```rust,no_run
/// use std::path::Path;
/// use arqemo_core::validate::file::validate_file;
///
/// let config = validate_file(Path::new("themes/brutalist/theme.toml"))?;
/// println!("theme name: {}", config.meta.name);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn validate_file(path: &Path) -> Result<ThemeConfig, ValidationError> {
    validate_file_exists(path)?;
    validate_is_file(path)?;
    validate_extension(path)?;
    validate_file_is_not_empty(path)?;
    validate_parsing(path)
}

fn validate_extension(path: &Path) -> Result<(), ValidationError> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("none");
    if ext != "toml" {
        return Err(FileError::WrongExtension(ext.to_string()).into());
    }
    Ok(())
}

fn validate_is_file(path: &Path) -> Result<(), ValidationError> {
    if !path.is_file() {
        return Err(FileError::PathIsNotFile(path.to_path_buf()).into());
    }
    Ok(())
}

fn validate_file_exists(path: &Path) -> Result<(), ValidationError> {
    if !path.exists() {
        return Err(FileError::PathDoesNotExist(path.to_path_buf()).into());
    }
    Ok(())
}

fn validate_file_is_not_empty(path: &Path) -> Result<(), ValidationError> {
    let meta = std::fs::metadata(path).map_err(FileError::OpenFileError)?;
    if meta.len() == 0 {
        return Err(FileError::FileIsEmpty(path.to_path_buf()).into());
    }
    Ok(())
}

fn validate_parsing(path: &Path) -> Result<ThemeConfig, ValidationError> {
    let content = std::fs::read_to_string(path).map_err(FileError::ReadFileError)?;
    toml::from_str(&content).map_err(|e| FileError::ParseError(e).into())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;
    use crate::schema::WallpaperMode;
    use std::io::Write;

    fn minimal_theme_toml() -> &'static str {
        r##"[meta]
name        = "test"
version     = "0.1"
description = "Unit test fixture."
tags        = ["test"]
renderer    = "none"
widgets     = "none"

[typography]
font_mono = "Iosevka"
font_size = 13

[colors]
bg             = "#0a0a0a"
fg             = "#e0e0e0"
black          = "#0a0a0a"
red            = "#ff5555"
green          = "#55ff55"
yellow         = "#f1c40f"
blue           = "#6699cc"
magenta        = "#cc66ff"
cyan           = "#66ffcc"
white          = "#e0e0e0"
bright_black   = "#333333"
bright_red     = "#ff7777"
bright_green   = "#77ff77"
bright_yellow  = "#f5d84f"
bright_blue    = "#88bbee"
bright_magenta = "#dd88ff"
bright_cyan    = "#88ffdd"
bright_white   = "#ffffff"
accent         = "#ffffff"
surface0       = "#111111"
surface1       = "#1a1a1a"
surface2       = "#242424"
muted          = "#555555"

[hyprland]
border_size = 1
gaps_in     = 4
gaps_out    = 8
rounding    = 0
blur        = false

[wallpaper]
mode  = "solid"
color = "#0a0a0a"
"##
    }

    #[test]
    fn nonexistent_path_returns_error() {
        let result = validate_file(Path::new("/tmp/arqemo_does_not_exist.toml"));
        let err = result.unwrap_err();
        assert!(
            matches!(err, ValidationError::File(FileError::PathDoesNotExist(_))),
            "expected PathDoesNotExist, got: {err:?}"
        );
    }

    #[test]
    fn directory_path_returns_not_file() {
        let dir = tempfile::tempdir().unwrap();
        let result = validate_file(dir.path());
        let err = result.unwrap_err();
        assert!(
            matches!(err, ValidationError::File(FileError::PathIsNotFile(_))),
            "expected PathIsNotFile, got: {err:?}"
        );
    }

    #[test]
    fn wrong_extension_returns_error() {
        let mut f = tempfile::Builder::new().suffix(".json").tempfile().unwrap();
        write!(f, "content").unwrap();
        let result = validate_file(f.path());
        let err = result.unwrap_err();
        assert!(
            matches!(err, ValidationError::File(FileError::WrongExtension(ref ext)) if ext == "json"),
            "expected WrongExtension(\"json\"), got: {err:?}"
        );
    }

    #[test]
    fn no_extension_returns_error() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("noext");
        std::fs::write(&path, "content").unwrap();
        let result = validate_file(&path);
        let err = result.unwrap_err();
        assert!(
            matches!(err, ValidationError::File(FileError::WrongExtension(ref ext)) if ext == "none"),
            "expected WrongExtension(\"none\"), got: {err:?}"
        );
    }

    #[test]
    fn empty_file_returns_error() {
        let f = tempfile::Builder::new().suffix(".toml").tempfile().unwrap();
        let result = validate_file(f.path());
        let err = result.unwrap_err();
        assert!(
            matches!(err, ValidationError::File(FileError::FileIsEmpty(_))),
            "expected FileIsEmpty, got: {err:?}"
        );
    }

    #[test]
    fn invalid_toml_returns_parse_error() {
        let mut f = tempfile::Builder::new().suffix(".toml").tempfile().unwrap();
        write!(f, "not valid toml [[").unwrap();
        let result = validate_file(f.path());
        let err = result.unwrap_err();
        assert!(
            matches!(err, ValidationError::File(FileError::ParseError(_))),
            "expected ParseError, got: {err:?}"
        );
    }

    #[test]
    fn valid_theme_parses_cleanly() {
        let mut f = tempfile::Builder::new().suffix(".toml").tempfile().unwrap();
        write!(f, "{}", minimal_theme_toml()).unwrap();
        let config = validate_file(f.path()).unwrap();
        assert_eq!(config.meta.name, "test");
        assert_eq!(config.colors.accent, "#ffffff");
        assert_eq!(config.wallpaper.mode, WallpaperMode::Solid);
    }
}
