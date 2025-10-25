//! Helpers for running shell commands for different sources

use std::process::Command;

use crate::{config::ROOT_CONFIG, package::package::Package};

/// Returns a new shell Command for
/// extending for running commands using
/// the configurated shell
pub fn new_shell_command() -> Command {
    // Get root config for specific command
    let root_conf = ROOT_CONFIG.get_config();
    let mut new_command = Command::new(&root_conf.shell);
    new_command.arg(&root_conf.shell_exec_arg);

    new_command
}

/// Formats the supplied package list as a string
/// for input to commands in the shell.
pub fn format_package_list_for_shell(package_list: &Vec<&Package>) -> String {
    package_list
        .iter()
        .map(|package| format!("{} ", package.name))
        .collect::<String>()
}
