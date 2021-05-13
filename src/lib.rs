/*!
# **rust-spice**

WOW! The complete NASA/NAIF Spice toolkit is actually usable on Rust.

## Using **rust-spice**

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

```.ignore
use spice;

let mut kernel = spice::Kernel::new("/path/to/metakernels.mk")?;

let et = spice::str2et("2027-MAR-23 16:00:00");
let (position, light_time) = spice::spkpos("TARGET_NAME", et, "FRAME_NAME", "NONE", "SUN");

kernel.unload()?;
```

You can also read other [examples](https://github.com/GregoireHENRY/rust-spice/tree/main/examples).

## In development

Developing an idiomatic interface for Spice in Rust takes time, and not all
functions are implemented yet. In the [module `**core**`][core], you will find
a guide detailing which functions are available. If yours is not, you can
always use the [unsafe API][c] which contains all [cspice functions](https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/index.html).

For instance, with the unsafe API, the example above would be,

```.ignore
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
*/
/// Complete NASA/NAIF C Spice binded functions, very unsafe, from [`cspice_sys`] wrapper.

// extern crate cspice_sys;
pub mod c {
    //pub use cspice_sys::*;
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

extern crate itertools;
extern crate nalgebra as na;
extern crate serial_test;
extern crate tool;

/// The Rust layer to ease the use of the wrapper.
pub mod core;
/// Tools developped on top of Spice for even easier usage of the library.
pub mod spicetools;

pub use crate::core::*;
pub use crate::spicetools::*;

/// Convert String to *mut i8
#[macro_export]
macro_rules! cstr {
    ($s:expr) => {{
        std::ffi::CString::new($s).unwrap().into_raw()
    }};
}
