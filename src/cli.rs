//! 基于clap的命令行界面定义
//! 
//! 本模块定义Toolboxer应用程序的命令行参数结构
//! 以及各个子命令的配置项。

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// 主命令行接口结构
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}


/// 枚举表示可用的子命令
#[derive(Subcommand)]
pub enum Commands {
    /// 以树状结构显示目录
    Tree(TreeArgs),
    /// 显示端口占用信息
    Portown(PortownArgs),
}
    // Additional subcommands will be added here as the toolkit expands


/// 'tree'子命令的参数
#[derive(Parser)]
pub struct TreeArgs {
    /// 生成目录树的根路径
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// 最大显示深度
    #[arg(short = 'd', long)]
    pub max_depth: Option<usize>,

    /// 包含隐藏文件
    #[arg(short, long)]
    pub all: bool,

    /// 显示文件权限
    #[arg(short, long)]
    pub permissions: bool,

    /// 显示文件大小（自动转换单位）
    #[arg(short = 's', long)]
    pub size: bool,

    /// 显示最后修改时间
    #[arg(short = 'M', long = "mod")]
    pub modified: bool,

    /// 按类型排序
    #[arg(short = 'T', long = "type")]
    pub type_sort: bool,

    /// 按大小排序
    #[arg(short = 'S', long = "sort-size")]
    pub size_sort: bool,

    /// 按修改时间排序
    #[arg(short = 't', long = "date")]
    pub date_sort: bool,

    /// 按模式过滤文件
    #[arg(short, long)]
    pub filter: Option<String>,
    /// 仅显示目录结构
    #[arg(short = 'D', long = "dirs")]
    pub directories_only: bool,
}



/// 'portown'子命令的参数
#[derive(Parser)]
pub struct PortownArgs {
    /// 显示监听状态端口
    #[arg(short, long)]
    pub listen: bool,

    /// 显示TCP连接
    #[arg(short = 't', long = "tcp")]
    pub tcp: bool,

    /// 显示UDP连接
    #[arg(short = 'u', long = "udp")]
    pub udp: bool,

    /// 设置显示深度（进程树层级）
    #[arg(short = 'd', long)]
    pub depth: Option<usize>,

    /// 仅显示已建立的连接
    #[arg(short = 'e', long)]
    pub established_only: bool,
}
