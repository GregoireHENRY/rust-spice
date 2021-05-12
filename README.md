# rust-spice

[![logo image]][crate link]

[![crate badge]][crate link]
[![doc badge]][doc link]
[![license badge]][license link]
[![pre-commit badge]][pre-commit link]
[![coverage doc badge]][coverage doc link]
[![coverage test badge]][coverage test link]

> WOW! The complete **NASA/NAIF Spice** toolkit is actually usable on Rust

---

[Intro](#intro) |
[Temporary brief notice](#temporary-brief-notice) |
[In action](#in-action) |
[In development](#in-development) |
[Installation](#installation) |
[License](#license)

---

## Intro

**SPICE** is *An Observation Geometry System for Space Science Missions*. Visit
their [website][naif link].

## Temporary brief notice

Apparently the crate is only available for Linux systems. It is due to the fact
that the crate is built on top of the wrapper [cspice-sys][cspice-sys link].
This problem will be solved in short delays as I plan to remove this dependency
and write a build script that uses the cspice library installed on the user's
pc.

## In action

A nice and idiomatic interface to Spice,

```rust
use spice;

let mut kernel = spice::Kernel::new("/path/to/metakernels.mk")?;

let et = spice::str2et("2027-MAR-23 16:00:00");
let (position, light_time) = spice::spkpos(
    "TARGET_NAME", et, "FRAME_NAME", "NONE", "SUN"
);

kernel.unload()?;
```

Read more in the [documentation online][doc link] and see
[examples][examples link].

## In development

Developing an idiomatic interface for Spice in Rust takes time, and not all
functions are implemented yet. In the [documentation online][doc link], a
complete guide details which functions are available. If yours is not, you can
always use the unsafe API which contains all [cspice functions][cspice api].

For instance, with the unsafe API, the example above would be,

```rust
use spice;
use std::ffi::CString;

unsafe {
    let kernel = CString::new("/path/to/metakernel.mk").unwrap().into_raw();
    spice::c::furnsh_c(kernel);

    let mut ephemeris_time = 0.0;
    let date = CString::new("2027-MAR-23 16:00:00").unwrap().into_raw();
    spice::c::str2et_c(date, &mut ephemeris_time);

    let target_c = CString::new("TARGET_NAME").unwrap().into_raw();
    let frame_c = CString::new("FRAME_NAME").unwrap().into_raw();
    let abcorr_c = CString::new("NONE").unwrap().into_raw();
    let observer_c = CString::new("SUN").unwrap().into_raw();
    let mut light_time = 0.0;
    let mut position = [0.0, 0.0, 0.0];
    spice::c::spkpos_c(
        target_c,
        ephemeris_time,
        frame_c,
        abcorr_c,
        observer_c,
        &mut position[0],
        &mut light_time,
    );

    spice::c::unload_c(kernel);
}
```

Much less friendly, but it's available. I would love some help in order to
complete the idiomatic development. You can raise an issue or propose a pull
request for the implementation of a specific function.

## Installation

Add the dependency **rust-spice** to your `Cargo.toml`:

```toml
...
[dependencies]
rust-spice = "*" # replace * by the latest version of the crate
```

## License

Licensed under the [Apache License, Version 2.0][license link].

[repository link]: https://github.com/GregoireHENRY/rust-spice
[old logo image]: https://raw.githubusercontent.com/GregoireHENRY/rust-spice/main/rsc/img/rust-spice-logo.png
[logo image]: rsc/img/logo_bg.png
[crate link]: https://crates.io/crates/rust-spice
[crate badge]: https://meritbadge.herokuapp.com/rust-spice?style=flat-square
[doc link]: https://docs.rs/rust-spice
[doc badge]: https://docs.rs/rust-spice/badge.svg
[license link]: https://raw.githubusercontent.com/GregoireHENRY/rust-spice/main/LICENSE
[license badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[pre-commit link]: https://pre-commit.com
[pre-commit badge]: https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white
[coverage doc badge]: https://img.shields.io/badge/Documentation-100%25-brightgreen
[coverage doc link]: https://docs.rs/crate/rust-spice
[coverage test badge]: https://img.shields.io/badge/Tests-90%25-green
[coverage test link]: https://docs.rs/crate/rust-spice
[examples link]: https://github.com/GregoireHENRY/rust-spice/tree/main/examples
[naif link]: https://naif.jpl.nasa.gov/naif
[cspice api]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/index.html
[cspice-sys link]: https://crates.io/crates/cspice-sys/0.0.1
