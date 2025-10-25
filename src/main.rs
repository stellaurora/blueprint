use ::log::{debug, error};

use crate::{
    commands::{apply::apply_command, init::init_command},
    log::setup_logging,
};

// Automatic argument handling
mod args;

// Different types of commands
mod commands;

// Configuration related modules
mod config;
mod parse_config;

// Logging handling
mod log;

// Package handling related module
mod package;

// Path cleaning
mod cleanpath;

fn main() {
    setup_logging();

    // Parse arguments from CLI
    let args = args::parse_args();
    debug!("blueprint running command: {}", args.command);

    // Run correct command for the type.
    let command_result = match args.command {
        args::Commands::Init { file } => init_command(file),
        args::Commands::Apply { file, section } => apply_command(file, section),
    };

    // Use error logger to print error..
    let _ = command_result.inspect_err(|err| {
        error!("{:?}", err);
    });
}
