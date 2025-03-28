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

/// 实现从walkdir::Error到自定义错误类型的转换
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
/// * `args` - 包含路径、最大深度、过滤模式等参数的TreeArgs结构体
/// * `config` - 包含显示选项、排序方式、过滤条件等运行时配置
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

/// 判断目录条目是否应包含在输出中
///
/// 过滤逻辑包含：
/// 1. 根据配置隐藏/显示隐藏文件（以点开头的文件）
/// 2. 按文件名模式过滤（当配置包含pattern时）
///
/// # 参数
/// * `entry` - 要检查的目录条目
/// * `config` - 包含过滤设置的配置项（show_hidden、pattern等）
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

/// 根据配置对目录条目进行排序
///
/// # 排序规则
/// - Type: 目录优先排序
/// - Size: 按文件大小升序排列
/// - Date: 按修改时间升序排列
/// - Name: 保持文件系统默认顺序
///
/// # 参数
/// * `entries` - 待排序的目录条目向量
/// * `config` - 包含排序枚举(SortBy)的配置项
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

/// 以带格式的方式打印目录条目及其元数据
///
/// 递归生成子目录时会重新应用：
/// 1. 过滤条件（filter_entry）
/// 2. 排序规则（sort_entries）
/// 3. 当前配置的显示选项
///
/// # 参数
/// * `entry` - 要打印的目录条目
/// * `root` - 目录树的根路径
/// * `is_last` - 当前条目是否为父目录的最后一个子项
/// * `prefix` - 用于构建树状缩进的前缀字符串
/// * `config` - 显示配置（权限、大小、日期等显示选项）
///
/// # 返回值
/// 打印成功返回`Ok(())`，访问元数据出错时返回`Err(Error)`
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
