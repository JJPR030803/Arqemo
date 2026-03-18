//! Error types for the apply pipeline.
//!
//! Owns one invariant: one error variant per failure domain.
//! Each variant carries context (`PathBuf` or `String`) for actionable diagnostics.

use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during theme application.
#[derive(Debug, Error)]
pub enum ApplyError {
    /// Hyprland compositor is not running.
    #[error("Hyprland not running: HYPRLAND_INSTANCE_SIGNATURE not set")]
    HyprlandNotRunning,

    /// Hyprland socket file does not exist at the expected path.
    #[error("Hyprland socket not found at '{0}'")]
    HyprlandSocketMissing(PathBuf),

    /// A hyprctl command failed to execute or returned an error.
    #[error("hyprctl command failed: {0}")]
    HyprctlFailed(String),

    /// A required wallpaper tool is not installed.
    #[error("wallpaper tool '{tool}' not found in PATH")]
    WallpaperToolMissing { tool: String },

    /// Wallpaper activation command failed.
    #[error("wallpaper command failed: {0}")]
    WallpaperFailed(String),

    /// Wallpaper mode is not yet implemented.
    #[error("wallpaper mode '{0}' not implemented yet")]
    WallpaperModeUnimplemented(String),
}
