/*!
Improvement on the procedurally generated functions.

## Description

The idiomatic Rust bindings to CSPICE can be very hard to generate in a procedural macro in some
specific cases. You can find, in this module, functions wrapped from [`raw`] to better match
an idiomatic usage. The improvements consists in functions:

+ taking a string as input in C requires to also send the size of the pointer to a char array. In Rust, you
  only send the string.
+ taking taking input for array size and outputing size whereas a vector can be used
+ which outputs string that be allocated from default length sometimes
*/

use crate::raw;
use crate::MAX_LEN_OUT;
#[cfg(any(feature = "lock", doc))]
use {crate::SpiceLock, spice_derive::impl_for};

/**
Translate the SPICE integer code of a body into a common name for that body.

See [`raw::bodc2n`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn bodc2n(code: i32) -> (String, bool) {
    raw::bodc2n(code, MAX_LEN_OUT as i32)
}

/**
Compute the local solar time for a given ephemeris epoch `et'
for an object on the surface of a body at a specified longitude.

See [`raw::et2lst`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn et2lst(
    et: f64,
    body_code: i32,
    lon: f64,
    lon_type: &str,
) -> (i32, i32, i32, String, String) {
    raw::et2lst(
        et,
        body_code,
        lon,
        lon_type,
        MAX_LEN_OUT as i32,
        MAX_LEN_OUT as i32,
    )
}

/**
This routine converts an input epoch represented in TDB seconds past the TDB epoch of J2000 to a
character string formatted to the specifications of a user's format picture.

See [`raw::timout`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn timout(et: f64, pictur: &str) -> String {
    raw::timout(et, pictur, pictur.len())
}

/**
Fetch triangular plates from a type 2 DSK segment.

See [`raw::dskp02`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn dskp02(handle: i32, dladsc: raw::DLADSC) -> Vec<[i32; 3]> {
    let (_nv, np) = raw::dskz02(handle, dladsc);
    raw::dskp02(handle, dladsc, 1, np as _)
}

/**
Fetch vertices from a type 2 DSK segment.

See [`raw::dskv02`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn dskv02(handle: i32, dladsc: raw::DLADSC) -> Vec<[f64; 3]> {
    let (nv, _np) = raw::dskz02(handle, dladsc);
    raw::dskv02(handle, dladsc, 1, nv as _)
}

/**
Fetch vertices from a type 2 DSK segment.

See [`raw::kdata`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn kdata(which: i32, kind: &str) -> (String, String, String, i32, bool) {
    raw::kdata(
        which,
        kind,
        MAX_LEN_OUT as i32,
        MAX_LEN_OUT as i32,
        MAX_LEN_OUT as i32,
    )
}
