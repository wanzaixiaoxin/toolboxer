//! Command-line interface definitions using clap
//! 
//! This module defines the structure of the command-line arguments
//! and subcommands for the Toolboxer application.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Main command-line interface structure
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Commands,
}

/// Enum representing available subcommands
#[derive(Subcommand)]
pub enum Commands {
    /// Display directory structure as a tree
    Tree(TreeArgs),
    /// Display port ownership information
    Portown(PortownArgs),
    // Additional subcommands will be added here as the toolkit expands
}

/// Arguments for the 'tree' subcommand
#[derive(Parser)]
pub struct TreeArgs {
    /// Root directory to start building the tree from
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Maximum depth to display
    #[arg(short, long)]
    pub max_depth: Option<usize>,

    /// Include hidden files
    #[arg(short, long)]
    pub all: bool,

    /// Show file permissions
    #[arg(short, long)]
    pub permissions: bool,

    /// Show human-readable sizes
    #[arg(long)]
    pub human_size: bool,

    /// Show last modified date
    #[arg(short, long)]
    pub modified: bool,

    /// Sort by type
    #[arg(long)]
    pub sort_type: bool,

    /// Sort by size
    #[arg(long)]
    pub sort_size: bool,

    /// Sort by modification date
    #[arg(long)]
    pub sort_date: bool,

    /// Filter by pattern
    #[arg(short, long)]
    pub filter: Option<String>,
}

/// Arguments for the 'portown' subcommand
#[derive(Parser)]
pub struct PortownArgs {
    /// Show only TCP connections
    #[arg(short = 't', long)]
    pub tcp_only: bool,

    /// Show only UDP connections
    #[arg(short = 'u', long)]
    pub udp_only: bool,

    /// Show only listening ports
    #[arg(short = 'l', long)]
    pub listen_only: bool,

    /// Show only established connections
    #[arg(short = 'e', long)]
    pub established_only: bool,
}