/*!
WOW! The complete NASA/NAIF Spice toolkit is actually usable on Rust.

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

Simply add the following to your `Cargo.toml` file:

```.ignore
[dependencies]
rust-spice = "*" // replace * by the latest version of the crate
```

Rust layered Spice functions of **rust-spice** are grouped in the root module `spice::` and all
functions from the FFI are grouped in `spice::c::`.

You can find a guide for the Rust interface [here][crate::core].

## **rust-spice** in action

A nice and idiomatic interface to Spice,

```
use spice;

let mut kernel = spice::furnsh("rsc/krn/hera_study_PO_EMA_2024.tm");

let et = spice::str2et("2027-MAR-23 16:00:00");
let (position, light_time) = spice::spkpos("DIMORPHOS", et, "J2000", "NONE", "SUN");

// position -> 18.62640405424448, 21.054373008357004, -7.136291402940499
// light time -> 0.00009674257074746383

spice::unload("rsc/krn/hera_study_PO_EMA_2024.tm");
```

You can look for some inspirations in the [tests][tests link].

## In development

Developing an idiomatic interface for Spice in Rust takes time, and not all
functions are implemented yet. In the [module `**core**`][core], you will find
a guide with the list of available functions. If yours is not, you can always
use the [unsafe API][c] which contains all
[cspice functions][cspice toolkit link].

For instance, with the unsafe API, the short example above would be,

```
use spice;
use std::ffi::CString;

unsafe {
    let kernel = CString::new("rsc/krn/hera_study_PO_EMA_2024.tm").unwrap().into_raw();
    spice::c::furnsh_c(kernel);

    let mut ephemeris_time = 0.0;
    let date = CString::new("2027-MAR-23 16:00:00").unwrap().into_raw();
    spice::c::str2et_c(date, &mut ephemeris_time);

    let target_c = CString::new("DIMORPHOS").unwrap().into_raw();
    let frame_c = CString::new("J2000").unwrap().into_raw();
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

[tests link]: https://github.com/GregoireHENRY/rust-spice/tree/main/rust-spice/tests
[config doc]: https://doc.rust-lang.org/cargo/reference/config.html
[cspice install link]: https://naif.jpl.nasa.gov/naif/toolkit_C.html
[cspice toolkit link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/index.html
*/

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/GregoireHENRY/rust-spice/main/rust-spice/rsc/img/logo_bg.png"
)]

extern crate cspice_sys;
extern crate itertools;
extern crate nalgebra as na;
extern crate serial_test;
extern crate spice_derive;
extern crate tool;

pub mod c {
    /*!
    Complete NASA/NAIF C SPICE binded functions, very unsafe.
    */
    pub use cspice_sys::*;
}
pub mod core;

pub use crate::core::*;
