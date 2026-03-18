//! Apply pipeline orchestration.
//!
//! Owns the sequencing invariant: preflight → hyprland → wallpaper.
//! This module does not contain business logic — it delegates to submodules
//! and ensures they execute in the correct order.
//!
//! # Pipeline Stages
//!
//! 1. **Preflight** — environment checks (Hyprland running, tools available)
//! 2. **Hyprland** — configure gaps, rounding, borders via hyprctl
//! 3. **Wallpaper** — activate wallpaper based on mode
//!
//! # Public Surface
//!
//! Only `apply()` and `ApplyError` are public. All submodules are private.

pub mod error;
mod hyprland;
mod preflight;
mod wallpaper;

pub use error::ApplyError;

use crate::schema::ThemeConfig;

/// Apply a validated theme configuration to the desktop.
///
/// Executes the full apply pipeline in order:
/// 1. Preflight checks (hard failures abort, soft failures warn)
/// 2. Hyprland configuration
/// 3. Wallpaper activation
///
/// # Errors
///
/// Returns `ApplyError` if any pipeline stage fails.
pub fn apply(config: &ThemeConfig) -> Result<(), ApplyError> {
    // Stage 1: Preflight checks
    let preflight_result = preflight::check(config)?;
    for warning in &preflight_result.warnings {
        eprintln!("warning: {warning}");
    }

    // Stage 2: Hyprland configuration
    hyprland::apply(config)?;

    // Stage 3: Wallpaper activation
    wallpaper::activate(config)?;

    Ok(())
}
