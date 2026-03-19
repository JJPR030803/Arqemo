//! Preflight checks before applying a theme.
//!
//! Owns two invariants:
//! 1. HARD: Hyprland must be running (`HYPRLAND_INSTANCE_SIGNATURE` set, socket exists)
//! 2. SOFT: Tools referenced in config should exist in PATH (warnings, not errors)
//!
//! Hard checks fail the pipeline immediately. Soft checks collect warnings
//! that are reported but do not block application.

use std::path::PathBuf;

use crate::schema::ThemeConfig;

use super::ApplyError;

/// Result of preflight checks, containing any warnings for soft failures.
pub struct PreflightResult {
    /// Warnings about missing tools or non-critical issues.
    pub warnings: Vec<String>,
}

/// Run all preflight checks.
///
/// # Errors
///
/// Returns `ApplyError` if a hard check fails (Hyprland not running).
pub fn check(config: &ThemeConfig) -> Result<PreflightResult, ApplyError> {
    // Hard check: Hyprland running
    check_hyprland_running()?;

    // Soft checks: tool availability
    let warnings = check_tools(config);

    Ok(PreflightResult { warnings })
}

/// Verify Hyprland is running by checking environment and socket.
fn check_hyprland_running() -> Result<(), ApplyError> {
    let _socket = hyprland_socket_path()?;
    Ok(())
}

/// Resolve the Hyprland socket path.
///
/// Hyprland 0.46+ uses `$XDG_RUNTIME_DIR/hypr/`, older versions used `/tmp/hypr/`.
fn hyprland_socket_path() -> Result<PathBuf, ApplyError> {
    let sig = std::env::var("HYPRLAND_INSTANCE_SIGNATURE")
        .map_err(|_| ApplyError::HyprlandNotRunning)?;

    // Hyprland 0.46+ uses XDG_RUNTIME_DIR, older versions used /tmp/hypr
    let base = std::env::var("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"));

    let socket = base.join("hypr").join(&sig).join(".socket.sock");

    if !socket.exists() {
        return Err(ApplyError::HyprlandSocketMissing(socket));
    }

    Ok(socket)
}

/// Check that required tools are available in PATH.
/// Returns warnings for missing tools (soft failures).
fn check_tools(config: &ThemeConfig) -> Vec<String> {
    let mut warnings = Vec::new();

    // Check wallpaper tool based on mode
    if config.wallpaper.mode == crate::schema::WallpaperMode::Image
        && !tool_exists("swww")
        && !tool_exists("swaybg")
    {
        warnings.push("neither swww nor swaybg found in PATH".to_string());
    }

    warnings
}

/// Check if a tool exists in PATH using `which`.
fn tool_exists(name: &str) -> bool {
    std::process::Command::new("which")
        .arg(name)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
