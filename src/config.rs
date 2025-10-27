//! Configuration structs and helpers for blueprint

use std::sync::OnceLock;

use serde::Deserialize;

use crate::{
    package::{package::PackageList, sources::PackageSource},
    parse_config::{ConfigLink, MultiplePackage},
};

/// Wrapper around oncelock config to help
/// retrieving config options globally.
pub struct GlobalConfig(OnceLock<Config>);

// Configuration from the root file oncelock that will be
// filled in once the config has been gotten
pub static ROOT_CONFIG: GlobalConfig = GlobalConfig(OnceLock::new());

/// Wrapper with helper methods for interacting
/// with a list of blueprint configs
pub struct BlueprintConfigs(pub Vec<Blueprint>);

/// Configuration for the a file in the blueprint system
///
/// config is not utilised outside of the root
/// file referenced directly by commands.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Blueprint {
    // Global blueprint configuration options.
    pub config: Option<Config>,

    // Links to other files to include in the configuration
    #[serde(rename = "link", default)]
    pub links: Vec<ConfigLink>,

    // Packages for keeping track of package management
    #[serde(rename = "package", default)]
    pub packages: PackageList,

    // Shorthand for multiple packages under one source
    #[serde(rename = "multi_packages", default)]
    pub multi_packages: Vec<MultiplePackage>,
}

/// Global blueprint configuration options.
///
/// Can only be used by the root blueprint
/// configuration file referenced in commands
/// in order to keep tracking configuration simple
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    // Which shell to use for running package manager related commands?
    // this should be posix compliant
    #[serde(default = "default_shell")]
    pub shell: String,

    // Argument to pass to shell for it to be capable of running commands
    // will be the first argument, and commands needed to be executed
    // as the second argument
    #[serde(default = "default_shell_exec_arg")]
    pub shell_exec_arg: String,

    // Default source to pull packages from on the system
    #[serde(default)]
    pub default_source: PackageSource,

    // Confirm inside of blueprint whether or not to
    // continue the apply operation?
    #[serde(default = "default_is_true")]
    pub prompt_apply_continue: bool,

    // Confirm for each installation source whether to
    // proceed with installing
    #[serde(default = "default_is_true")]
    pub prompt_install_per_source: bool,

    // Confirm for each installation source whether to
    // proceed with removing uneeded packages
    #[serde(default = "default_is_true")]
    pub prompt_removal_per_source: bool,

    // Global toggle for removing software determined
    // as "unrequired"
    #[serde(default = "default_is_true")]
    pub remove_unrequired_software: bool,
}

fn default_is_true() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            shell: default_shell(),
            shell_exec_arg: default_shell_exec_arg(),
            prompt_apply_continue: default_is_true(),
            prompt_install_per_source: default_is_true(),
            prompt_removal_per_source: default_is_true(),
            remove_unrequired_software: default_is_true(),
            default_source: PackageSource::default(),
        }
    }
}

fn default_shell_exec_arg() -> String {
    String::from("-c")
}

fn default_shell() -> String {
    String::from("bash")
}

impl FromIterator<Blueprint> for BlueprintConfigs {
    fn from_iter<T: IntoIterator<Item = Blueprint>>(iter: T) -> Self {
        // Collect into wrapped form
        let iter_vec: Vec<Blueprint> = iter.into_iter().collect();
        BlueprintConfigs(iter_vec)
    }
}

impl BlueprintConfigs {
    /// Decomposes down all of the blueprint configs
    /// into their useful data as lists.
    pub fn flatten_data(self: Self) -> PackageList {
        // Decompose each config and collect all of the packages
        self.0
            .into_iter()
            .flat_map(|config| config.packages.0)
            .collect()
    }
}

impl GlobalConfig {
    /// Set's the global config
    /// in the system to be this config
    pub fn set_config(self: &Self, global_config: Config) {
        ROOT_CONFIG.0.get_or_init(|| global_config);
    }

    /// Get's the root config
    /// or returns an error if it could not succesfully be gotten
    pub fn get_config(self: &Self) -> &Config {
        ROOT_CONFIG.0.wait()
    }

    /// Get's whether or not the root config is initialised
    pub fn is_initialised(self: &Self) -> bool {
        ROOT_CONFIG.0.get().is_some()
    }
}
