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
    pub fn from_file<I: AsRef<Path>>(path: I) -> Result<Manifest, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let entries = serde_json::from_reader(reader)?;

        Ok(Manifest::new(entries))
    }

    /// Construct a new `Manifest` from the given `entries`. This function should normally not be
    /// used. Use `from_file` instead.
    pub fn new(entries: HashMap<String, String>) -> Manifest {
        Manifest { entries: entries }
    }

    /// Translate a given asset `name` into a versioned asset path using the manifest.
    pub fn asset_path(&self, name: &str) -> Option<&str> {
        self.entries.get(name).map(|p| p.as_str())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_asset_path() {
        let mut entries = HashMap::new();
        entries.insert(
            "js/app.js".to_owned(),
            "public/js/app-06fd465b135293c687c152ec96567f5d.js".to_owned(),
        );

        let manifest = Manifest::new(entries);

        assert_eq!(
            manifest.asset_path("js/app.js"),
            Some("public/js/app-06fd465b135293c687c152ec96567f5d.js")
        )
    }
}
