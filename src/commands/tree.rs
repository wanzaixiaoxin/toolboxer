//! tree命令实现
//!
//! 本模块提供以树状格式显示目录结构的功能，
//! 支持多种显示选项和排序方式。

use crate::cli::TreeArgs;
use crate::config::{Config, SortBy};
use crate::error::{Error, Result};
use crate::utils;
use colored::*;
use humansize::{format_size, BINARY};
use std::fs;
use std::path::Path;
use std::time::SystemTime;
use walkdir::{DirEntry, WalkDir};

/// Implement conversion from walkdir::Error to our custom Error type
impl From<walkdir::Error> for Error {
    fn from(err: walkdir::Error) -> Self {
        Error::Io(err.into())
    }
}

/// Helper extension trait for DirEntry to create entries from paths
trait DirEntryExt {
    /// 从路径创建DirEntry条目
    fn from_path(path: &Path) -> Result<DirEntry>;
}

/// Implementation of the DirEntryExt trait for DirEntry
impl DirEntryExt for DirEntry {
    fn from_path(path: &Path) -> Result<DirEntry> {
        Ok(WalkDir::new(path).into_iter().next().unwrap()?)
    }
}

/// 使用给定的参数和配置执行tree命令
///
/// # 参数
/// * `args` - tree命令的命令行参数
/// * `config` - 命令的配置设置
///
/// # 返回
/// * `Ok(())` 命令执行成功时返回
/// * `Err(Error)` 执行过程中发生错误时返回
pub fn execute(args: &TreeArgs, config: &Config) -> Result<()> {
    let root = &args.path;
    let walker = WalkDir::new(root).max_depth(config.max_depth.unwrap_or(std::usize::MAX));

    let mut entries: Vec<DirEntry> = walker
        .into_iter()
        .filter_entry(|e| filter_entry(e, config))
        .filter_map(|e| e.ok())
        .collect();

    sort_entries(&mut entries, config);

    for (index, entry) in entries.iter().enumerate() {
        let is_last = index == entries.len() - 1;
        print_entry(entry, root, is_last, "", config)?;
    }

    Ok(())
}

/// Determines whether a directory entry should be included in the output
///
/// # Arguments
/// * `entry` - The directory entry to check
/// * `config` - Configuration containing filter settings
///
/// # Returns
/// `true` if the entry should be included, `false` otherwise
fn filter_entry(entry: &DirEntry, config: &Config) -> bool {
    if !config.show_hidden && utils::is_hidden(entry.path()) {
        return false;
    }
    if let Some(ref pattern) = config.pattern {
        entry
            .file_name()
            .to_str()
            .map(|s| s.contains(pattern))
            .unwrap_or(false)
    } else {
        true
    }
}

/// Sorts directory entries according to the configuration
///
/// # Arguments
/// * `entries` - Vector of directory entries to sort
/// * `config` - Configuration containing sort settings
fn sort_entries(entries: &mut Vec<DirEntry>, config: &Config) {
    match config.sort_by {
        SortBy::Type => entries.sort_by_key(|a| !a.file_type().is_dir()),
        SortBy::Size => entries.sort_by_key(|a| a.metadata().map(|m| m.len()).unwrap_or(0)),
        SortBy::Date => entries.sort_by_key(|a| {
            a.metadata()
                .ok()
                .and_then(|m| m.modified().ok())
                .unwrap_or(SystemTime::UNIX_EPOCH)
        }),
        SortBy::Name => {} // Default sorting by name is already done by the file system
    }
}

/// Prints a directory entry with appropriate formatting and metadata
///
/// # Arguments
/// * `entry` - The directory entry to print
/// * `root` - The root path of the tree
/// * `is_last` - Whether this is the last entry in its directory
/// * `prefix` - The prefix to use for this entry (for tree structure)
/// * `config` - Configuration for display options
///
/// # Returns
/// * `Ok(())` if the entry is printed successfully
/// * `Err(Error)` if an error occurs while accessing entry metadata
fn print_entry(
    entry: &DirEntry,
    root: &Path,
    is_last: bool,
    prefix: &str,
    config: &Config,
) -> Result<()> {
    let file_name = entry.file_name().to_string_lossy();
    let depth = entry.depth();

    let new_prefix = if depth == 0 {
        String::new()
    } else if is_last {
        format!("{}└── ", prefix)
    } else {
        format!("{}├── ", prefix)
    };

    let mut line = new_prefix.clone();
    line.push_str(&file_name);

    if entry.file_type().is_dir() {
        line = line.blue().to_string();
    }

    if let Ok(metadata) = entry.metadata() {
        if config.show_permissions {
            line = format!("{} {}", utils::format_permissions(&metadata), line);
        }

        if config.show_size && !entry.file_type().is_dir() {
            let size = metadata.len();
            line = format!("{} {}", line, format_size(size, BINARY).green());
        }

        if config.show_date {
            if let Ok(time) = metadata.modified() {
                let formatted_time = utils::format_time(time);
                line = format!("{} {}", line, formatted_time.yellow());
            }
        }
    }

    println!("{}", line);

    if entry.file_type().is_dir() {
        let new_prefix = if depth == 0 {
            String::new()
        } else if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };

        let dir_entries = fs::read_dir(entry.path())?;
        let mut children: Vec<DirEntry> = Vec::new();
        
        for dir_result in dir_entries {
            if let Ok(dir) = dir_result {
                if let Ok(entry) = DirEntry::from_path(dir.path().as_path()) {
                    if filter_entry(&entry, config) {
                        children.push(entry);
                    }
                }
            }
        }
        
        sort_entries(&mut children, config);

        for (i, child) in children.iter().enumerate() {
            let is_last_child = i == children.len() - 1;
            print_entry(
                &child,
                root,
                is_last_child,
                &new_prefix,
                config,
            )?;
        }
    }

    Ok(())
}
