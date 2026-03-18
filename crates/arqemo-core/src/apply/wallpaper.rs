//! Wallpaper activation based on [wallpaper] mode.
//!
//! Owns the invariant: mode field → appropriate wallpaper tool invocation.
//! Phase 0 implements `image` mode only (swww/swaybg).
//! Other modes return unimplemented errors with comments indicating their target phase.

use std::process::Command;

use crate::schema::ThemeConfig;

use super::ApplyError;

/// Activate wallpaper based on theme configuration.
///
/// Dispatches to the appropriate tool based on the `mode` field:
/// - `image`: static image via swww or swaybg (Phase 0)
/// - `solid`: solid color (Phase 1)
/// - `glsl`: GLSL shader (Phase 3)
/// - `renderer`: wgpu world engine (Phase 3)
///
/// # Errors
///
/// Returns `ApplyError` if wallpaper activation fails or mode is unimplemented.
pub fn activate(config: &ThemeConfig) -> Result<(), ApplyError> {
    match config.wallpaper.mode.as_str() {
        "image" => activate_image(config),
        "solid" => {
            // Phase 1: solid color wallpaper via swaybg --mode solid_color --color
            Err(ApplyError::WallpaperModeUnimplemented("solid".to_string()))
        }
        "glsl" => {
            // Phase 3: GLSL shader via mpvpaper or custom renderer
            Err(ApplyError::WallpaperModeUnimplemented("glsl".to_string()))
        }
        "renderer" => {
            // Phase 3: wgpu world engine via arqemo-renderer
            Err(ApplyError::WallpaperModeUnimplemented("renderer".to_string()))
        }
        mode => Err(ApplyError::WallpaperModeUnimplemented(mode.to_string())),
    }
}

/// Activate a static image wallpaper.
///
/// Tries swww first (better transitions), falls back to swaybg.
fn activate_image(config: &ThemeConfig) -> Result<(), ApplyError> {
    let path = config
        .wallpaper
        .path
        .as_ref()
        .ok_or_else(|| ApplyError::WallpaperFailed("image mode requires path".to_string()))?;

    // Try swww first, fall back to swaybg
    if try_swww(path).is_ok() {
        return Ok(());
    }

    try_swaybg(path)
}

/// Try to set wallpaper using swww.
fn try_swww(path: &str) -> Result<(), ApplyError> {
    let output = Command::new("swww")
        .args(["img", path])
        .output()
        .map_err(|e| ApplyError::WallpaperFailed(e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(ApplyError::WallpaperFailed(stderr.to_string()));
    }
    Ok(())
}

/// Try to set wallpaper using swaybg.
///
/// Note: swaybg runs as a daemon, so we spawn and detach.
fn try_swaybg(path: &str) -> Result<(), ApplyError> {
    Command::new("swaybg")
        .args(["--image", path, "--mode", "fill"])
        .spawn()
        .map_err(|e| ApplyError::WallpaperFailed(e.to_string()))?;
    Ok(())
}
