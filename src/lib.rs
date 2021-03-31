/*!
# **rust-spice**

WOW! The complete NASA/NAIF Spice toolkit is actually usable on Rust.

## Using **rust-spice**

Simply add the following to your `Cargo.toml` file:

```.ignore
[dependencies]
rust-spice = "0.3.5"
```

Rust layered Spice functions of **rust-spice** are grouped in the root module `spice::` and all wrapper functions in `spice::c::`.

## **rust-spice** basics

```.ignore
use spice;

// Example using binded C Spice.
unsafe {
    let kernel = CString::new("/path/to/metakernel.mk").unwrap().into_raw();
    spice::c::furnsh_c(kernel);
    spice::c::unload_c(kernel);
}

// Using the nice Rust interface.
let mut kernel = spice::Kernel::new("/path/to/metakernels.mk")?;
kernel.unload()?;

// If you are curious reading the sources, do not be tempted to use directly `spice::load(..)`.
// The Kernel type adds extra error checks.
```

## In action

With this code, you can get the evolution of the distance in the system.

```.ignore
use itertools::multizip;
use spice;

// Define the system.
let mut system = spice::SystemBuilder::default()
    .kernel("/path/to/metakernels.mk")?
    .frame("J2000")
    .observer("HERA")
    .target("DIMORPHOS")
    .start_date("2027-MAR-23 16:00:00")
    .duration(129.0 * spice::DAY)
    .aberration_correction("NONE")
    .build()?;

// Define the time step.
let time_step = 1.0 * spice::DAY;

// Get all the times string-formatted.
let times = system.times_formatted(time_step);

// Get all the positions at the times.
let positions = system.positions(time_step);

// Get the distances from the positions.
let distances = spice::distance(positions);

// Display
for (time, distance) in multizip((times.iter(), distances.iter())) {
    println!("{} -> {:.2} km", time, distance);
}

// Always unload the kernels.
system.unload()?;
```

You can also read other [examples](https://github.com/GregoireHENRY/rust-spice/tree/main/examples).

## Rust layer in development

### Intro to C wrapper

The complete NASA/NAIF C Spice toolkit is accessible under the module `spice::c`. It has been
generated using [`bindgen`](https://docs.rs/bindgen/0.57.0/bindgen/).

If you feel some troubles to use the C interface, you can visit the sources of [`core`]. Most
conversions from pointers to list of doubles or chars, back and forth, are done there. A complete
guide can be written if needed.

### The damn Rust layer

The Rust layer is nicer to use: no unsafe and easy types. But it takes a long time to write because it
is not automated, I have to do it by hand. I started from my needs, basically getting positions from
start to end date of target in referential. I will implemented a nice Rust layer more and more when
I will be using other functions. You can submit issues if you want a layer to some functionalities,
and we will implement it immediately.

The Rust layer for the most Used C Spice API for accessing kernel data is located in [`core`].

The type [`System`] provides some tools, built on top of spice.
*/

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate itertools;
extern crate log;
extern crate nalgebra as na;
extern crate simplelog;

/// Complete NASA/NAIF C Spice binded functions, very unsafe.
pub mod c {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
/// The Rust layer to ease the use of the wrapper.
pub mod core;
/// Tools developped on top of Spice for even easier usage of the library.
pub mod spicetools;
/// Generic functions for geometry and matrix operations.
#[macro_use]
pub mod toolbox;

pub use crate::core::*;
pub use crate::spicetools::*;
pub use crate::toolbox::*;
