/*!
# **rust-spice**

NASA/NAIF Spice toolkit usable on Rust

## Using **kalast**

Simply add the following to your `Cargo.toml` file:

```.ignore
[dependencies]
rust-spice = "0.3.0"
```

Useful functionalities of **rust-spice** are grouped in the root module `spice::`.

## **rust-spice** in action

```
use spice;
```

You can also read other [examples](https://github.com/GregoireHENRY/rust-spice/examples).
*/

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate itertools;
extern crate log;
extern crate nalgebra as na;
extern crate simplelog;

/// Core features.
pub mod core;
/// Generic functions for math, physics, matrix operations.
#[macro_use]
pub mod toolbox;

pub use crate::core::*;
pub use crate::toolbox::*;
