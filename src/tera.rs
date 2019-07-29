//! Tera integration module.
//!
//! To enable Tera integration, specify the `tera` feature when installing.
//!
//! Then, configure your Tera instance using the `configure` function.
//!
//! # Helpers
//!
//! Misto provides the following helper functions that you can use in your Tera templates:
//!
//! ## `asset_path(name)`
//!
//! Takes the source path of an asset as `name` and returns the path to compiled to asset.

use tera as t;

/// Configures the given Tera `instance` with the `asset_path` helper function.
pub fn configure(instance: &mut t::Tera, manifest: crate::manifest::Manifest) {
    instance.register_function("asset_path", make_asset_path_function(manifest))
}

/// Factory function to construct the `asset_path` helper.
fn make_asset_path_function(manifest: crate::manifest::Manifest) -> t::GlobalFn {
    Box::new(move |args| -> t::Result<t::Value> {
        match args.get("name") {
            Some(val) => match t::from_value::<String>(val.clone()) {
                Ok(name) => Ok(t::to_value(
                    manifest.asset_path(&name).ok_or("Unknown asset")?,
                )?),
                Err(e) => Err(e.into()),
            },
            None => Err("Missing argument `name`".into()),
        }
    })
}
