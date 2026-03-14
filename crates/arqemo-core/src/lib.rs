pub mod cache;
pub mod schema;
pub mod template;
pub mod validate;

use anyhow::Result;

/// Apply a theme to the desktop.
///
/// # Errors
///
/// Returns an error if the theme cannot be applied.
#[allow(clippy::unused_async)]
pub async fn apply(theme: &str, dry_run: bool) -> Result<()> {
    let _ = (theme, dry_run);
    todo!("apply pipeline — Phase 0")
}

/// Validate a theme file without applying it.
///
/// # Errors
///
/// Returns an error if the theme is invalid.
//pub fn validate(theme: &str) -> Result<()> {
//  let _ = theme;
//  todo!("validate command — Phase 1 and expose api to other crates")
//}

/// List all available themes.
///
/// # Errors
///
/// Returns an error if the themes directory cannot be read.
pub fn list() -> Result<()> {
    todo!("list command — Phase 6")
}
