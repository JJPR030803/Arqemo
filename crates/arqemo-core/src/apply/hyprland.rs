//! Hyprland configuration via hyprctl.
//!
//! Owns the invariant: [hyprland] section → hyprctl keyword calls.
//! Phase 0 uses `std::process::Command`, not the IPC socket.
//! Phase 3+ will migrate to `arqemo-ipc` for direct socket communication.

use std::process::Command;

use crate::schema::ThemeConfig;

use super::ApplyError;

/// Apply Hyprland configuration from the theme.
///
/// Sets gaps, rounding, border size, and border color via `hyprctl keyword`.
///
/// # Errors
///
/// Returns `ApplyError::HyprctlFailed` if any hyprctl command fails.
pub fn apply(config: &ThemeConfig) -> Result<(), ApplyError> {
    let h = &config.hyprland;

    hyprctl_keyword("general:gaps_in", &h.gaps_in.to_string())?;
    hyprctl_keyword("general:gaps_out", &h.gaps_out.to_string())?;
    hyprctl_keyword("decoration:rounding", &h.rounding.to_string())?;
    hyprctl_keyword("general:border_size", &h.border_size.to_string())?;

    // Border color from theme colors
    let border_color = &config.colors.accent;
    hyprctl_keyword(
        "general:col.active_border",
        &format!("rgb({})", border_color.trim_start_matches('#')),
    )?;

    Ok(())
}

/// Execute a single hyprctl keyword command.
fn hyprctl_keyword(key: &str, value: &str) -> Result<(), ApplyError> {
    let output = Command::new("hyprctl")
        .args(["keyword", key, value])
        .output()
        .map_err(|e| ApplyError::HyprctlFailed(e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(ApplyError::HyprctlFailed(stderr.to_string()));
    }
    Ok(())
}
