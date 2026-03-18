pub mod cache;
pub mod config;
pub mod schema;
pub mod template;
pub mod validate;

use std::path::Path;
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
            println!("{}", path.file_name().unwrap().to_str().unwrap());
        }
        Ok(())
    }

    
}

pub fn validate_theme(theme: &str,info:bool) -> Result<()> {
    let theme_paths = ConfigRoot::locate()?.themes_dir;
    let themes = std::fs::read_dir(&theme_paths)?;
    let mut exists = false;
    
    for th in themes {
        if theme == th?.path().file_name().unwrap().to_str().unwrap_or(
            "IO error reading themes directory."
        ) {
            println!("{} is valid name", theme);
            exists = true;
            break;
        } 
    }
    
    let theme_to_validate = theme_paths.join(theme).join("theme.toml");
    if exists {
        let theme_cfg =  match validate::file::validate_file(&theme_to_validate) { 
            Ok(cfg) => Ok(cfg),
            Err(e) => Err(e),
        };
        let semantic_errors = match validate::semantic::validate_semantic(&theme_cfg?) { 
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        };
        if info { 
            println!("{} is a valid theme", theme);
            println!("{} is located at {}", theme, theme_to_validate.display());
            println!("{} is semantically valid", theme);
            println!("{} has the following errors:", theme);
        }
    } else { 
        println!("{} is not a valid name", theme);
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