#![doc(
    html_logo_url = "https://raw.githubusercontent.com/GregoireHENRY/rust-spice/main/rust-spice/rsc/img/logo_squared.png",
    html_favicon_url = "https://raw.githubusercontent.com/GregoireHENRY/rust-spice/main/rust-spice/rsc/img/logo_squared.png"
)]
// Doc tests in the README fail with 'lock' enabled
#![cfg_attr(not(feature = "lock"), doc = include_str!("../README.md"))]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "noclang")]
extern crate cspice_sys_no_clang as cspice_sys;

/// The string version of **rust-spice**.
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
pub use crate::core::cell::{Cell, CellItem, CELL_MAXID, CELL_MAX_LEN};
pub use crate::core::ffi::{
    UdBail, UdFunb, UdFunc, UdFuns, UdRefn, UdRepf, UdRepi, UdRepu, UdStep,
};
pub use crate::core::raw::{dsk02, ELLIPSE, PLANE};
// GENERATED: the constants of `raw`, which `lock` would otherwise put out of reach.
pub use crate::core::raw::{
    DAF_MAXSUM, DSK02_SPADSZ, DSKXSI_DCSIZE, DSKXSI_ICSIZE, DSK_KEYAMG, DSK_KEYLAL, DSK_KEYPTM,
    DSK_KEYSGR, DSK_KEYSPM, DSK_KEYXFR, DSK_NSYPAR, SPK_DSCSIZ, TLE_NELTS, TLE_NGEOPHS,
};
pub use crate::core::{DLADSC, DSKDSC, MAX_LEN_OUT, TIME_FORMAT, TIME_FORMAT_SIZE};

#[cfg(any(feature = "lock", doc))]
#[cfg_attr(docsrs, doc(cfg(feature = "lock")))]
pub use crate::core::lock::SpiceLock;
