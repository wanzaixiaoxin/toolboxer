//! Utility functions for the Toolboxer application
//!
//! This module provides helper functions for common tasks like
//! formatting permissions, handling file metadata, and colorizing output.

use colored::Colorize;
use std::fs::{self, Metadata};
use std::path::Path;
use std::time::SystemTime;
use crate::error::Result;

/// Formats file permissions as a string (e.g., "rwxr--r--")
///
/// # Arguments
/// * `metadata` - File metadata containing permission information
///
/// # Returns
/// A string representation of the file permissions
pub fn format_permissions(metadata: &Metadata) -> String {
    let mut result = String::with_capacity(9);
    let readonly = metadata.permissions().readonly();
    
    if cfg!(windows) {
        // Windows只显示简单的读写权限
        result.push(if !readonly { 'r' } else { '-' });
        result.push(if !readonly { 'w' } else { '-' });
        result.push('-');
        result.push_str("------");
    } else {
        // Unix-like系统使用更详细的权限
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = metadata.permissions().mode();
            
            // 用户权限
            result.push(if mode & 0o400 != 0 { 'r' } else { '-' });
            result.push(if mode & 0o200 != 0 { 'w' } else { '-' });
            result.push(if mode & 0o100 != 0 { 'x' } else { '-' });
            
            // 组权限
            result.push(if mode & 0o040 != 0 { 'r' } else { '-' });
            result.push(if mode & 0o020 != 0 { 'w' } else { '-' });
            result.push(if mode & 0o010 != 0 { 'x' } else { '-' });
            
            // 其他用户权限
            result.push(if mode & 0o004 != 0 { 'r' } else { '-' });
            result.push(if mode & 0o002 != 0 { 'w' } else { '-' });
            result.push(if mode & 0o001 != 0 { 'x' } else { '-' });
        }
        
        #[cfg(not(unix))]
        {
            result.push_str("rw-r--r--");
        }
    }
    
    result
}

/// Formats a system time as a string
///
/// # Arguments
/// * `time` - The system time to format
///
/// # Returns
/// A string representation of the time (seconds since epoch)
pub fn format_time(time: std::time::SystemTime) -> String {
    use std::time::UNIX_EPOCH;
    time.duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "Unknown".to_string())
}

/// Logs command execution metrics including duration and status
///
/// # Arguments
/// * `command` - The command being executed
/// * `duration` - Execution duration in milliseconds
/// * `status` - Execution status (success/failure)
/// * `output_size` - Optional output size in bytes
pub fn log_command_metrics(command: &str, duration: u128, status: &str, output_size: Option<usize>) {
    /*
    println!(
        "[CMD_METRICS] command={} duration={}ms status={} output_size={}",
        command,
        duration,
        status,
        output_size.unwrap_or(0)
    );
    */
}

/// Determines if a file or directory is hidden
///
/// Works cross-platform:
/// - On Windows: Checks the hidden attribute
/// - On Unix-like systems: Checks if the name starts with a dot
///
/// # Arguments
/// * `path` - The path to check
///
/// # Returns
/// `true` if the file or directory is hidden, `false` otherwise
pub fn is_hidden(path: &Path) -> bool {
    if cfg!(windows) {
        use std::os::windows::fs::MetadataExt;
        if let Ok(metadata) = fs::metadata(path) {
            // FILE_ATTRIBUTE_HIDDEN = 0x2
            return metadata.file_attributes() & 0x2 != 0;
        }
    }
    
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}

/// Colorizes a file or directory name based on its type
///
/// # Arguments
/// * `name` - The name to colorize
/// * `is_dir` - Whether the name represents a directory
///
/// # Returns
/// A colorized string representation of the name
pub fn colorize_name(name: &str, is_dir: bool) -> String {
    if is_dir {
        name.blue().bold().to_string()
    } else if name.ends_with(".exe") || name.ends_with(".bat") || name.ends_with(".cmd") {
        name.green().to_string()
    } else if name.ends_with(".rs") || name.ends_with(".toml") {
        name.yellow().to_string()
    } else {
        name.to_string()
    }
}

/// Checks if a file name matches a simple pattern
///
/// # Arguments
/// * `path` - The path to check
/// * `pattern` - The pattern to match against
///
/// # Returns
/// `true` if the file name contains the pattern, `false` otherwise
pub fn matches_pattern(path: &Path, pattern: &str) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.contains(pattern))
        .unwrap_or(false)
}

/// Retrieves file size and modification time
///
/// # Arguments
/// * `path` - The path to the file
///
/// # Returns
/// A tuple containing the file size in bytes and the modification time
/// or an error if the file cannot be accessed
pub fn get_file_info(path: &Path) -> Result<(u64, SystemTime)> {
    let metadata = fs::metadata(path)?;
    let size = metadata.len();
    let modified = metadata.modified()?;
    Ok((size, modified))
}