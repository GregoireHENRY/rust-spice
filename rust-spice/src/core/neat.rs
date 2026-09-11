/*!
Improvement on the procedurally generated functions.

## Description

The idiomatic Rust bindings to CSPICE can be very hard to generate in a procedural macro in some
specific cases. You can find, in this module, functions wrapped from [`raw`] to better match
an idiomatic usage. The improvements consists in functions:

+ taking a string as input in C requires to also send the size of the pointer to a char array. In
  Rust, you only send the string.
+ taking an input for the array size and outputting a size, whereas a vector can be used
+ reporting their result by appending to a caller allocated [cell][crate::core::cell::Cell], whereas
  an owned cell can be returned

This is the module the `lock` feature exposes whenever a function has a neat version; the raw
version is exposed for the rest.
*/

use crate::core::cell::{Cell, CELL_MAXID};
use crate::raw;
use crate::MAX_LEN_OUT;

#[cfg(any(feature = "lock", doc))]
use {crate::SpiceLock, spice_derive::impl_for};

/// Default capacity, in double precision numbers, of the coverage windows allocated here.
pub const CELL_MAXWIN: usize = 10_000;

/* -------------------------------------------------------------------------------------------- */
/* Strings sized with [`MAX_LEN_OUT`]                                                             */
/* -------------------------------------------------------------------------------------------- */

/**
Translate the SPICE integer code of a body into a common name for that body.

See [`raw::bodc2n`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn bodc2n(code: i32) -> (String, bool) {
    raw::bodc2n(code, MAX_LEN_OUT as i32)
}

/**
Translate a body ID code to either the corresponding name or, if no name exists, the string
representation of the code.

See [`raw::bodc2s`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn bodc2s(code: i32) -> String {
    raw::bodc2s(code, MAX_LEN_OUT as i32)
}

/**
Look up the name of a reference frame associated with an ID code.

See [`raw::frmnam`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn frmnam(frcode: i32) -> String {
    raw::frmnam(frcode, MAX_LEN_OUT as i32)
}

/**
Convert an input epoch, in ephemeris seconds past J2000, to a UTC string.

See [`raw::et2utc`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn et2utc(et: f64, format: &str, prec: i32) -> String {
    raw::et2utc(et, format, prec, MAX_LEN_OUT as i32)
}

/**
This routine converts an input epoch represented in TDB seconds past the TDB epoch of J2000 to a
character string formatted to the specifications of a user's format picture.

See [`raw::timout`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn timout(et: f64, pictur: &str) -> String {
    raw::timout(et, pictur, MAX_LEN_OUT as i32)
}

/**
Parse a time string, returning the seconds past J2000 and the error message, empty on success.

See [`raw::tparse`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn tparse(string: &str) -> (f64, String) {
    raw::tparse(string, MAX_LEN_OUT as i32)
}

/**
Convert ephemeris seconds past J2000 to a spacecraft clock string.

See [`raw::sce2s`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn sce2s(sc: i32, et: f64) -> String {
    raw::sce2s(sc, et, MAX_LEN_OUT as i32)
}

/**
Convert a double precision encoding of spacecraft clock time into a character representation.

See [`raw::scdecd`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn scdecd(sc: i32, sclkdp: f64) -> String {
    raw::scdecd(sc, sclkdp, MAX_LEN_OUT as i32)
}

/**
Translate a surface ID code, together with a body ID code, to the corresponding surface name.

See [`raw::srfc2s`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn srfc2s(code: i32, bodyid: i32) -> (String, bool) {
    raw::srfc2s(code, bodyid, MAX_LEN_OUT as i32)
}

/**
Translate a surface ID code, together with a body string, to the corresponding surface name.

See [`raw::srfcss`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn srfcss(code: i32, bodstr: &str) -> (String, bool) {
    raw::srfcss(code, bodstr, MAX_LEN_OUT as i32)
}

/**
Return the file name, type, source and handle of a loaded kernel.

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

/**
Return the type, source and handle of a loaded kernel, given its name.

See [`raw::kinfo`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn kinfo(file: &str) -> (String, String, i32, bool) {
    raw::kinfo(file, MAX_LEN_OUT as i32, MAX_LEN_OUT as i32)
}

/**
Return the character values of a kernel pool variable.

See [`raw::gcpool`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn gcpool(name: &str, start: usize, room: usize) -> Vec<String> {
    raw::gcpool(name, start, room, MAX_LEN_OUT)
}

/**
Return the field-of-view parameters of an instrument, given its NAIF ID code.

See [`raw::getfov`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn getfov(instid: i32, room: usize) -> (String, String, [f64; 3], Vec<[f64; 3]>) {
    raw::getfov(instid, room, MAX_LEN_OUT, MAX_LEN_OUT)
}

/* -------------------------------------------------------------------------------------------- */
/* Arrays sized from the file itself                                                              */
/* -------------------------------------------------------------------------------------------- */

