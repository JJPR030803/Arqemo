pub mod cache;
pub mod config;
pub mod schema;
pub mod template;
pub mod validate;

use anyhow::Result;
use cache::CacheLayout;

/// Initialise the arqemo directory structure.
///
/// Creates the following directories if they do not already exist:
/// - `~/.config/arqemo/`
/// - `~/.config/arqemo/themes/`
/// - `~/.config/arqemo/templates/`
/// - `~/.cache/arqemo/`
/// - `~/.cache/arqemo/rendered/`
/// - `~/.cache/arqemo/themes/`
///
/// Idempotent — safe to run multiple times.
///
/// # Errors
///
/// Returns an error if any directory cannot be created.
pub fn init() -> Result<()> {
    use config::root::ConfigRoot;

    let config = ConfigRoot::ensure()?;

    let templates = template::templates_dir()?;
    std::fs::create_dir_all(&templates)?;

    let cache_path = cache::search_cache_dir()?;
    CacheLayout::ensure(&cache_path)?;

    println!("config:    {}", config.base.display());
    println!("themes:    {}", config.themes_dir.display());
    println!("templates: {}", templates.display());
    println!("cache:     {}", cache_path.display());

    Ok(())
}

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

    use crate::config::root::ConfigRoot;

    let root = ConfigRoot::locate()?;

    println!("{:#?}", root);

    Ok(())
}

#[test]
fn test_list() -> Result<()> {
    list()
}