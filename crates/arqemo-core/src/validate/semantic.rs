//! Semantic validation layer.
//! Runs after serde type validation (file.rs). Checks conditional rules
//! that serde cannot express: mode-conditional wallpaper keys,
//! empty required strings, color hex format.

use crate::schema::ThemeConfig;
use crate::validate::errors::{SemanticError, ValidationError};

/// Validate semantic rules on a parsed [`ThemeConfig`].
///
/// Runs three groups of checks in order:
/// 1. **Wallpaper mode** — required/forbidden keys per mode (see `.claude/schema.md`)
/// 2. **Empty strings** — required fields must not be `""`
/// 3. **Color format** — all 23 color fields must match `#RRGGBB`
///
/// This is phase 2 of validation. Call after
/// [`validate_file()`](super::file::validate_file) has produced a `ThemeConfig`.
///
/// # Errors
///
/// Returns a [`ValidationError::Semantic`](super::errors::ValidationError::Semantic)
/// variant. See [`SemanticError`](super::errors::SemanticError) for the full list.
///
/// # Examples
///
/// ```rust,no_run
/// use std::path::Path;
/// use arqemo_core::validate::file::validate_file;
/// use arqemo_core::validate::semantic::validate_semantic;
///
/// let config = validate_file(Path::new("themes/brutalist/theme.toml"))?;
/// validate_semantic(&config)?;
/// // config is now fully validated
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn validate_semantic(config: &ThemeConfig) -> Result<(), ValidationError> {
    validate_wallpaper(config)?;
    validate_no_empty_strings(config)?;
    validate_color_format(config)?;
    Ok(())
}

fn validate_wallpaper(config: &ThemeConfig) -> Result<(), ValidationError> {
    let wp = &config.wallpaper;

    match wp.mode.as_str() {
        "image" => {
            if wp.path.is_none() {
                return Err(SemanticError::MissingWallpaperPath.into());
            }
            if wp.color.is_some() {
                return Err(SemanticError::ForbiddenWallpaperKey {
                    mode: "image".into(),
                    key: "color".into(),
                }
                .into());
            }
            if wp.shader.is_some() {
                return Err(SemanticError::ForbiddenWallpaperKey {
                    mode: "image".into(),
                    key: "shader".into(),
                }
                .into());
            }
        }
        "solid" => {
            if wp.color.is_none() {
                return Err(SemanticError::MissingWallpaperColor.into());
            }
            if wp.path.is_some() {
                return Err(SemanticError::ForbiddenWallpaperKey {
                    mode: "solid".into(),
                    key: "path".into(),
                }
                .into());
            }
            if wp.shader.is_some() {
                return Err(SemanticError::ForbiddenWallpaperKey {
                    mode: "solid".into(),
                    key: "shader".into(),
                }
                .into());
            }
        }
        "glsl" => {
            if wp.shader.is_none() {
                return Err(SemanticError::MissingWallpaperShader.into());
            }
            if wp.path.is_some() {
                return Err(SemanticError::ForbiddenWallpaperKey {
                    mode: "glsl".into(),
                    key: "path".into(),
                }
                .into());
            }
            if wp.color.is_some() {
                return Err(SemanticError::ForbiddenWallpaperKey {
                    mode: "glsl".into(),
                    key: "color".into(),
                }
                .into());
            }
        }
        "renderer" => {
            if config.renderer.is_none() {
                return Err(SemanticError::MissingRendererSection.into());
            }
            if wp.path.is_some() {
                return Err(SemanticError::ForbiddenWallpaperKey {
                    mode: "renderer".into(),
                    key: "path".into(),
                }
                .into());
            }
            if wp.color.is_some() {
                return Err(SemanticError::ForbiddenWallpaperKey {
                    mode: "renderer".into(),
                    key: "color".into(),
                }
                .into());
            }
            if wp.shader.is_some() {
                return Err(SemanticError::ForbiddenWallpaperKey {
                    mode: "renderer".into(),
                    key: "shader".into(),
                }
                .into());
            }
        }
        other => return Err(SemanticError::UnknownWallpaperMode(other.into()).into()),
    }

    Ok(())
}

fn check_not_empty(value: &str, section: &str, field: &str) -> Result<(), ValidationError> {
    if value.is_empty() {
        return Err(SemanticError::EmptyField {
            section: section.into(),
            field: field.into(),
        }
        .into());
    }
    Ok(())
}

fn validate_no_empty_strings(config: &ThemeConfig) -> Result<(), ValidationError> {
    // meta
    check_not_empty(&config.meta.name, "meta", "name")?;
    check_not_empty(&config.meta.version, "meta", "version")?;
    check_not_empty(&config.meta.description, "meta", "description")?;
    check_not_empty(&config.meta.renderer, "meta", "renderer")?;
    check_not_empty(&config.meta.widgets, "meta", "widgets")?;

    // typography
    check_not_empty(&config.typography.font_mono, "typography", "font_mono")?;

    // wallpaper
    check_not_empty(&config.wallpaper.mode, "wallpaper", "mode")?;

    // colors — all 23 fields
    let c = &config.colors;
    for (field, value) in color_fields(c) {
        check_not_empty(value, "colors", field)?;
    }

    Ok(())
}

