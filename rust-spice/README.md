# rust-spice

[![logo image]][repository link]

[![crate badge]][crate link]
[![doc badge]][doc link]
[![license badge]][license link]
[![pre-commit badge]][pre-commit link]
[![coverage doc badge]][coverage doc link]
[![coverage test badge]][coverage test link]

> **WOW! The complete NASA/NAIF Spice toolkit is actually usable on Rust**

---

[Intro](#intro) |
[Requirements](#requirements) |
[In action](#in-action) |
[In development](#in-development) |
[Usage](#usage) |
[Roadmap](#roadmap) |
[Contributors](#contributors) |
[License](#license)

---

## Intro

**SPICE** is *An Observation Geometry System for Space Science Missions*. Visit
their [website][naif link].

## Requirements

1) Install [CSPICE library][cspice install link] for your platform.
2) In your folder `/path/to/cspice/lib`, rename the static libraries to match standards:
    1) `cspice.a` -> `libcspice.a`
    2) `csupport.a` -> `libcsupport.a`
3) Tell Cargo where to look for the CSPICE library. This is done by adding some
lines to `$HOME/.cargo/config.toml`. If the file doesn't exist, create it (read
[Configuration doc][config doc]). You need to write:

```toml
[target.YOUR_PLATFORM.cspice]
rustc-link-lib = ["cspice"]
rustc-link-search = ["/path/to/cspice/lib"]
rustc-cdylib-link-arg = ["-I/path/to/cspice/include"]
```

replace `YOUR_PLATFORM` by either:

+ for linux: `x86_64-unknown-linux-gnu`
+ for mac: `x86_64-apple-darwin`
+ for windows: `x86_64-pc-windows-msvc`

and replace `/path/to/cspice` with the absolute path to your CSPICE installation.

## Usage

Add the dependency **rust-spice** to your `Cargo.toml`:

```toml
...
[dependencies]
rust-spice = "*" # replace * by the latest version of the crate
```

## In action

A nice and idiomatic interface to Spice,

```rust
use spice;

let mut kernel = spice::furnsh("hera/kernels/mk/hera_study_PO_EMA_2024.tm");

let et = spice::str2et("2027-MAR-23 16:00:00");
let (position, light_time) = spice::spkpos("DIMORPHOS", et, "J2000", "NONE", "SUN");

// position -> 18.62640405424448, 21.054373008357004, -7.136291402940499
// light time -> 0.00009674257074746383

spice::unload("hera/kernels/mk/hera_study_PO_EMA_2024.tm");
```

You can look for some inspirations in the [tests][tests link].

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

Much less friendly.. yet it is available. I would love some help in order to
complete the idiomatic development. You can raise an issue or propose a pull
request for the implementation of a specific function.

## Roadmap

+ provide a packaging of the test assets
+ complete most-used API
+ complete whole API
+ refactoring of the procedural macros
+ refactoring of Cell

## Contributors

Hall of fame:

+ [@s-rah][s-rah url]: [#2][PR 2]

A huge thanks for their contributions!!

In addition, this crate is based on the unsafe bindings to CSPICE provided by
@jacob-pro in [`cspice-sys`][cspice-sys link].

## License

Licensed under the [Apache License, Version 2.0][license link].

[repository link]: https://github.com/GregoireHENRY/rust-spice
[logo image]: https://raw.githubusercontent.com/GregoireHENRY/rust-spice/main/rust-spice/rsc/img/logo_bg.png
[crate link]: https://crates.io/crates/rust-spice
[crate badge]: https://img.shields.io/crates/v/rust-spice.svg
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
[tests link]: https://github.com/GregoireHENRY/rust-spice/tree/main/rust-spice/tests
[naif link]: https://naif.jpl.nasa.gov/naif
[cspice api]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/index.html
[cspice install link]: https://naif.jpl.nasa.gov/naif/toolkit_C.html
[cspice-sys link]: https://github.com/jacob-pro/cspice-rs/tree/master/cspice-sys
[config doc]: https://doc.rust-lang.org/cargo/reference/config.html

[s-rah url]: https://github.com/s-rah
[PR 2]: https://github.com/GregoireHENRY/rust-spice/pull/2
