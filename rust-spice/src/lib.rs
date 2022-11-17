#![doc(
    html_logo_url = "https://raw.githubusercontent.com/GregoireHENRY/rust-spice/main/rust-spice/rsc/img/logo_squared.png",
    html_favicon_url = "https://raw.githubusercontent.com/GregoireHENRY/rust-spice/main/rust-spice/rsc/img/logo_squared.png"
)]
// Doc tests in the README fail with 'lock' enabled
#![cfg_attr(not(feature = "lock"), doc = include_str!("../README.md"))]
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

// The unguarded API should only be exposed if the lock is disabled
#[cfg(not(feature = "lock"))]
pub mod core;
#[cfg(not(feature = "lock"))]
pub use crate::core::*;
// If it is enabled, only this crate should see it
#[cfg(feature = "lock")]
pub(crate) mod core;
#[cfg(feature = "lock")]
pub(crate) use crate::core::*;

// These items need to be exposed regardless of whether 'lock' is enabled or not
pub use crate::core::{DLADSC, DSKDSC, MAX_LEN_OUT, TIME_FORMAT, TIME_FORMAT_SIZE};

#[cfg(any(feature = "lock", doc))]
#[cfg_attr(docsrs, doc(cfg(feature = "lock")))]
pub use crate::core::lock::SpiceLock;
