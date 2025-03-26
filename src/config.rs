//! Configuration management for Toolboxer
//! 
//! This module provides configuration structures and methods for managing
//! various settings and options used throughout the application.

use crate::error::{Error, Result};
use std::path::PathBuf;

/// Configuration structure for command execution
/// 
/// Holds all the settings that control how commands operate,
/// particularly for the tree command's display options.
#[derive(Debug, Clone)]
pub struct Config {
    /// Root directory path for operations
    pub root: PathBuf,
    /// Maximum depth to traverse (None for unlimited)
    pub max_depth: Option<usize>,
    /// Whether to show hidden files
    pub show_hidden: bool,
    /// How to sort the directory entries
    pub sort_by: SortBy,
    /// Whether to show file permissions
    pub show_permissions: bool,
    /// Whether to show file sizes
    pub show_size: bool,
    /// Whether to show modification dates
    pub show_date: bool,
    /// Optional pattern for filtering files
    pub pattern: Option<String>,
}

/// Enumeration of available sorting methods
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortBy {
    /// Sort by file/directory name
    Name,
    /// Sort by entry type (directories first)
    Type,
    /// Sort by file size
    Size,
    /// Sort by modification date
    Date,
}

impl Config {
    /// Creates a new Config instance with default settings
    /// 
    /// # Arguments
    /// * `root` - The root directory path for operations
    /// 
    /// # Returns
    /// A new Config instance with default settings
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            max_depth: None,
            show_hidden: false,
            sort_by: SortBy::Name,
            show_permissions: false,
            show_size: false,
            show_date: false,
            pattern: None,
        }
    }

    /// Sets the maximum depth for directory traversal
    /// 
    /// # Arguments
    /// * `depth` - Maximum depth to traverse (must be non-negative)
    /// 
    /// # Returns
    /// * `Ok(Config)` - Updated configuration
    /// * `Err(Error)` - If depth is negative
    pub fn with_max_depth(mut self, depth: i32) -> Result<Self> {
        if depth < 0 {
            return Err(Error::InvalidDepth(depth));
        }
        self.max_depth = Some(depth as usize);
        Ok(self)
    }

    /// Sets whether to show hidden files
    /// 
    /// # Arguments
    /// * `show_hidden` - Whether to show hidden files and directories
    pub fn with_show_hidden(mut self, show_hidden: bool) -> Self {
        self.show_hidden = show_hidden;
        self
    }

    /// Sets the sorting method for directory entries
    /// 
    /// # Arguments
    /// * `sort_by` - The sorting method to use
    pub fn with_sort_by(mut self, sort_by: SortBy) -> Self {
        self.sort_by = sort_by;
        self
    }

    /// Sets whether to show file permissions
    /// 
    /// # Arguments
    /// * `show_permissions` - Whether to display file permissions
    pub fn with_show_permissions(mut self, show_permissions: bool) -> Self {
        self.show_permissions = show_permissions;
        self
    }

    /// Sets whether to show file sizes
    /// 
    /// # Arguments
    /// * `show_size` - Whether to display file sizes
    pub fn with_show_size(mut self, show_size: bool) -> Self {
        self.show_size = show_size;
        self
    }

    /// Sets whether to show modification dates
    /// 
    /// # Arguments
    /// * `show_date` - Whether to display modification dates
    pub fn with_show_date(mut self, show_date: bool) -> Self {
        self.show_date = show_date;
        self
    }

    /// Sets a pattern for filtering files
    /// 
    /// # Arguments
    /// * `pattern` - Optional pattern string for filtering files
    /// 
    /// # Returns
    /// * `Ok(Config)` - Updated configuration
    /// * `Err(Error)` - If pattern is invalid
    pub fn with_pattern(mut self, pattern: Option<String>) -> Result<Self> {
        if let Some(p) = pattern {
            // Here you might want to validate the pattern
            self.pattern = Some(p);
        }
        Ok(self)
    }
}