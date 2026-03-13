//! Semantic validation layer.
//! Runs after serde type validation. Checks conditional rules
//! that serde cannot express: mode-conditional wallpaper keys,
//! renderer requirement, missing color keys, path existence.

use crate::schema::ThemeConfig;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error(transparent)]
    File(#[from] FileError),


    #[error(transparent)]
    Semantic(#[from] SemanticError),

}

#[derive(Error, Debug)]
pub enum FileError{

    // Path related errors

    /// When path dont exists uses `std::path::PathBuf`.
    #[error("Path does not exist: {0}")]
    PathDoesNotExist(std::path::PathBuf),

    /// When path given is not a file, meaning a directory `std::path::PathBuf`.
    #[error("Path is not a file: {0}")]
    PathIsNotFile(std::path::PathBuf),

    /// When extension is not TOML `String`.
    #[error("Wrong extension: {0}")]
    WrongExtension(String),

    #[error("File is empty: {0}")]
    FileIsEmpty(std::path::PathBuf),

    // Open Read related errors
    #[error("failed to open file: {0}")]
    OpenFileError(std::io::Error),

    #[error("failed to read file: {0}")]
    ReadFileError(std::io::Error),


    // Parsing errors
    #[error("failed to parse file: {0}")]
   ParseError(#[from] toml::de::Error),

}

#[derive(Error, Debug)]
pub enum SemanticError{
    #[error("invalid theme config: {0}")]
    InvalidThemeConfig(String),
}
