#![doc = include_str!("../../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/GregoireHENRY/rust-spice/main/rust-spice/rsc/img/logo_squared.png",
    html_favicon_url = "https://raw.githubusercontent.com/GregoireHENRY/rust-spice/main/rust-spice/rsc/img/logo_squared.png"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate cspice_sys;
extern crate itertools;
extern crate libc;
extern crate nalgebra as na;
extern crate serial_test;
extern crate spice_derive;

/// The string version of **kalast**.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod c {
    /*!
    Complete NASA/NAIF C SPICE binded functions, very unsafe.
    */
    pub use cspice_sys::*;
}
pub mod core;

pub use crate::core::*;
