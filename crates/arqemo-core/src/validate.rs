//! Semantic validation layer.
//! Runs after serde type validation. Checks conditional rules
//! that serde cannot express: mode-conditional wallpaper keys,
//! renderer requirement, missing color keys, path existence.

use crate::schema::ThemeConfig;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("[wallpaper] mode = \"renderer\" requires a [renderer] section")]
    MissingRendererSection,

    #[error("[wallpaper] mode = \"image\" requires a path key")]
    MissingWallpaperPath,

    #[error("[wallpaper] mode = \"solid\" requires a color key")]
    MissingWallpaperColor,

    #[error("[wallpaper] mode = \"glsl\" requires a shader key")]
    MissingWallpaperShader,

    #[error("[wallpaper] forbidden key for mode \"{mode}\": {key}")]
    ForbiddenWallpaperKey { mode: String, key: String },

    #[error("path does not exist: {0}")]
    PathNotFound(std::path::PathBuf),

    #[error("unknown wallpaper mode: {0}")]
    UnknownWallpaperMode(String),
}

/// Validate a theme configuration.
///
/// # Errors
///
/// Returns a `ValidationError` if the theme config violates semantic rules.
pub fn validate(config: &ThemeConfig) -> Result<(), ValidationError> {
    validate_wallpaper(config)?;
    Ok(())
}

fn validate_wallpaper(config: &ThemeConfig) -> Result<(), ValidationError> {
    let wp = &config.wallpaper;

    match wp.mode.as_str() {
        "image" => {
            if wp.path.is_none() {
                return Err(ValidationError::MissingWallpaperPath);
            }
            if wp.color.is_some() {
                return Err(ValidationError::ForbiddenWallpaperKey {
                    mode: "image".into(),
                    key: "color".into(),
                });
            }
            if wp.shader.is_some() {
                return Err(ValidationError::ForbiddenWallpaperKey {
                    mode: "image".into(),
                    key: "shader".into(),
                });
            }
        }
        "solid" => {
            if wp.color.is_none() {
                return Err(ValidationError::MissingWallpaperColor);
            }
        }
        "glsl" => {
            if wp.shader.is_none() {
                return Err(ValidationError::MissingWallpaperShader);
            }
        }
        "renderer" => {
            if config.renderer.is_none() {
                return Err(ValidationError::MissingRendererSection);
            }
        }
        other => return Err(ValidationError::UnknownWallpaperMode(other.into())),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    // Integration-style tests live in crates/arqemo-core/tests/
    // Unit tests for individual validation rules go here
    #[test]
    fn placeholder_validate_test() {
        // Replace with real cases in Phase 1
    }
}
