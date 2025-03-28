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