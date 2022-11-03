#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/GregoireHENRY/rust-spice/main/rust-spice/rsc/img/logo_squared.png",
    html_favicon_url = "https://raw.githubusercontent.com/GregoireHENRY/rust-spice/main/rust-spice/rsc/img/logo_squared.png"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "default")]
extern crate cspice_sys;

#[cfg(feature = "noclang")]
extern crate cspice_sys_no_clang as cspice_sys;

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

// TODO: We probably want to break the usual API when the lock is enabled to prevent accidental
// usage, but that breaks integration and doc tests
// #[cfg(any(test, not(feature = "lock")))]
pub use crate::core::*;

pub use crate::core::{DLADSC, DSKDSC};

#[cfg(feature = "lock")]
pub use crate::core::lock::SpiceLock;
