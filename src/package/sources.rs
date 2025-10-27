//! Provider for basic package source related things
//! which are essentially hooks into system package managers

use std::fmt::Display;

use inquire::Confirm;
use serde::Deserialize;

use crate::{
    config::ROOT_CONFIG,
    package::{
        package::Package,
        source::{pacman, paru},
    },
};

#[derive(Deserialize, Debug, PartialEq, Copy, Clone, Hash, Eq, PartialOrd, Ord)]
pub enum PackageSource {
    // Use the Arch Linux "Paru" package manager as the source
    // for these packages
    #[serde(rename = "archlinux-paru")]
    ArchParu,

    // Use the Arch Linux "Pacman" package manager as the source
    // for these packages
    #[serde(rename = "archlinux-pacman")]
    ArchPacman,
}

impl Display for PackageSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageSource::ArchParu => write!(f, "Arch-Linux using Paru"),
            PackageSource::ArchPacman => write!(f, "Arch-Linux using Pacman"),
        }
    }
}

/// Function types for generic operation on a package.
pub type PackageListFunction = fn(package: &Vec<&Package>) -> anyhow::Result<()>;

/// Empty for "nothing" results
pub fn empty_package_list_fn(_: &Vec<&Package>) -> anyhow::Result<()> {
    Ok(())
}

impl Default for PackageSource {
    fn default() -> Self {
        Self::ArchPacman
    }
}

impl PackageSource {
    /// Should return the appropriate function
    /// to remove all non-packagelist packages.
    pub fn remove_all_non_packagelist(self: &Self) -> anyhow::Result<PackageListFunction> {
        if ROOT_CONFIG.get_config().prompt_removal_per_source {
            let continue_this =
                Confirm::new(format!("Continue with removing packages from source {}?, No will skip to the next source", self).as_str())
                    .with_default(true)
                    .prompt()?;

            if !continue_this {
                return Ok(empty_package_list_fn);
            }
        }

        match self {
            PackageSource::ArchParu => Ok(paru::remove_non_packages),
            PackageSource::ArchPacman => Ok(pacman::remove_non_packages),
        }
    }

    /// Should return the appropriate function
    /// to ensure all packagelist packages are intalled
    pub fn install_all_packagelist(self: &Self) -> anyhow::Result<PackageListFunction> {
        // Confirm or skip this source
        if ROOT_CONFIG.get_config().prompt_install_per_source {
            let continue_this =
                Confirm::new(format!("Continue with installing packages from source {}?, No will skip to the next source", self).as_str())
                    .with_default(true)
                    .prompt()?;

            if !continue_this {
                return Ok(empty_package_list_fn);
            }
        }

        Ok(match self {
            PackageSource::ArchParu => paru::install_packages,
            PackageSource::ArchPacman => pacman::install_packages,
        })
    }
}
