//! Error handling for the Toolboxer application
//!
//! This module defines custom error types and a Result type alias
//! to streamline error handling across the application.

use std::path::PathBuf;
use thiserror::Error;

/// Custom Result type for Toolboxer operations
pub type Result<T> = std::result::Result<T, Error>;

/// Enum representing various error types that can occur in Toolboxer
#[derive(Error, Debug)]
pub enum Error {
    /// Represents I/O errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Represents errors when accessing a file or directory
    #[error("Failed to access path: {}", .0.display())]
    PathAccess(PathBuf),

    /// Represents errors when an invalid depth value is provided
    #[error("Invalid depth value: {0}")]
    InvalidDepth(i32),

    /// Represents errors related to file pattern matching
    #[error("Pattern error: {0}")]
    Pattern(String),

    /// Represents any other unspecified errors
    #[error("Unknown error: {0}")]
    Other(String),
}