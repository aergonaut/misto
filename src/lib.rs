//! # Misto
//!
//! Misto is a crate providing integration between [Laravel Mix][] and Rust web frameworks. It is
//! framework agnostic, providing a set of tools that can be used to integrate with any framework.
//! It can also provide helpers to integrate with popular template systems Tera.
//!
//! [laravel mix]: https://laravel-mix.com/
//!
//! ## Quick Start
//!
//! This example assumes you are using Tera.
//!
//! ```ignore
//! let mix_manifest = misto::Manifest::from_file("public/mix-manifest.json");
//! let mut tera = compile_templates!("templates/**/*");
//! misto::tera::configure(tera, mix_manifest);
//! ```
//!
//! Then in your template, you can use the `asset_path` helper function to resolve an asset to its
//! versioned path.
//!
//! ```plain
//! <script type="text/javascript" src="{{ asset_path(name="js/app.js") | safe }}"></script>
//! ```
//!
//! `asset_path` takes one argument, `name`, which is the path to the source asset. It returns the
//! versioned asset path from the manifest. If the source path does not correspond to a compiled
//! asset, an error is thrown.

#![deny(missing_docs, unsafe_code, clippy::missing_docs_in_private_items)]

pub mod manifest;
pub use manifest::Manifest;

#[cfg(feature = "tera")]
pub mod tera;
