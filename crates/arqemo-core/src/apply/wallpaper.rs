//! Wallpaper activation based on [wallpaper] mode.
//!
//! Owns the invariant: mode field → appropriate wallpaper tool invocation.
//! Phase 0 implements `image` mode only (swww/hyprpaper).
//! Other modes return unimplemented errors with comments indicating their target phase.

use std::path::PathBuf;
use std::process::Command;

use crate::schema::{ThemeConfig, WallpaperBackend, WallpaperMode};

use super::ApplyError;

/// Activate wallpaper based on theme configuration.
///
/// Dispatches to the appropriate tool based on the `mode` field:
/// - `image`: static image via swww or hyprpaper (Phase 0)
/// - `solid`: solid color (Phase 1)
/// - `glsl`: GLSL shader (Phase 3)
/// - `renderer`: wgpu world engine (Phase 3)
///
/// # Errors
///
/// Returns `ApplyError` if wallpaper activation fails or mode is unimplemented.
pub fn activate(config: &ThemeConfig) -> Result<(), ApplyError> {
    match config.wallpaper.mode {
        WallpaperMode::Image => activate_image(config),
        WallpaperMode::Solid => {
            Err(ApplyError::WallpaperModeUnimplemented("solid".to_string()))
        }
        WallpaperMode::Glsl => {
            Err(ApplyError::WallpaperModeUnimplemented("glsl".to_string()))
        }
        WallpaperMode::Renderer => {
            Err(ApplyError::WallpaperModeUnimplemented("renderer".to_string()))
        }
    }
}

/// Expand `~` at the start of a path to the user's home directory.
fn expand_tilde(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home).join(rest);
        }
    }
    PathBuf::from(path)
}

/// Resolve the wallpaper image path from either `path` or `pool`+`default`.
fn resolve_image_path(config: &ThemeConfig) -> Result<PathBuf, ApplyError> {
    let w = &config.wallpaper;

    // Direct path takes priority
    if let Some(ref p) = w.path {
        return Ok(expand_tilde(p));
    }

    // Pool + default: join pool dir with the default filename
    if let Some(ref pool_dir) = w.pool {
        let default_name = w.default.as_ref().ok_or_else(|| {
            ApplyError::WallpaperFailed("pool requires a default wallpaper name".to_string())
        })?;
        return Ok(expand_tilde(pool_dir).join(default_name));
    }

    Err(ApplyError::WallpaperFailed(
        "image mode requires path or pool+default".to_string(),
    ))
}

/// Activate a static image wallpaper.
///
/// Respects the `backend` field from the theme config.
/// Defaults to hyprpaper when no backend is specified.
fn activate_image(config: &ThemeConfig) -> Result<(), ApplyError> {
    let path = resolve_image_path(config)?;
    let path_str = path.to_string_lossy();

    let backend = config
        .wallpaper
        .backend
        .unwrap_or(WallpaperBackend::Hyprpaper);

    match backend {
        WallpaperBackend::Swww => try_swww(&path_str, config.wallpaper.transition.as_ref()),
        WallpaperBackend::Hyprpaper => try_hyprpaper(&path_str),
    }
}

/// Set wallpaper via hyprctl hyprpaper (preload + wallpaper).
fn try_hyprpaper(path: &str) -> Result<(), ApplyError> {
    // Unload all first to avoid "already loaded" errors
    let _ = Command::new("hyprctl")
        .args(["hyprpaper", "unload", "all"])
        .output();

    let output = Command::new("hyprctl")
        .args(["hyprpaper", "preload", path])
        .output()
        .map_err(|e| ApplyError::WallpaperFailed(format!("hyprctl hyprpaper preload: {e}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(ApplyError::WallpaperFailed(format!(
            "hyprpaper preload: {stderr}"
        )));
    }

    // Set on all monitors with empty monitor selector
    let wallpaper_arg = format!(",{path}");
    let output = Command::new("hyprctl")
        .args(["hyprpaper", "wallpaper", &wallpaper_arg])
        .output()
        .map_err(|e| ApplyError::WallpaperFailed(format!("hyprctl hyprpaper wallpaper: {e}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(ApplyError::WallpaperFailed(format!(
            "hyprpaper wallpaper: {stderr}"
        )));
    }
    Ok(())
}

/// Ensure swww-daemon is running, starting it if needed.
fn ensure_swww_daemon() -> Result<(), ApplyError> {
    // Quick check: if `swww query` succeeds, daemon is already up
    if let Ok(output) = Command::new("swww").arg("query").output() {
        if output.status.success() {
            return Ok(());
        }
    }

    // Spawn the daemon and give it a moment to initialize
    Command::new("swww-daemon")
        .spawn()
        .map_err(|e| ApplyError::WallpaperFailed(format!("failed to start swww-daemon: {e}")))?;

    std::thread::sleep(std::time::Duration::from_millis(500));
    Ok(())
}

/// Set wallpaper using swww, with optional transition config.
fn try_swww(
    path: &str,
    transition: Option<&crate::schema::WallpaperTransition>,
) -> Result<(), ApplyError> {
    ensure_swww_daemon()?;

    let mut args = vec!["img".to_string(), path.to_string()];

    if let Some(t) = transition {
        args.push("--transition-type".to_string());
        args.push(t.kind.clone());
        args.push("--transition-duration".to_string());
        args.push(t.duration.to_string());
        args.push("--transition-fps".to_string());
        args.push(t.fps.to_string());
        if let Some(ref bezier) = t.bezier {
            args.push("--transition-bezier".to_string());
            args.push(bezier.clone());
        }
    }

    let arg_refs: Vec<&str> = args.iter().map(std::string::String::as_str).collect();
    let output = Command::new("swww")
        .args(&arg_refs)
        .output()
        .map_err(|e| ApplyError::WallpaperFailed(format!("swww: {e}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(ApplyError::WallpaperFailed(format!("swww: {stderr}")));
    }
    Ok(())
}
