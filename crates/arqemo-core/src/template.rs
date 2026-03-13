//! Tera template rendering.
//! Reads .tera templates from ~/.config/arqemo/templates/
//! Writes rendered output to ~/.cache/arqemo/

use std::path::PathBuf;

use anyhow::Result;

/// Returns the path to the arqemo templates directory.
///
/// # Errors
///
/// Returns an error if the config directory cannot be determined.
pub fn templates_dir() -> Result<PathBuf> {
    let base = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("could not determine config directory"))?;
    Ok(base.join("arqemo").join("templates"))
}

/// Returns the path to the arqemo cache directory.
///
/// # Errors
///
/// Returns an error if the cache directory cannot be determined.
pub fn cache_dir() -> Result<PathBuf> {
    let base =
        dirs::cache_dir().ok_or_else(|| anyhow::anyhow!("could not determine cache directory"))?;
    Ok(base.join("arqemo"))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn placeholder_template_test() {
        // Fixture-based rendering tests go in crates/arqemo-core/tests/
        let _ = templates_dir();
    }
}
