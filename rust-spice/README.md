# rust-spice

[![logo image]][repository link]

[![ci badge]][ci link]
[![crate badge]][crate link]
[![doc badge]][doc link]
[![license badge]][license link]
[![pre-commit badge]][pre-commit link]
[![coverage doc badge]][coverage doc link]
[![coverage test badge]][coverage test link]

> **WOW! The complete NASA/NAIF Spice toolkit is actually usable on Rust**
>
> *The Rust wrapper now covers 100% of CSPICE N0067: all 644 routines that can be called,
> every one of them tested.*

---

[Intro](#intro) |
[Requirements](#requirements) |
[Usage](#usage) |
[In action](#in-action) |
[Coverage](#coverage) |
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

On Linux and macOS, [`.github/install-cspice.sh`][install script link] does all three for you:

```bash
./.github/install-cspice.sh ~/cspice
export CSPICE_DIR=~/cspice
```

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

```rust,no_run
use spice;

spice::furnsh("/path/to/metakernel.tm");

let et = spice::str2et("2027-MAR-23 16:00:00");
let (position, light_time) = spice::spkpos("DIMORPHOS", et, "J2000", "NONE", "SUN");

// position -> 18.62640405424448, 21.054373008357004, -7.136291402940499
// light time -> 0.00009674257074746383

spice::kclear();
```

You can look for some inspirations in the [tests][core tests link]. They are self contained: the
kernels they need are generated in a temporary directory when the suite starts, so `cargo test`
works on any machine with CSPICE installed.

### Errors

CSPICE keeps its own error state, and out of the box a failing routine prints a report to the
screen and then **terminates the process**. Ask it to return instead, and turn what it reports into
a Rust `Result`:

```rust,no_run
use spice;

spice::errors::quiet();

spice::furnsh("/path/to/metakernel.tm");
if let Err(error) = spice::errors::check() {
    eprintln!("could not load the kernels: {error}");
}
```

### Cells

A few routines report their results by filling a SPICE *cell*, a fixed capacity array the toolkit
manages itself. `spice::Cell` owns its storage, so it is created and dropped like any other Rust
value:

```rust,no_run
use spice;

spice::furnsh("/path/to/metakernel.tm");

// The convenience form allocates the cell for you.
let bodies = spice::dskobj("/path/to/shape.bds");
for id in bodies.iter() {
    println!("{}", spice::bodc2n(id).0);
}

// The raw form takes the cell you sized, exactly as CSPICE does.
let mut ids = spice::Cell::<i32>::new(64);
spice::raw::spkobj("/path/to/ephemeris.bsp", &mut ids);

spice::kclear();
```

## Coverage

The whole toolkit is wrapped: all 644 routines of CSPICE N0067 that can be
called. They cover ephemerides, orientation, shape models, frames, time and
spacecraft clocks, coordinates, vector and matrix algebra, rotations and
quaternions, planes and ellipses, two-body orbits, two-line elements, cells,
sets and windows, the geometry finder, events kernels, the kernel pool and
error handling. The [documentation online][doc link] indexes every one of them,
and every one of them is called by a test.

The headers declare 649 `*_c` functions: four are private internals whose names
begin with `zz`, and `prefix_c` is declared but never compiled into the library
NAIF ships, so it cannot be called at all.

The unsafe [cspice functions][cspice api] are still there for anything you
would rather drive yourself. The example above would be,

```rust,no_run
use spice;
use std::ffi::CString;

unsafe {
    let kernel = CString::new("/path/to/metakernel.tm").unwrap().into_raw();
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

```rust,no_run
# #[cfg(feature = "lock")]
# {
use spice::SpiceLock;

// `try_acquire` will return `Err` if a lock already exists
let sl = SpiceLock::try_acquire().unwrap();

// SPICE functions are now associated functions of the lock with a `&self` arg
sl.furnsh("/path/to/metakernel.tm");

let et = sl.str2et("2027-MAR-23 16:00:00");
let (position, light_time) = sl.spkpos("DIMORPHOS", et, "J2000", "NONE", "SUN");

sl.kclear();
# }
```

## Roadmap

Done: the test suite builds the kernels it needs rather than shipping them, the
whole API is wrapped, the procedural macros were rewritten, and `Cell` owns its
storage.

Next: idiomatic forms for the routines that still ask the caller for a buffer
size, and a guard that can be shared rather than moved.

## Contributors

Hall of fame:

+ [@s-rah][s-rah url]: [#2][PR 2]
+ [@mclrc][mclrc url]: [#6][PR 6] [#10][PR 10]
+ [@jodavaho][jodavaho url]: None yet!

A huge thanks for their contributions!!

## License

Licensed under the [Apache License, Version 2.0][license link].

[ci link]: https://github.com/GregoireHENRY/rust-spice/actions/workflows/ci.yml
[ci badge]: https://github.com/GregoireHENRY/rust-spice/actions/workflows/ci.yml/badge.svg
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
[coverage test badge]: https://img.shields.io/badge/Tests-100%25-brightgreen
[coverage test link]: https://docs.rs/crate/rust-spice
[core tests link]: https://github.com/GregoireHENRY/rust-spice/tree/main/rust-spice/tests
[naif link]: https://naif.jpl.nasa.gov/naif
[cspice api]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/index.html
[cspice install link]: https://naif.jpl.nasa.gov/naif/toolkit_C.html
[cspice-sys link]: https://github.com/jacob-pro/cspice-rs/tree/master/cspice-sys
[config doc]: https://doc.rust-lang.org/cargo/reference/config.html
[install script link]: https://github.com/GregoireHENRY/rust-spice/blob/main/.github/install-cspice.sh
[raw link]: https://docs.rs/rust-spice/latest/spice/core/raw/index.html
[neat link]: https://docs.rs/rust-spice/latest/spice/core/neat/index.html

[s-rah url]: https://github.com/s-rah
[mclrc url]: https://github.com/mclrc
[jodavaho url]: https://github.com/jodavaho
[PR 2]: https://github.com/GregoireHENRY/rust-spice/pull/2
[PR 6]: https://github.com/GregoireHENRY/rust-spice/pull/6
[PR 10]: https://github.com/GregoireHENRY/rust-spice/pull/10
