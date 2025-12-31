mod config;
mod paths;
mod sorter;
mod utils;
mod watcher;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// 🧤 Alfred - Your personal file organizer
#[derive(Parser)]
#[command(name = "alfred")]
#[command(author = "Gregory Corin")]
#[command(version = "0.1.0")]
#[command(about = "Automatically organizes your folders", long_about = None)]
struct Cli {
    /// Custom path to watch instead of Downloads
    #[arg(short, long, value_name = "FOLDER")]
    path: Option<PathBuf>,

    /// Simulate actions without actually moving files
    #[arg(long)]
    dry_run: bool,

    /// Path to a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Clean and organize all existing files
    Clean,
    /// Watch for new files and organize them automatically
    Watch,
}

fn main() {
    let cli = Cli::parse();

    // Load config (placeholder for now, will use cli.config later)
    let app_config = config::load_config();

    // Resolve target path (custom or default Downloads)
    let target_path = match cli.path {
        Some(p) => {
            if !p.exists() {
                utils::error(&format!("Path does not exist: {}", p.display()));
                std::process::exit(1);
            }
            p
        }
        None => paths::get_downloads_path().expect("Failed to resolve downloads path"),
    };

    // Show dry-run warning if enabled
    if cli.dry_run {
        utils::warn("🧪 Dry-run mode enabled - no files will be moved");
    }

    // Show config path if provided
    if let Some(config_path) = &cli.config {
        utils::info(&format!(
            "Using config: {}",
            utils::highlight(&config_path.display().to_string())
        ));
    }

    match cli.command {
        Some(Commands::Clean) => {
            utils::info("🧤 Alfred - Clean Mode");
            utils::info(&format!(
                "Target folder: {}",
                utils::highlight(&target_path.display().to_string())
            ));
            sorter::scan_and_sort(&target_path, cli.dry_run);
        }
        Some(Commands::Watch) => {
            utils::info("🧤 Alfred - Watch Mode");
            utils::info(&format!(
                "Watching folder: {}",
                utils::highlight(&target_path.display().to_string())
            ));
            watcher::start_watching(target_path, app_config, cli.dry_run);
        }
        None => {
            // Default: Scan + Watch
            utils::info("🧤 Alfred starting...");
            utils::info(&format!(
                "Target folder: {}",
                utils::highlight(&target_path.display().to_string())
            ));
            sorter::scan_and_sort(&target_path, cli.dry_run);
            watcher::start_watching(target_path, app_config, cli.dry_run);
        }
    }
}