/**
Fetch every triangular plate of a type 2 DSK segment.

See [`raw::dskp02`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn dskp02(handle: i32, dladsc: raw::DLADSC) -> Vec<[i32; 3]> {
    let (_nv, np) = raw::dskz02(handle, dladsc);
    raw::dskp02(handle, dladsc, 1, np.max(0) as usize)
}

/**
Fetch every vertex of a type 2 DSK segment.

See [`raw::dskv02`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn dskv02(handle: i32, dladsc: raw::DLADSC) -> Vec<[f64; 3]> {
    let (nv, _np) = raw::dskz02(handle, dladsc);
    raw::dskv02(handle, dladsc, 1, nv.max(0) as usize)
}

/* -------------------------------------------------------------------------------------------- */
/* Cells allocated on the caller's behalf                                                         */
/* -------------------------------------------------------------------------------------------- */

/**
Find the set of body ID codes of all objects for which topographic data are provided in a specified
DSK file.

See [`raw::dskobj`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn dskobj(dsk: &str) -> Cell<i32> {
    let mut bodids = Cell::new(CELL_MAXID);
    raw::dskobj(dsk, &mut bodids);
    bodids
}

/**
Find the set of surface ID codes for all surfaces associated with a given body in a specified DSK
file.

See [`raw::dsksrf`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn dsksrf(dsk: &str, bodyid: i32) -> Cell<i32> {
    let mut srfids = Cell::new(CELL_MAXID);
    raw::dsksrf(dsk, bodyid, &mut srfids);
    srfids
}

/**
Find the set of ID codes of all objects in a specified SPK file.

See [`raw::spkobj`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn spkobj(spk: &str) -> Cell<i32> {
    let mut ids = Cell::new(CELL_MAXID);
    raw::spkobj(spk, &mut ids);
    ids
}

/**
Find the coverage window for a specified ephemeris object in a specified SPK file.

See [`raw::spkcov`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn spkcov(spk: &str, idcode: i32) -> Cell<f64> {
    let mut cover = Cell::new(CELL_MAXWIN);
    raw::spkcov(spk, idcode, &mut cover);
    cover
}

/**
Find the set of ID codes of all objects in a specified CK file.

See [`raw::ckobj`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn ckobj(ck: &str) -> Cell<i32> {
    let mut ids = Cell::new(CELL_MAXID);
    raw::ckobj(ck, &mut ids);
    ids
}

/**
Find the coverage window for a specified object in a specified CK file.

See [`raw::ckcov`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn ckcov(
    ck: &str,
    idcode: i32,
    needav: bool,
    level: &str,
    tol: f64,
    timsys: &str,
) -> Cell<f64> {
    let mut cover = Cell::new(CELL_MAXWIN);
    raw::ckcov(ck, idcode, needav, level, tol, timsys, &mut cover);
    cover
}

/**
Find the coverage window for a specified reference frame in a specified binary PCK file.

See [`raw::pckcov`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn pckcov(pck: &str, idcode: i32) -> Cell<f64> {
    let mut cover = Cell::new(CELL_MAXWIN);
    raw::pckcov(pck, idcode, &mut cover);
    cover
}

/**
Determine the time windows, within a confinement window, when one body is occulted by or in transit
across another, as seen from an observer.

See [`raw::gfoclt`] for the raw interface.
*/
#[allow(clippy::too_many_arguments)]
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn gfoclt(
    occtyp: &str,
    front: &str,
    fshape: &str,
    fframe: &str,
    back: &str,
    bshape: &str,
    bframe: &str,
    abcorr: &str,
    obsrvr: &str,
    step: f64,
    cnfine: &mut Cell<f64>,
) -> Cell<f64> {
    let mut result = Cell::new(CELL_MAXWIN);
    raw::gfoclt(
        occtyp,
        front,
        fshape,
        fframe,
        back,
        bshape,
        bframe,
        abcorr,
        obsrvr,
        step,
        cnfine,
        &mut result,
    );
    result
}

/**
Find the set of reference frame class ID codes of all frames in a specified binary PCK file.

See [`raw::pckfrm`] for the raw interface.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn pckfrm(pck: &str) -> Cell<i32> {
    let mut ids = Cell::new(CELL_MAXID);
    raw::pckfrm(pck, &mut ids);
    ids
}
