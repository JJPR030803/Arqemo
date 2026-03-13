//! Cache file management.
//! Writes colors.lua, colors.css, active.toml to ~/.cache/arqemo/

use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::template::cache_dir;

/// Write a file to the arqemo cache directory.
///
/// # Errors
///
/// Returns an error if the cache directory cannot be created or the file cannot be written.
pub fn write(filename: &str, content: &str) -> Result<()> {
    let dir = cache_dir()?;
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("could not create cache dir: {}", dir.display()))?;
    let path = dir.join(filename);
    std::fs::write(&path, content)
        .with_context(|| format!("could not write cache file: {}", path.display()))?;
    Ok(())
}

/// Returns the path to the active theme cache file.
///
/// # Errors
///
/// Returns an error if the cache directory cannot be determined.
pub fn active_toml_path() -> Result<PathBuf> {
    Ok(cache_dir()?.join("active.toml"))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn placeholder_cache_test() {
        let _ = active_toml_path();
    }
}
