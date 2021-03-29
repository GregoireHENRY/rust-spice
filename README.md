# rust-spice

[![logo image]][crate link]

[![crate badge]][crate link]
[![doc badge]][doc link]
[![license badge]][license link]
[![pre-commit badge]][pre-commit link]

> NASA/NAIF Spice toolkit actually usable on Rust

---

[Rational](#rational) |
[Installation](#installation) |
[License](#license)

---

## Rational

Another crate already exist for wrapping C spice: [spice-sys][spice-sys link].
The motivation behind the creation of [rust-spice][crate link] was 1) the need
to access the complete spice API from Rust and 2) the need to have a Rust layer
to feel natural to use for Rust users.

### Complete API

[spice-sys][spice-sys link] does not use [bindgen][bindgen link] to wrap
  which is error-prone and does not provide a complete API. The description of
  spice-sys says:
  > *Currently, it provides only the most common API calls as outlined
  [here][cspice most common].*

  This boosted me to build a complete C spice wrapper.

### Rust interface

[spice-sys][spice-sys link] does not provide a Rust user-friendly interface,
which constrains the user to use FFI/libc tools to convert strings, floats and
array to C types and using unsafe scopes.

I wanted to use spice from Rust without feeling it is a C wrapper, for quicker
and simplier usage in my crates.

### S/O

Many thanks to the [author][author spice-sys] of the crate
[spice-sys][spice-sys link] for the idea of the wget script to avoid crate
upload, I included your script copyrighted under your MIT license and added some
modifications that I listed carefully.

## Installation

Add the dependency **rust-spice** to your `Cargo.toml`:

```toml
...
[dependencies]
rust-spice = "0.3.0"
```

## License

Licensed under the [Apache License, Version 2.0][license link].

[repository link]: https://github.com/GregoireHENRY/rust-spice
[logo image]: rsc/img/rust-spice-logo.png
[crate link]: https://crates.io/crates/rust-spice
[crate badge]: https://meritbadge.herokuapp.com/rust-spice?style=flat-square
[doc link]: https://docs.rs/rust-spice
[doc badge]: https://docs.rs/rust-spice/badge.svg
[license link]: LICENSE
[license badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[pre-commit link]: https://pre-commit.com
[pre-commit badge]: https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white
[spice-sys link]: https://crates.io/crates/spice-sys
[author spice-sys]: https://github.com/rjpower4
[cspice most common]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/info/mostused.html
[bindgen link]: https://crates.io/crates/bindgen
