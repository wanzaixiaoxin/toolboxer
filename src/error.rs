//! Toolboxer 应用错误处理模块
//!
//! 本模块定义了自定义错误类型和Result类型别名
//! 用于统一管理应用程序中的错误处理。

use std::path::PathBuf;
use thiserror::Error;

/// Toolboxer 操作的自定义Result类型
pub type Result<T> = std::result::Result<T, Error>;

/// Enum representing various error types that can occur in Toolboxer
#[derive(Error, Debug)]
pub enum Error {
    /// 表示I/O错误
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// 表示整型转换错误
    #[error("Integer conversion error: {0}")]
    IntConversion(#[from] std::num::TryFromIntError),

    /// 表示访问文件/目录时的错误
    #[error("Failed to access path: {}", .0.display())]
    PathAccess(PathBuf),

    /// 表示提供无效深度值时的错误
    #[error("Invalid depth value: {0}")]
    InvalidDepth(i32),

    /// 表示文件模式匹配相关错误
    #[error("Pattern error: {0}")]
    Pattern(String),

    /// 表示其他未指定错误
    #[error("Unknown error: {0}")]
    Other(String),
}