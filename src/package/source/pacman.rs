//! Pacman integration for package management

use anyhow::{Context, bail};
use log::info;

use crate::package::{
    package::Package,
    source::shell::{format_package_list_for_shell, new_shell_command},
};

const PACMAN_COMMAND_STRING: &'static str = "pacman";

/// Install a list of software using pacman
pub fn install_packages(package_list: &Vec<&Package>) -> anyhow::Result<()> {
    info!("Using pacman to install {} packages", package_list.len());

    // Generate pacman command argument
    let command = format!(
        "{} -Syu --needed {}",
        PACMAN_COMMAND_STRING,
        format_package_list_for_shell(package_list)
    );

    let mut output = new_shell_command().arg(command).spawn().with_context(|| {
        format!(
            "While trying to spawn PACMAN instance to install {} packages using pacman",
            package_list.len()
        )
    })?;

    // Wait for pacman to finish
    let pacman_exit_status = output.wait().with_context(|| {
        format!(
            "While trying to wait for pacman to install {} packages using pacman",
            package_list.len()
        )
    })?;

    // Ensure output status is 0 for good :D
    if !pacman_exit_status.success() {
        bail!(
            "Pacman did not successfully run, returned exit code of {}, aborting",
            pacman_exit_status
        );
    }

    info!(
        "Pacman completed installation, with exit code {}, proceeding",
        pacman_exit_status
    );

    Ok(())
}

/// Remove all non-package list software using pacman
pub fn remove_non_packages(package_list: &Vec<&Package>) -> anyhow::Result<()> {
    info!(
        "Using pacman to ensure only {} packages are installed",
        package_list.len()
    );

    // Step 1, get all software installed on the system
    let command = format!("{} -Qq", PACMAN_COMMAND_STRING);
    let output = new_shell_command().arg(command).output().with_context(|| {
        format!("While trying to spawn pacman to get all packages installed on system")
    })?;

    // Remove all new lines from output
    let all_packages = String::from_utf8(output.stdout)
        .with_context(|| format!("While trying to convert pacman all packages output to string"))?
        .replace('\n', " ");

    info!("Marking all packages as installed as dependency to prime for removal",);

    // Step 2, mark as installed as dependency
    let command = format!("{} -D --asdeps {}", PACMAN_COMMAND_STRING, all_packages);
    let mut output = new_shell_command().arg(command).spawn().with_context(|| {
        format!("While trying to spawn pacman to mark packages installed on system as deps")
    })?;

    // Wait for command to complete.
    let pacman_exit_status = output.wait().with_context(|| {
        format!("While trying to wait for pacman to mark packages installed on system as deps")
    })?;

    // Ensure exit status was okay
    if !pacman_exit_status.success() {
        bail!(
            "Pacman did not successfully run to mark packages installed on system as deps, returned exit code of {}, aborting",
            pacman_exit_status
        );
    }

    info!(
        "Marking {} packages as explicitly installed to keep them on system using pacman",
        package_list.len()
    );

    // Step 3, mark all the package list packages
    let command = format!(
        "{} -D --asexplicit {}",
        PACMAN_COMMAND_STRING,
        format_package_list_for_shell(package_list)
    );
    let mut output = new_shell_command().arg(command).spawn().with_context(|| {
        format!(
            "While trying to spawn pacman to mark packages in blueprint as explicitly installed."
        )
    })?;

    // Wait for command to complete.
    let pacman_exit_status = output.wait().with_context(|| {
        format!(
            "While trying to spawn pacman to mark packages in blueprint as explicitly installed."
        )
    })?;

    // Ensure exit status was okay
    if !pacman_exit_status.success() {
        bail!(
            "Pacman did not successfully run to mark packages in blueprint as explicitly installed. returned exit code of {}, aborting",
            pacman_exit_status
        );
    }

    info!("Getting all orphaned packages");

    // Step 4, Get all orphaned packages
    let command = format!("{} -Qtdq", PACMAN_COMMAND_STRING);
    let output = new_shell_command().arg(command).output().with_context(|| {
        format!("While trying to spawn pacman to get all orphaned packages installed on system")
    })?;

    // Remove all new lines from output
    let orphaned_packages = String::from_utf8(output.stdout)
        .with_context(|| {
            format!("While trying to convert pacman all orphaned packages output to string")
        })?
        .replace('\n', " ");

    // If there are no orphaned packages, we dont need to continue
    if orphaned_packages.is_empty() {
        info!("No orphaned packages found from pacman, nothing needed to be removed, returning");
        return Ok(());
    }

    info!(
        "Removing orphaned packages from pacman: {}",
        orphaned_packages
    );

    // Step 5, Remove all orphaned packages
    let command = format!("{} -Rns {}", PACMAN_COMMAND_STRING, orphaned_packages);
    let mut output = new_shell_command()
        .arg(command)
        .spawn()
        .with_context(|| format!("While trying to spawn pacman to remove orphaned packages"))?;

    // Wait for command to complete
    let pacman_exit_status = output
        .wait()
        .with_context(|| format!("While trying to wait for pacman to remove orphaned packages"))?;

    // Ensure exit status was okay
    if !pacman_exit_status.success() {
        bail!(
            "Pacman did not successfully remove orphaned packages, returned exit code of {}, aborting",
            pacman_exit_status
        );
    }

    info!("Successfully removed orphaned packages, system now contains only specified packages");

    Ok(())
}
