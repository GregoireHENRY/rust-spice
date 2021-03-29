/*!
# **rust-spice**

WOW! The complete NASA/NAIF Spice toolkit is actually usable on Rust.

## Using **kalast**

Simply add the following to your `Cargo.toml` file:

```.ignore
[dependencies]
rust-spice = "0.3.5"
```

Useful functionalities of **rust-spice** are grouped in the root module `spice::`.

## **rust-spice** basics

```.ignore
use spice;

// To load and unload a metakernel with the Rust layer.
spice::load("/path/to/metakernel.mk");
spice::unload("/path/to/metakernel.mk");

// To load and unload a metakernel with the C wrapper.
unsafe {
    let kernel = CString::new("/path/to/metakernel.mk").unwrap();
    spice::c::furnsh_c(kernel.as_ptr() as *mut _);
    spice::c::unload_c(kernel.as_ptr() as *mut _);
}
```

## In action

With this code, you can get the evolution of the distance in the system.

```.ignore
use itertools::multizip;
use spice;

// Define the system.
// The metakernel is automatically loaded.
let mut system = spice::System::new(
    "rsc/data/hera_PO_EMA_2024.tm", // kernel
    "J2000",                        // frame
    "HERA",                         // observer
    "DIMORPHOS",                    // target
    "2027-MAR-23 16:00:00",         // start date
    129.0 * spice::DAY(),           // duration
    "NONE",                         // aberration correction
);

// Define the time step.
let time_step = 1.0 * spice::DAY();

// Get all the times string-formatted.
let times = system.times_formatted(time_step);

// Get all the positions at the times.
let mut positions = system.positions(time_step);

// Get the distances from the positions.
let distances = spice::compute_distances(&mut positions);

// Display
for (time, distance) in multizip((times.iter(), distances.iter())) {
    println!("{} -> {:.2} km", time, distance);
}

// Always unload the kernels.
system.unload();

```

You can also read other [examples](https://github.com/GregoireHENRY/rust-spice/examples).

## Rust layer in development

### Intro to C wrapper

The complete NASA/NAIF C Spice toolkit is accessible under the module `spice::c`. It has been
generated using [`bindgen`](https://docs.rs/bindgen/0.57.0/bindgen/).

If you feel some troubles to use the C interface, you can visit the sources of [`System`]. Most
conversions from pointers to list of doubles or chars, back and forth, are done there. A complete
guide will be written if needed.

### The damn Rust layer

The Rust layer is nicer to use: no unsafe and easy types. But it takes a long time to write because it
is not automated, I have to do it by hand. I started from my needs, basically getting positions from
start to end date of target in referential. I will implemented a nice Rust layer more and more when
I will be using other functions. You can submit issues if you want a layer to some functionalities,
and we will implement it immediately.
*/

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate itertools;
extern crate log;
extern crate nalgebra as na;
extern crate simplelog;

/// Unsafe C Spice wrapped.
pub mod c {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

/// Core features.
pub mod core;
/// Generic functions for math, physics, matrix operations.
#[macro_use]
pub mod toolbox;
/// Some tools developped for quicker Spice usage.
pub mod spicetools;

pub use crate::core::*;
pub use crate::spicetools::*;
pub use crate::toolbox::*;
