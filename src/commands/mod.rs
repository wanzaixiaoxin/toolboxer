//! Command modules for the Toolboxer application
//!
//! This module organizes and re-exports command executors for various
//! Toolboxer features.

/// Module containing the implementation of the 'tree' command
pub mod tree;

// Re-export command executors for easier access
/// Re-export of the tree command's execute function
pub use tree::execute as execute_tree;