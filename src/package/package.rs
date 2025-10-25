use std::{
    collections::{HashMap, hash_map::Entry},
    fmt::Display,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use serde::Deserialize;

use crate::{
    cleanpath::CleanPath,
    package::sources::{PackageListFunction, PackageSource},
};

/// An individiual package to have within the system
/// including its dependencies
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Package {
    // Name of the associated package
    pub name: String,

    // Which source to use to apply this package?
    #[serde(default)]
    pub source: PackageSource,

    // What file is this package defined in for debugging info
    #[serde(skip)]
    pub src: PathBuf,
}

/// Newtype to help handling packages better
/// with more methods on a list
#[derive(Deserialize, Debug, Default)]
pub struct PackageList(pub Vec<Package>);

impl Deref for PackageList {
    type Target = Vec<Package>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PackageList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Package {
    /// Creates a new package representation from its components
    /// src file should be configured for debugging into from add_source_debug_path
    pub fn new(name: String, source: PackageSource) -> Self {
        Self {
            name,
            source,
            src: Default::default(),
        }
    }

    /// Adds the path to this package for debugging tracking
    /// of the root pathw
    pub fn add_source_debug_path(self: &mut Self, file_path: &PathBuf) -> anyhow::Result<()> {
        self.src = file_path.clean_path()?;
        Ok(())
    }
}

impl FromIterator<Package> for PackageList {
    fn from_iter<T: IntoIterator<Item = Package>>(iter: T) -> Self {
        // Collect into wrapped form
        let iter_vec: Vec<Package> = iter.into_iter().collect();
        PackageList(iter_vec)
    }
}

impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        // Source file doesn't really matter.
        self.name == other.name && self.source == other.source
    }
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "package {}, from source {}, referenced in configuration file {:?}",
            self.name, self.source, self.src
        )
    }
}

impl PackageList {
    /// Removes all of the duplicate packages inside
    /// of this package list, may error.
    pub fn remove_duplicates(self: Self) -> anyhow::Result<Self> {
        let mut result = Vec::new();

        for ele in self.0 {
            // Check if we've seen this name before
            if result.contains(&ele) {
                continue;
            }

            result.push(ele);
        }

        Ok(Self(result))
    }

    /// Runs a function dependant on the source of packages
    /// on all packages in the source on this package list
    pub fn run_source_function(
        self: &Self,
        source_function: fn(source: &PackageSource) -> anyhow::Result<PackageListFunction>,
    ) -> anyhow::Result<()> {
        let mut source_map: HashMap<PackageSource, Vec<&Package>> = HashMap::new();

        // Split all of the packages based on their source.
        for current_package in &self.0 {
            match source_map.entry(current_package.source) {
                Entry::Occupied(mut occupied_entry) => {
                    // Existing entry, should ve a vec
                    occupied_entry.get_mut().push(current_package);
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(vec![current_package]);
                }
            }
        }

        // Run packagelistfunction now on each
        source_map
            .iter()
            .try_for_each(|(source, package_list)| source_function(source)?(package_list))?;

        Ok(())
    }
}
