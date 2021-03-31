# rust-spice

[![logo image]][crate link]

[![crate badge]][crate link]
[![doc badge]][doc link]
[![license badge]][license link]
[![pre-commit badge]][pre-commit link]
[![coverage doc badge]][coverage doc link]
[![coverage test badge]][coverage test link]

> WOW! The complete NASA/NAIF Spice toolkit is actually usable on Rust

---

[In action](#in-action) |
[Installation](#installation) |
[License](#license)

---

## In action

Quick example showing the simplicity of using Spice in Rust.

```Rust
use spice;

// ugly and unsafe: using binded C Spice.
unsafe {
    let kernel = CString::new("/path/to/metakernel.mk").unwrap().into_raw();
    spice::c::furnsh_c(kernel);
    spice::c::unload_c(kernel);
}

// pretty: using the nice Rust interface.
let mut kernel = spice::Kernel::new("/path/to/metakernels.mk")?;
kernel.unload()?;
```

Read more in the documentation [online][doc link].

## Installation

Add the dependency **rust-spice** to your `Cargo.toml`:

```toml
...
[dependencies]
rust-spice = "0.4.1"
```

## License

Licensed under the [Apache License, Version 2.0][license link].

[repository link]: https://github.com/GregoireHENRY/rust-spice
[logo image]: https://github.com/GregoireHENRY/rust-spice/blob/main/rsc/img/rust-spice-logo.png
[crate link]: https://crates.io/crates/rust-spice
[crate badge]: https://meritbadge.herokuapp.com/rust-spice?style=flat-square
[doc link]: https://docs.rs/rust-spice
[doc badge]: https://docs.rs/rust-spice/badge.svg
[license link]: https://github.com/GregoireHENRY/rust-spice/blob/main/LICENSE
[license badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[pre-commit link]: https://pre-commit.com
[pre-commit badge]: https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white
[coverage doc badge]: https://img.shields.io/badge/Documentation-100%25-brightgreen
[coverage doc link]: https://docs.rs/crate/rust-spice
[coverage test badge]: https://img.shields.io/badge/Tests-90%25-green
[coverage test link]: https://docs.rs/crate/rust-spice
