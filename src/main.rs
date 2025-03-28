//! Toolboxer - 开发者命令行工具集
//!
//! 应用程序主入口点，负责：
//! - 命令行参数解析
//! - 命令路由分发
//! - 整体错误处理

use clap::Parser;
use toolboxer::cli::{Cli, Commands};
use toolboxer::commands;
use toolboxer::config::{Config, SortBy};

/// Toolboxer应用程序主入口
///
/// # 错误处理
/// 返回`toolboxer::Result<()>`封装可能出现的各类错误
fn main() -> toolboxer::Result<()> {
    // 使用clap解析命令行参数
    let cli = Cli::parse();

    // 匹配子命令并路由处理逻辑
    match &cli.command {
        // 处理'tree'目录树子命令
        Commands::Tree(args) => {
            // 创建配置实例并指定根路径
            let mut config = Config::new(args.path.clone());
            
            // 根据命令行参数配置显示选项
            // Set maximum traversal depth if specified
            if let Some(depth) = args.max_depth {
                config = config.with_max_depth(depth.try_into()?)?;
            }
            
            // Configure display options: hidden files, permissions, sizes, and dates
            config = config
                .with_show_hidden(args.all)
                .with_show_permissions(args.permissions)
                .with_show_size(args.human_size)
                .with_show_date(args.modified);

            // 根据命令行标志设置排序模式
            // Priority: type > size > date > name (default)
            let sort_by = if args.sort_type {
                SortBy::Type
            } else if args.sort_size {
                SortBy::Size
            } else if args.sort_date {
                SortBy::Date
            } else {
                SortBy::Name
            };
            config = config.with_sort_by(sort_by);

            // 应用用户提供的文件名过滤模式
            if let Some(pattern) = args.filter.clone() {
                config = config.with_pattern(Some(pattern))?;
            }

            // 使用配置参数执行tree命令
            commands::execute_tree(args, &config)?;
        }
        // 处理'portown'端口占用查询命令
        Commands::Portown(args) => {
            commands::execute_portown(args)?;
        }
        // Additional subcommands will be handled here as the toolkit expands
    }

    Ok(())
}