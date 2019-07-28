//! # Misto
//!
//! Misto is a crate providing integration between [Laravel Mix][] and Rust web frameworks. It is
//! framework agnostic, providing a set of tools that can be used to integrate with any framework.
//! It can also provide helpers to integrate with popular template systems Tera and Handlebars.
//!
//! [laravel mix]: https://laravel-mix.com/
//!
//! ## Quick Start
//!
//! ```ignore
//! let mix_manifest = misto::Manifest::from_file("public/mix-manifest.json");
//! let mut tera = compile_templates!("templates/**/*");
//! misto::tera::configure(tera, mix_manifest);
//! ```
//!
//! This example assumes you are using Tera.

#![deny(missing_docs, unsafe_code, clippy::missing_docs_in_private_items)]

pub mod manifest;
pub use manifest::Manifest;

#[cfg(feature = "tera")]
pub mod tera;
