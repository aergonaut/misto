# Misto

**Misto** is a crate providing integration between [Laravel Mix][] and Rust web
frameworks. It is framework agnostic, and simply provides a set of tools that
can be used to integrate with any framework.

[laravel mix]: https://laravel-mix.com/

## Installation

To use Misto, you must have Laravel Mix installed in your project.

Add the following to your `Cargo.toml`:

```toml
[dependencies]
misto = "^0.1.0"
```

## Framework Integration

Misto supports Tera and Handlebars via optional features. To enable either of
these integrations, specify the `tera` or `handlebars` features when installing
the crate

```toml
[dependencies]
misto = { version = "^0.1.0", features = ["tera"] }
```

## Usage

First, construct a `Manifest` instance from your `mix-manifest.json` file. This
file is generated by Laravel Mix at build time and contains a mapping between
your asset file names and the paths of the compiled assets. This is important
when using cache busting so that your templates don't need to be updated when
the asset fingerprint changes.

```rust
let mix_manifest = misto::Manifest::from_file("public/mix-manifest.json");
```

Replace `public/mix-manifest.json` with the path to your manifest file.

If using the Tera or Handlebars integrations, Misto provides a helper function
that you can use to automatically configure the template instance with Misto's
helper functions.

The following example shows this using Tera:

```rust
let mut tera = compile_templates!("templates/**/*");
misto::tera::configure(tera, mix_manifest);
```

Finally, in your templates, refer to your assets using the `asset_path` helper:

```tera
<script type="text/javascript" src="{{ asset_path(name="js/main.js") | safe }}"></script>
```

## License

Licensed under either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
