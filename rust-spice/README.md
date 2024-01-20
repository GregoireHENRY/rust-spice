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
[Usage](#usage) |
[In action](#in-action) |
[In development](#in-development) |
[Multi-threaded usage](#multi-threaded-usage) |
[Roadmap](#roadmap) |
[Contributors](#contributors) |
[License](#license)

---

## Intro

**SPICE** is *An Observation Geometry System for Space Science Missions*. Visit
their [website][naif link].

## Requirements

1) Install [CSPICE library][cspice install link] for your platform.
2) Set the environment variable `CSPICE_DIR` to your CSPICE installation folder
   (where CSPICE subfolders `include` and `lib` are located. You can do that in the
   [Cargo configuration][config doc]).
3) In the `cspice/lib` folder you might need for Unix systems to rename the
   static library to match standards: `cspice.a` -> `libcspice.a`

See other requirements at [`cspice-sys`][cspice-sys link] library which provides
unsafe bindings to CSPICE.

## Usage

Add the dependency **rust-spice** to your `Cargo.toml`:

```toml
[dependencies]
rust-spice = "*" # replace * by the latest version of the crate
```

[`cspice-sys`][cspice-sys link] library depends on Clang which might not be
available to your system. In this case, you can use the feature `noclang`:

```toml
[dependencies]
rust-spice = {version = "*", default-features = false, features = ["noclang"] }
```

To enable the `lock` feature (see [## Multi-threaded usage](#multi-threaded-usage)).

```toml
[dependencies]
rust-spice = {version = "*", features = ["lock"] }
```

## In action

A nice and idiomatic interface to Spice,

```rust
use spice;

let mut kernel = spice::furnsh("/Users/gregoireh/data/spice-kernels/hera/kernels/mk/hera_study_PO_EMA_2024.tm");

let et = spice::str2et("2027-MAR-23 16:00:00");
let (position, light_time) = spice::spkpos("DIMORPHOS", et, "J2000", "NONE", "SUN");

// position -> 18.62640405424448, 21.054373008357004, -7.136291402940499
// light time -> 0.00009674257074746383

spice::kclear();
```

You can look for some inspirations in the [core tests][core tests link].

This dataset used as an example can be downloaded from
[here](https://s2e2.cosmos.esa.int/bitbucket/projects/SPICE_KERNELS/repos/hera/browse).

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
    let kernel = CString::new("/Users/gregoireh/data/spice-kernels/hera/kernels/mk/hera_study_PO_EMA_2024.tm").unwrap().into_raw();
    spice::c::furnsh_c(kernel);

    let mut et = 0.0;
    let date = CString::new("2027-MAR-23 16:00:00").unwrap().into_raw();
    spice::c::str2et_c(date, &mut et);

    let target_c = CString::new("DIMORPHOS").unwrap().into_raw();
    let frame_c = CString::new("J2000").unwrap().into_raw();
    let abcorr_c = CString::new("NONE").unwrap().into_raw();
    let observer_c = CString::new("SUN").unwrap().into_raw();
    let mut light_time = 0.0;
    let mut position = [0.0, 0.0, 0.0];
    spice::c::spkpos_c(
        target_c,
        et,
        frame_c,
        abcorr_c,
        observer_c,
        &mut position[0],
        &mut light_time,
    );

    spice::c::kclear_c();
}
```

Much less friendly.. yet it is available. I would love some help in order to
complete the idiomatic development. You can raise an issue or propose a pull
request for the implementation of a specific function.

## Multi-threaded usage

CSPICE itself contains massive amounts of shared mutable state and is thus not
thread-safe - concurrent calls to any SPICE functions will almost always lead
to crashes. To prevent this, if you need to call SPICE functions from multiple
threads, this crate provides a thread-safe API with the `lock` feature. When
enabled, the API is exposed in the form of associated functions on a guard
singleton `SpiceLock`, which is `!Sync + Send`. You can then only share this
singleton and thus the methods it provides between threads using a `Mutex`,
preventing concurrent API usage.

The lock exposes the [neat][neat link] versions of functions where available,
and the [raw][raw link] versions for the rest. For functions which have
neither, you will have to use the unsafe (and unguarded) direct C bindings.
Just make sure you have the lock before calling them.

```rust
# #[cfg(feature = "lock")]
# {
use spice::SpiceLock;

// `try_acquire` will return `Err` if a lock already exists
let sl = SpiceLock::try_acquire().unwrap();

// SPICE functions are now associated functions of the lock with a `&self` arg
let mut kernel = sl.furnsh("/Users/gregoireh/data/spice-kernels/hera/kernels/mk/hera_study_PO_EMA_2024.tm");

let et = sl.str2et("2027-MAR-23 16:00:00");
let (position, light_time) = sl.spkpos("DIMORPHOS", et, "J2000", "NONE", "SUN");

sl.kclear();
# }
```

## Roadmap

+ provide a packaging of the test assets
+ complete most-used API
+ complete whole API
+ refactoring of the procedural macros
+ refactoring of Cell

## Contributors

Hall of fame:

+ [@s-rah][s-rah url]: [#2][PR 2]
+ [@pixldemon][pixldemon url]: [#6][PR 6] [#10][PR 10]
+ [@jodavaho][jodavaho url]: None yet!

A huge thanks for their contributions!!

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
[core tests link]: https://github.com/GregoireHENRY/rust-spice/tree/main/rust-spice/tests/core/mod.rs
[naif link]: https://naif.jpl.nasa.gov/naif
[cspice api]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/index.html
[cspice install link]: https://naif.jpl.nasa.gov/naif/toolkit_C.html
[cspice-sys link]: https://github.com/jacob-pro/cspice-rs/tree/master/cspice-sys
[config doc]: https://doc.rust-lang.org/cargo/reference/config.html
[raw link]: https://docs.rs/rust-spice/latest/spice/core/raw/index.html
[neat link]: https://docs.rs/rust-spice/latest/spice/core/neat/index.html

[s-rah url]: https://github.com/s-rah
[pixldemon url]: https://github.com/pixldemon
[PR 2]: https://github.com/GregoireHENRY/rust-spice/pull/2
[PR 6]: https://github.com/GregoireHENRY/rust-spice/pull/6
[PR 10]: https://github.com/GregoireHENRY/rust-spice/pull/10
[jodavaho url]: https://github.com/jodavaho