fn validate_color_format(config: &ThemeConfig) -> Result<(), ValidationError> {
    let c = &config.colors;
    for (field, value) in color_fields(c) {
        if !is_valid_hex_color(value) {
            return Err(SemanticError::InvalidColorFormat {
                field: field.into(),
                value: value.into(),
            }
            .into());
        }
    }
    Ok(())
}

fn is_valid_hex_color(s: &str) -> bool {
    let bytes = s.as_bytes();
    bytes.len() == 7 && bytes[0] == b'#' && bytes[1..].iter().all(u8::is_ascii_hexdigit)
}

fn color_fields(c: &crate::schema::Colors) -> [(&str, &str); 23] {
    [
        ("bg", &c.bg),
        ("fg", &c.fg),
        ("black", &c.black),
        ("red", &c.red),
        ("green", &c.green),
        ("yellow", &c.yellow),
        ("blue", &c.blue),
        ("magenta", &c.magenta),
        ("cyan", &c.cyan),
        ("white", &c.white),
        ("bright_black", &c.bright_black),
        ("bright_red", &c.bright_red),
        ("bright_green", &c.bright_green),
        ("bright_yellow", &c.bright_yellow),
        ("bright_blue", &c.bright_blue),
        ("bright_magenta", &c.bright_magenta),
        ("bright_cyan", &c.bright_cyan),
        ("bright_white", &c.bright_white),
        ("accent", &c.accent),
        ("surface0", &c.surface0),
        ("surface1", &c.surface1),
        ("surface2", &c.surface2),
        ("muted", &c.muted),
    ]
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;
    use crate::schema::*;

    fn valid_config() -> ThemeConfig {
        ThemeConfig {
            meta: Meta {
                name: "test".into(),
                version: "0.1".into(),
                description: "Test theme.".into(),
                tags: vec!["test".into()],
                renderer: "none".into(),
                widgets: "none".into(),
            },
            typography: Typography {
                font_mono: "Iosevka".into(),
                font_size: 13,
            },
            colors: Colors {
                bg: "#0a0a0a".into(),
                fg: "#e0e0e0".into(),
                black: "#0a0a0a".into(),
                red: "#ff5555".into(),
                green: "#55ff55".into(),
                yellow: "#f1c40f".into(),
                blue: "#6699cc".into(),
                magenta: "#cc66ff".into(),
                cyan: "#66ffcc".into(),
                white: "#e0e0e0".into(),
                bright_black: "#333333".into(),
                bright_red: "#ff7777".into(),
                bright_green: "#77ff77".into(),
                bright_yellow: "#f5d84f".into(),
                bright_blue: "#88bbee".into(),
                bright_magenta: "#dd88ff".into(),
                bright_cyan: "#88ffdd".into(),
                bright_white: "#ffffff".into(),
                accent: "#ffffff".into(),
                surface0: "#111111".into(),
                surface1: "#1a1a1a".into(),
                surface2: "#242424".into(),
                muted: "#555555".into(),
            },
            hyprland: Hyprland {
                border_size: 1,
                gaps_in: 4,
                gaps_out: 8,
                rounding: 0,
                blur: false,
                animations: None,
            },
            workspaces: None,
            wallpaper: Wallpaper {
                mode: "solid".into(),
                path: None,
                color: Some("#0a0a0a".into()),
                shader: None,
            },
            renderer: None,
            widgets: None,
        }
    }

    // --- wallpaper mode rules ---

    #[test]
    fn image_mode_missing_path_returns_error() {
        let mut cfg = valid_config();
        cfg.wallpaper.mode = "image".into();
        cfg.wallpaper.color = None;
        cfg.wallpaper.path = None;
        let err = validate_semantic(&cfg).unwrap_err();
        assert!(
            matches!(
                err,
                ValidationError::Semantic(SemanticError::MissingWallpaperPath)
            ),
            "expected MissingWallpaperPath, got: {err:?}"
        );
    }

    #[test]
    fn image_mode_with_forbidden_color_returns_error() {
        let mut cfg = valid_config();
        cfg.wallpaper.mode = "image".into();
        cfg.wallpaper.path = Some("/tmp/wall.png".into());
        cfg.wallpaper.color = Some("#000000".into());
        let err = validate_semantic(&cfg).unwrap_err();
        assert!(
            matches!(err, ValidationError::Semantic(SemanticError::ForbiddenWallpaperKey { ref mode, ref key }) if mode == "image" && key == "color"),
            "expected ForbiddenWallpaperKey image/color, got: {err:?}"
        );
    }

    #[test]
    fn image_mode_with_forbidden_shader_returns_error() {
        let mut cfg = valid_config();
        cfg.wallpaper.mode = "image".into();
        cfg.wallpaper.path = Some("/tmp/wall.png".into());
        cfg.wallpaper.color = None;
        cfg.wallpaper.shader = Some("frag.glsl".into());
        let err = validate_semantic(&cfg).unwrap_err();
        assert!(
            matches!(err, ValidationError::Semantic(SemanticError::ForbiddenWallpaperKey { ref mode, ref key }) if mode == "image" && key == "shader"),
            "expected ForbiddenWallpaperKey image/shader, got: {err:?}"
        );
    }

    #[test]
    fn solid_mode_missing_color_returns_error() {
        let mut cfg = valid_config();
        cfg.wallpaper.mode = "solid".into();
        cfg.wallpaper.color = None;
        let err = validate_semantic(&cfg).unwrap_err();
        assert!(
            matches!(
                err,
                ValidationError::Semantic(SemanticError::MissingWallpaperColor)
            ),
            "expected MissingWallpaperColor, got: {err:?}"
        );
    }

    #[test]
    fn glsl_mode_missing_shader_returns_error() {
        let mut cfg = valid_config();
        cfg.wallpaper.mode = "glsl".into();
        cfg.wallpaper.color = None;
        cfg.wallpaper.shader = None;
        let err = validate_semantic(&cfg).unwrap_err();
        assert!(
            matches!(
                err,
                ValidationError::Semantic(SemanticError::MissingWallpaperShader)
            ),
            "expected MissingWallpaperShader, got: {err:?}"
        );
    }

    #[test]
    fn renderer_mode_missing_section_returns_error() {
        let mut cfg = valid_config();
        cfg.wallpaper.mode = "renderer".into();
        cfg.wallpaper.color = None;
        cfg.renderer = None;
        let err = validate_semantic(&cfg).unwrap_err();
        assert!(
            matches!(
                err,
                ValidationError::Semantic(SemanticError::MissingRendererSection)
            ),
            "expected MissingRendererSection, got: {err:?}"
        );
    }

    #[test]
    fn renderer_mode_with_forbidden_path_returns_error() {
        let mut cfg = valid_config();
        cfg.wallpaper.mode = "renderer".into();
        cfg.wallpaper.color = None;
        cfg.wallpaper.path = Some("/tmp/wall.png".into());
        cfg.renderer = Some(Renderer {
            binary: "bin".into(),
            args: vec![],
            fps: 30,
            params: None,
        });
        let err = validate_semantic(&cfg).unwrap_err();
        assert!(
            matches!(err, ValidationError::Semantic(SemanticError::ForbiddenWallpaperKey { ref mode, ref key }) if mode == "renderer" && key == "path"),
            "expected ForbiddenWallpaperKey renderer/path, got: {err:?}"
        );
    }

    #[test]
    fn unknown_wallpaper_mode_returns_error() {
        let mut cfg = valid_config();
        cfg.wallpaper.mode = "banana".into();
        cfg.wallpaper.color = None;
        let err = validate_semantic(&cfg).unwrap_err();
        assert!(
            matches!(err, ValidationError::Semantic(SemanticError::UnknownWallpaperMode(ref m)) if m == "banana"),
            "expected UnknownWallpaperMode, got: {err:?}"
        );
    }

    // --- empty string checks ---

    #[test]
    fn empty_meta_name_returns_error() {
        let mut cfg = valid_config();
        cfg.meta.name = String::new();
        let err = validate_semantic(&cfg).unwrap_err();
        assert!(
            matches!(err, ValidationError::Semantic(SemanticError::EmptyField { ref section, ref field }) if section == "meta" && field == "name"),
            "expected EmptyField meta.name, got: {err:?}"
        );
    }

    #[test]
    fn empty_color_field_returns_error() {
        let mut cfg = valid_config();
        cfg.colors.accent = String::new();
        let err = validate_semantic(&cfg).unwrap_err();
        assert!(
            matches!(err, ValidationError::Semantic(SemanticError::EmptyField { ref section, ref field }) if section == "colors" && field == "accent"),
            "expected EmptyField colors.accent, got: {err:?}"
        );
    }

    // --- color format ---

    #[test]
    fn invalid_color_hex_returns_error() {
        let mut cfg = valid_config();
        cfg.colors.red = "not-a-color".into();
        let err = validate_semantic(&cfg).unwrap_err();
        assert!(
            matches!(err, ValidationError::Semantic(SemanticError::InvalidColorFormat { ref field, ref value }) if field == "red" && value == "not-a-color"),
            "expected InvalidColorFormat red, got: {err:?}"
        );
    }

    // --- happy path ---

    #[test]
    fn valid_config_passes_semantic_validation() {
        let cfg = valid_config();
        validate_semantic(&cfg).unwrap();
    }
}
