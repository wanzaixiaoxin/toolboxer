//! Toolboxer library crate
//! 
//! This crate provides the core functionality for the Toolboxer command-line toolkit.
//! It includes modules for CLI argument parsing, command execution, configuration,
//! error handling, and utility functions.

/// Command-line interface definitions and argument parsing
pub mod cli;
/// Command execution logic for various Toolboxer features
pub mod commands;
/// Configuration structures and methods
pub mod config;
/// Error types and result definitions
pub mod error;
/// Utility functions used across the application
pub mod utils;

// Re-export error types for convenience
pub use error::{Error, Result};

// Re-export commonly used items for easier access
pub use config::Config;