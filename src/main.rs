//! Toolboxer - A comprehensive command-line toolkit for developers
//! 
//! This is the main entry point of the application. It handles command-line argument
//! parsing and dispatches commands to their respective handlers.

use clap::Parser;
use toolboxer::cli::{Cli, Commands};
use toolboxer::commands;
use toolboxer::config::{Config, SortBy};

/// Main entry point for the Toolboxer application.
/// 
/// # Error
/// Returns a `toolboxer::Result<()>` which wraps any errors that might occur
/// during the execution of the program.
fn main() -> toolboxer::Result<()> {
    // Parse command line arguments using clap
    let cli = Cli::parse();

    // Match on the subcommand and handle each command
    match &cli.command {
        // Handle the 'tree' subcommand
        Commands::Tree(args) => {
            // Create a new configuration instance with the specified root path
            let mut config = Config::new(args.path.clone());
            
            // Configure the tree display options based on command line arguments
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

            // Determine and set the sorting mode based on command line flags
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

            // Apply file name pattern filter if provided by the user
            if let Some(pattern) = args.filter.clone() {
                config = config.with_pattern(Some(pattern))?;
            }

            // Execute the tree command with the configured options
            commands::execute_tree(args, &config)?;
        }
        // Handle the 'portown' subcommand
        Commands::Portown(args) => {
            commands::execute_portown(args)?;
        }
        // Additional subcommands will be handled here as the toolkit expands
    }

    Ok(())
}