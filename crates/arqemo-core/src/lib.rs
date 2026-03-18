pub mod cache;
pub mod config;
pub mod schema;
pub mod template;
pub mod validate;
pub mod apply;

use crate::validate::{file, semantic};
use anyhow::Result;
use cache::CacheLayout;
use crate::config::root::ConfigRoot;

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
/// Loads and validates the theme, then applies it via the apply pipeline:
/// 1. Preflight checks (Hyprland running, tools available)
/// 2. Hyprland configuration (gaps, rounding, borders)
/// 3. Wallpaper activation
///
/// # Errors
///
/// Returns an error if the theme cannot be loaded, validated, or applied.
#[allow(clippy::unused_async)]
pub async fn apply(theme: &str, dry_run: bool) -> Result<()> {
    let theme_dir = ConfigRoot::locate()?.themes_dir.join(theme);
    let toml_path = theme_dir.join("theme.toml");

    if !toml_path.exists() {
        anyhow::bail!("theme '{}' not found in themes directory", theme);
    }

    let config = file::validate_file(&toml_path)?;
    semantic::validate_semantic(&config)?;

    if dry_run {
        println!("dry-run: would apply theme '{}'", theme);
        return Ok(());
    }

    apply::apply(&config)?;
    Ok(())
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
pub fn list_themes(complete:bool) -> Result<()> {

    use crate::config::root::ConfigRoot;

    let themes_root = ConfigRoot::locate()?.themes_dir;
    let themes = std::fs::read_dir(&themes_root)?;
    if complete{
        for theme in themes {
            let theme = theme?;
            let path = theme.path();
            println!("{}", path.display());
        }
         Ok(())
    }else { 
        for theme in themes {
            let theme = theme?;
            let path = theme.path();
            if let Some (name) = path.file_name().and_then(|s| s.to_str()) { 
                println!("{}", name);
            } 
        }
        Ok(())
    }

    
}

pub fn validate_theme(theme: &str,info:bool) -> Result<()> {
   let theme_dir = ConfigRoot::locate()?.themes_dir.join(theme);
    let toml_theme = theme_dir.join("theme.toml");
    
    if !toml_theme.exists() { 
        anyhow::bail!("theme '{}' not found in themes directory", theme);
    } 
    
    let cfg = validate::file::validate_file(&toml_theme)?;
    validate::semantic::validate_semantic(&cfg)?;
    
    if info  { 
        println!("{} is valid",theme);
        println!("{}", toml_theme.display());
    } 
    Ok(())
}

#[test]
fn test_list() -> Result<()> {
    list_themes(true)
}

#[test]
fn test_validate() -> Result<()> {
    validate_theme("brutalist",true)
}