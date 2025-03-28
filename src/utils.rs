//! Toolboxer应用程序的工具函数模块
//!
//! 本模块提供常用功能的辅助函数，包括：
//! 权限格式化、文件元数据处理和输出着色等功能。

use colored::Colorize;
use std::fs::{self, Metadata};
use std::path::Path;



/// 将文件权限格式化为字符串（例如："rwxr--r--"）
///
/// # 参数
/// * `metadata` - 包含权限信息的文件元数据
///
/// # 返回值
/// 文件权限的字符串表示
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

/// 将系统时间格式化为字符串
///
/// # 参数
/// * `time` - 需要格式化的系统时间
///
/// # 返回值
/// 时间的字符串表示（自纪元起的秒数）
pub fn format_time(time: std::time::SystemTime) -> String {
    use std::time::UNIX_EPOCH;
    time.duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "Unknown".to_string())
}

/// 记录命令执行指标（包含耗时和状态）
///
/// # 参数
/// * `command` - 正在执行的命令
/// * `duration` - 执行耗时（毫秒）
/// * `status` - 执行状态（成功/失败）
/// * `output_size` - 可选输出大小（字节数）
#[allow(unused_variables)]
pub fn log_command_metrics(command: &str, _duration: u128, _status: &str, _output_size: Option<usize>) {
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

/// 判断文件或目录是否隐藏
///
/// 跨平台实现：
/// - Windows系统：检查隐藏属性
/// - 类Unix系统：检查名称是否以点开头
///
/// # 参数
/// * `path` - 需要检查的路径
///
/// # 返回值
/// 如果隐藏返回`true`，否则返回`false`
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

/// 根据类型为文件/目录名称着色
///
/// # 参数
/// * `name` - 需要着色的名称
/// * `is_dir` - 是否表示目录
///
/// # 返回值
/// 着色后的字符串表示
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


pub fn is_directory(path: &Path) -> std::io::Result<bool> {
    path.metadata().map(|md| md.is_dir())
}
