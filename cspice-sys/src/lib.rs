#![allow(clippy::redundant_static_lifetimes)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[cfg(not(feature = "generate"))]
mod cspice;
#[cfg(feature = "generate")]
mod cspice {
    include!(concat!(env!("OUT_DIR"), "/cspice.rs"));
}

pub use cspice::*;
