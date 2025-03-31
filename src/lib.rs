//! Toolboxer 库箱
//!
//! 本箱体提供 Toolboxer 命令行工具包的核心功能
//! 包含 CLI 参数解析、命令执行、配置管理、
//! 错误处理和工具函数等模块

/// 命令行界面定义和参数解析
pub mod cli;
/// 各类 Toolboxer 功能的命令执行逻辑
pub mod commands;
/// 配置结构和相关方法
pub mod config;
/// 错误类型和结果定义
pub mod error;
/// 跨应用程序使用的工具函数
pub mod utils;

// 为方便使用重新导出错误类型
pub use error::{Error, Result};

// 重新导出常用项以便快速访问
pub use config::Config;
