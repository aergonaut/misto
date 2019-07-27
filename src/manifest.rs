//! See the `Manifest` struct for more info.

use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// The `Manifest` struct is used to read the contents of the mix-manifest.json file generated
/// during asset compilation. This file maintains the mapping between the source file names and the
/// versioned paths of the compiled files.
#[derive(Deserialize)]
pub struct Manifest {
    /// The map from source paths to versioned paths.
    entries: HashMap<String, String>,
}

impl Manifest {
    /// Read an asset manifest from the given `path`.
    ///
    /// # Errors
    ///
    /// Errors if the file path could not be read.
    pub fn from_file<I: AsRef<Path>>(path: &I) -> Result<Manifest, Box<Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let entries = serde_json::from_reader(reader)?;

        Ok(Manifest { entries: entries })
    }

    /// Translate a given asset `name` into a versioned asset path using the manifest.
    pub fn asset_path(&self, name: &str) -> Option<&str> {
        self.entries.get(name).map(|p| p.as_str())
    }
}
