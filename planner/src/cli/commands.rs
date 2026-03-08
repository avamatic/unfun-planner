use std::path::PathBuf;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "fnv-planner", about = "Fallout: New Vegas character build planner")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Simulate a character build from a plan file
    Simulate {
        /// Path to game data JSON (from xNVSE extractor)
        #[arg(long)]
        data: PathBuf,

        /// Path to build plan TOML file
        #[arg(long)]
        plan: PathBuf,

        /// Output format
        #[arg(long, default_value = "table")]
        format: OutputFormat,
    },

    /// Display information about extracted game data
    Info {
        /// Path to game data JSON
        #[arg(long)]
        data: PathBuf,

        /// What to display
        #[command(subcommand)]
        topic: InfoTopic,
    },

    /// Generate a template build plan TOML file
    Init {
        /// Path to game data JSON
        #[arg(long)]
        data: PathBuf,

        /// Output file path
        #[arg(long, default_value = "build.toml")]
        output: PathBuf,
    },
}

#[derive(Subcommand)]
pub enum InfoTopic {
    /// List all skills
    Skills,
    /// List all perks
    Perks,
    /// List all traits
    Traits,
    /// List all implants
    Implants,
    /// Show leveling formulas
    Leveling,
}

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
}
