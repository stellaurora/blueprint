//! Argument Parsing/help message generation for blueprint using Clap

use std::fmt::Display;

use clap::{Parser, Subcommand};

// Root-arguments for blueprint
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Which operation to run with blueprint
    #[command(subcommand)]
    pub command: Commands,
}

// Enum for commands for different operations within blueprint
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialises a basic template file in the directory
    Init {
        /// Path to the template file to create
        #[arg(short, long, default_value = "blueprint.toml")]
        file: String,
    },

    /// Runs blueprint and ensures all applyed packages only
    /// match the config provided, unapplying all other software
    Apply {
        /// Path to the template file to create
        #[arg(short, long)]
        file: String,

        /// Name of the provided section for
        /// Quill TOML extensions. ALL of the config files
        /// should share this section to minimise confusion.
        #[arg(short, long, default_value = "blueprint")]
        section: String,
    },
}

impl Display for Commands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Should display what type of command.
        match self {
            Commands::Init { .. } => write!(f, "init"),
            Commands::Apply { .. } => write!(f, "apply"),
        }
    }
}

/// Parses arguments to blueprint returning the
/// arguments, exits on error.
pub fn parse_args() -> Args {
    Args::parse()
}
