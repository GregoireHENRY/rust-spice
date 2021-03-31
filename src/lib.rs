/*!
# **rust-spice**

WOW! The complete NASA/NAIF Spice toolkit is actually usable on Rust.

## Using **rust-spice**

Simply add the following to your `Cargo.toml` file:

```.ignore
[dependencies]
rust-spice = "0.4.1"
```

Rust layered Spice functions of **rust-spice** are grouped in the root module `spice::` and all
wrapper functions in `spice::c::`.

## **rust-spice** basics

```.ignore
use spice;

// ugly: using binded C Spice.
unsafe {
    let kernel = CString::new("/path/to/metakernel.mk").unwrap().into_raw();
    spice::c::furnsh_c(kernel);
    spice::c::unload_c(kernel);
}

// pretty: using the nice Rust interface.
let mut kernel = spice::Kernel::new("/path/to/metakernels.mk")?;
kernel.unload()?;
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

The Rust layer is nicer to use: no unsafe and easy types. But it takes a long time to write because it
is not automated, I have to do it by hand. I started from my needs, basically getting positions from
start to end date of target in referential. I will implemented a nice Rust layer more and more when
I will be using other functions. You can submit issues if you want a layer to some functionalities,
and we will implement it immediately.

The Rust layer for the most Used C Spice API for accessing kernel data is located in [`core`].

The type [`System`] provides some tools, built on top of spice.
*/

extern crate cspice_sys;
extern crate itertools;
extern crate log;
extern crate nalgebra as na;
extern crate simplelog;

/// Complete NASA/NAIF C Spice binded functions, very unsafe, from [`cspice_sys`] wrapper.
pub mod c {
    pub use cspice_sys::*;
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
