//! Ensures software configuration matches exactly as defined by the package list
//! provided to the command

use std::path::PathBuf;

use anyhow::bail;
use inquire::Confirm;
use log::info;

use crate::{
    cleanpath::CleanPath, config::ROOT_CONFIG, package::sources::PackageSource,
    parse_config::parse_config,
};

/// Questions the user whether or not to continue the apply based on
/// the configuration
fn continue_apply_prompt(num_packages: usize) -> anyhow::Result<bool> {
    if !ROOT_CONFIG.get_config().prompt_apply_continue {
        info!("Running apply on {} packages", num_packages);
        return Ok(true);
    }

    Ok(
        Confirm::new(format!("Run apply on {} packages?", num_packages).as_str())
            .with_default(true)
            .prompt()?,
    )
}

pub fn apply_command(file: String) -> anyhow::Result<()> {
    // Validate file path
    let path = PathBuf::from(file).clean_path()?;

    // Parse configs to config structs.
    let (root, configs) = parse_config(path)?;

    // Fill in global root config from root
    let global_config = root.config.unwrap_or_default();
    ROOT_CONFIG.set_config(global_config);

    // Re-get the config for usage later
    let config = ROOT_CONFIG.get_config();

    // Grab all of the packages
    let mut package_list = configs.flatten_data();
    package_list.extend(root.packages.0.into_iter());
    package_list = package_list.remove_duplicates()?;

    if !continue_apply_prompt(package_list.len())? {
        bail!("Aborting apply operation");
    }

    // First, ensure all software is installed before removing non-required
    package_list.run_source_function(PackageSource::install_all_packagelist)?;

    if config.remove_unrequired_software {
        // Then, remove all non-required software from the system.
        package_list.run_source_function(PackageSource::remove_all_non_packagelist)?;
    }

    Ok(())
}
