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
    Portown,
    // Additional subcommands will be added here as the toolkit expands
}

/// Arguments for the 'tree' subcommand
#[derive(Parser)]
pub struct TreeArgs {
    /// Root directory to start from
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Maximum depth to traverse
    #[arg(short, long)]
    pub depth: Option<i32>,

    /// Show hidden files
    #[arg(short = 'a', long)]
    pub all: bool,

    /// Sort by type (directories first)
    #[arg(short = 't', long)]
    pub sort_type: bool,

    /// Sort by size
    #[arg(short = 's', long)]
    pub sort_size: bool,

    /// Sort by modification date
    #[arg(short = 'D', long)]
    pub sort_date: bool,

    /// Show file permissions
    #[arg(short = 'p', long)]
    pub permissions: bool,

    /// Show file sizes in human-readable format
    #[arg(short = 'S', long)]
    pub human_size: bool,

    /// Show modification dates
    #[arg(short = 'm', long)]
    pub modified: bool,

    /// Filter files by pattern (e.g., *.rs for Rust files)
    #[arg(short = 'f', long)]
    pub filter: Option<String>,
}