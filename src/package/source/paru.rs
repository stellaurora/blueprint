//! Paru integration for package management

use anyhow::{Context, bail};
use log::info;

use crate::package::{
    package::Package,
    source::shell::{format_package_list_for_shell, new_shell_command},
};

const PARU_COMMAND_STRING: &'static str = "paru";

/// Install a list of software using paru
pub fn install_packages(package_list: &Vec<&Package>) -> anyhow::Result<()> {
    info!("Using paru to install {} packages", package_list.len());

    // Generate paru command argument
    let command = format!(
        "{} -Syu --needed {}",
        PARU_COMMAND_STRING,
        format_package_list_for_shell(package_list)
    );

    let mut output = new_shell_command().arg(command).spawn().with_context(|| {
        format!(
            "While trying to spawn PARU instance to install {} packages using paru",
            package_list.len()
        )
    })?;

    // Wait for paru to finish
    let paru_exit_status = output.wait().with_context(|| {
        format!(
            "While trying to wait for paru to install {} packages using paru",
            package_list.len()
        )
    })?;

    // Ensure output status is 0 for good :D
    if !paru_exit_status.success() {
        bail!(
            "Paru did not successfully run, returned exit code of {}, aborting",
            paru_exit_status
        );
    }

    info!(
        "Paru completed installation, with exit code {}, proceeding",
        paru_exit_status
    );

    Ok(())
}

/// Remove all non-package list software using paru
pub fn remove_non_packages(package_list: &Vec<&Package>) -> anyhow::Result<()> {
    info!(
        "Using paru to ensure only {} packages are installed",
        package_list.len()
    );

    // Step 1, get all software installed on the system
    let command = format!("{} -Qq", PARU_COMMAND_STRING);
    let output = new_shell_command().arg(command).output().with_context(|| {
        format!("While trying to spawn paru to get all packages installed on system")
    })?;

    // Remove all new lines from output
    let all_packages = String::from_utf8(output.stdout)
        .with_context(|| format!("While trying to convert paru all packages output to string"))?
        .replace('\n', " ");

    info!("Marking all packages as installed as dependency to prime for removal",);

    // Step 2, mark as installed as dependency
    let command = format!("{} -D --asdeps {}", PARU_COMMAND_STRING, all_packages);
    let mut output = new_shell_command().arg(command).spawn().with_context(|| {
        format!("While trying to spawn paru to mark packages installed on system as deps")
    })?;

    // Wait for command to complete.
    let paru_exit_status = output.wait().with_context(|| {
        format!("While trying to wait for paru to mark packages installed on system as deps")
    })?;

    // Ensure exit status was okay
    if !paru_exit_status.success() {
        bail!(
            "Paru did not successfully run to mark packages installed on system as deps, returned exit code of {}, aborting",
            paru_exit_status
        );
    }

    info!(
        "Marking {} packages as explicitly installed to keep them on system using paru",
        package_list.len()
    );

    // Step 3, mark all the package list packages
    let command = format!(
        "{} -D --asexplicit {}",
        PARU_COMMAND_STRING,
        format_package_list_for_shell(package_list)
    );
    let mut output = new_shell_command().arg(command).spawn().with_context(|| {
        format!("While trying to spawn paru to mark packages in blueprint as explicitly installed.")
    })?;

    // Wait for command to complete.
    let paru_exit_status = output.wait().with_context(|| {
        format!("While trying to spawn paru to mark packages in blueprint as explicitly installed.")
    })?;

    // Ensure exit status was okay
    if !paru_exit_status.success() {
        bail!(
            "Paru did not successfully run to mark packages in blueprint as explicitly installed. returned exit code of {}, aborting",
            paru_exit_status
        );
    }

    info!("Getting all orphaned packages");

    // Step 4, Get all orphaned packages
    let command = format!("{} -Qtdq", PARU_COMMAND_STRING);
    let output = new_shell_command().arg(command).output().with_context(|| {
        format!("While trying to spawn paru to get all orphaned packages installed on system")
    })?;

    // Remove all new lines from output
    let orphaned_packages = String::from_utf8(output.stdout)
        .with_context(|| {
            format!("While trying to convert paru all orphaned packages output to string")
        })?
        .replace('\n', " ");

    // If there are no orphaned packages, we dont need to continue
    if orphaned_packages.is_empty() {
        info!("No orphaned packages found from paru, nothing needed to be removed, returning");
        return Ok(());
    }

    info!(
        "Removing orphaned packages from paru: {}",
        orphaned_packages
    );

    // Step 5, Remove all orphaned packages
    let command = format!("{} -Rns {}", PARU_COMMAND_STRING, orphaned_packages);
    let mut output = new_shell_command()
        .arg(command)
        .spawn()
        .with_context(|| format!("While trying to spawn paru to remove orphaned packages"))?;

    // Wait for command to complete
    let paru_exit_status = output
        .wait()
        .with_context(|| format!("While trying to wait for paru to remove orphaned packages"))?;

    // Ensure exit status was okay
    if !paru_exit_status.success() {
        bail!(
            "Paru did not successfully remove orphaned packages, returned exit code of {}, aborting",
            paru_exit_status
        );
    }

    info!("Successfully removed orphaned packages, system now contains only specified packages");

    Ok(())
}
