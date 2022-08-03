/*!
A Rust idiomatic CSPICE wrapper built with [procedural macros][`spice_derive`].
*/

use crate::{c, cstr, fcstr, get_scalar, get_varr, init_scalar, malloc, mallocstr, mptr};
use spice_derive::{cspice_proc, return_output, return_result};
use std::ops::{Deref, DerefMut};

#[allow(clippy::upper_case_acronyms)]
pub type DLADSC = c::SpiceDLADescr;
#[allow(clippy::upper_case_acronyms)]
pub type DSKDSC = c::SpiceDSKDescr;
#[allow(clippy::upper_case_acronyms)]
pub type CELL = c::SpiceCell;
pub const CELL_MAXID: usize = 10_000;

/**
A cell is a data structure intended to provide safe array access within the applications.

See the [C documentation](https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/req/cells.html).
*/
#[derive(Debug)]
pub struct Cell(c::SpiceCell);

impl Cell {
    /**
    Declare a cell from integer.
    */
    pub fn new_int() -> Self {
        let base = malloc!(i32, CELL_MAXID + c::SPICE_CELL_CTRLSZ as usize);
        Self(CELL {
            dtype: c::_SpiceDataType_SPICE_INT,
            length: 0i32,
            size: CELL_MAXID as i32,
            card: 0i32,
            isSet: 1i32,
            adjust: 0i32,
            init: 0i32,
            base: base as *mut libc::c_void,
            data: base.wrapping_add(c::SPICE_CELL_CTRLSZ as usize) as *mut libc::c_void,
        })
    }

    /**
    Declare data from a cell at index.
    */
    pub fn get_data_int(&self, index: usize) -> i32 {
        unsafe { *(self.data as *mut i32).wrapping_add(index) }
    }
}

impl Deref for Cell {
    type Target = c::SpiceCell;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Cell {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

cspice_proc! {
    /**
    Translate the SPICE integer code of a body into a common name for that body.

    This function has a [neat version][crate::neat::bodyc2n].
    */
    pub fn bodc2n(code: i32, lenout: i32) -> (String, bool) {}
}

cspice_proc! {
    /**
    Determine whether values exist for some item for any body in the kernel pool.
    */
    #[return_output]
    pub fn bodfnd(body: i32, item: &str) -> bool {}
}

cspice_proc! {
    /**
    Translate the name of a body or object to the corresponding SPICE integer ID code.
    */
    pub fn bodn2c(name: &str) -> (i32, bool) {}
}

cspice_proc! {
    /**
    close a das file.
    */
    pub fn dascls(handle: i32) {}
}

cspice_proc! {
    /**
    Open a DAS file for reading.
    */
    pub fn dasopr(fname: &str) -> i32 {}
}

cspice_proc! {
    /**
    Begin a forward segment search in a DLA file.
    */
    pub fn dlabfs(handle: i32) -> (DLADSC, bool) {}
}

cspice_proc! {
    /**
    Return the DSK descriptor from a DSK segment identified  by a DAS handle and DLA descriptor.
    */
    pub fn dskgd(handle: i32, dladsc: DLADSC) -> DSKDSC {}
}

cspice_proc! {
    /**
    Compute the unit normal vector for a specified plate from a type 2 DSK segment.
    */
    pub fn dskn02(handle: i32, dladsc: DLADSC, plid: i32) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Find the set of body ID codes of all objects for which topographic data are provided in a
    specified DSK file.
    */
    pub fn dskobj(dsk: &str) -> Cell {}
}

/**
Fetch triangular plates from a type 2 DSK segment.

This function has a [neat version][crate::neat::dskp02].
*/
pub fn dskp02(handle: i32, mut dladsc: DLADSC, start: i32, room: i32) -> (i32, Vec<[i32; 3]>) {
    let mut varout_0 = init_scalar!();
    let varout_1 = malloc!([i32; 3], room);
    unsafe {
        crate::c::dskp02_c(handle, &mut dladsc, start, room, mptr!(varout_0), varout_1);
        (
            get_scalar!(varout_0),
            get_varr!(varout_1, get_scalar!(varout_0)),
        )
    }
}

/**
Fetch vertices from a type 2 DSK segment.

This function has a [neat version][crate::neat::dskv02].
*/
pub fn dskv02(handle: i32, mut dladsc: DLADSC, start: i32, room: i32) -> (i32, Vec<[f64; 3]>) {
    let mut varout_0 = init_scalar!();
    let varout_1 = malloc!([f64; 3], room);
    unsafe {
        crate::c::dskv02_c(handle, &mut dladsc, start, room, mptr!(varout_0), varout_1);
        (
            get_scalar!(varout_0),
            get_varr!(varout_1, get_scalar!(varout_0)),
        )
    }
}

cspice_proc! {
    /**
    Determine the plate ID and body-fixed coordinates of the intersection of a specified ray with
    the surface defined by a type 2 DSK plate model.
    */
    pub fn dskx02(
        handle: i32,
        dladsc: DLADSC,
        vertex: [f64; 3],
        raydir: [f64; 3],
    ) -> (i32, [f64; 3], bool) {
    }
}

cspice_proc! {
    /**
    Return plate model size parameters---plate count and vertex count---for a type 2 DSK segment.
    */
    pub fn dskz02(handle: i32, dladsc: DLADSC) -> (i32, i32) {}
}

cspice_proc! {
    /**
    Retrieve or set the default error action.
    */
    pub fn erract(op: &str, actlen: i32, action: &str) {}
}

cspice_proc! {
    /**
    Return SPICETRUE if an error condition has been signaled via sigerr_c. failed_c is the CSPICE status indicator.
    */
    #[return_output]
    pub fn failed() -> bool {}
}

cspice_proc! {
    /**
    Convert geodetic coordinates to rectangular coordinates.
     */
    pub fn georec(lon: f64, lat: f64, alt: f64, re: f64, f: f64) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Retrieve the current short error message, the explanation of the short error message, or the long error message.
    */
    pub fn getmsg(option: &str, msglen: i32) -> String {}
}

cspice_proc! {
    /**
    Compute the illumination angles---phase, incidence, and emission---at a specified point on a
    target body. Return logical flags indicating whether the surface point is visible from the
    observer's position and whether the surface point is illuminated.

    The target body's surface is represented using topographic data provided by DSK files, or by a
    reference ellipsoid.

    The illumination source is a specified ephemeris object.
    */
    pub fn illumf(
        method: &str,
        target: &str,
        ilusrc: &str,
        et: f64,
        fixref: &str,
        abcorr: &str,
        obsrvr: &str,
        spoint: [f64; 3],
    ) -> (f64, [f64; 3], f64, f64, f64, bool, bool) {
    }
}

cspice_proc! {
    /**
    Load one or more SPICE kernels into a program.
    */
    #[return_result]
    pub fn furnsh(name: &str) {}
}

cspice_proc! {
    /**
    Clear the KEEPER subsystem: unload all kernels, clear the kernel pool, and re-initialize the
    subsystem. Existing watches on kernel variables are retained.
    */
    pub fn kclear() {}
}

/**
Fetch vertices from a type 2 DSK segment.

This function has a [neat version][crate::neat::kdata].
*/
pub fn kdata(
    which: i32,
    kind: &str,
    fillen: i32,
    typlen: i32,
    srclen: i32,
) -> (String, String, String, i32, bool) {
    #[allow(unused_unsafe)]
    unsafe {
        let varout_0 = mallocstr!(fillen);
        let varout_1 = mallocstr!(typlen);
        let varout_2 = mallocstr!(srclen);
        let mut varout_3 = 0i32;
        let mut varout_4 = 0i32;
        crate::c::kdata_c(
            which,
            cstr!(kind),
            fillen,
            typlen,
            srclen,
            varout_0,
            varout_1,
            varout_2,
            &mut varout_3,
            &mut varout_4,
        );
        (
            fcstr!(varout_0),
            fcstr!(varout_1),
            fcstr!(varout_2),
            varout_3,
            varout_4 != 0,
        )
    }
}

cspice_proc! {
    /**
    Return the current number of kernels that have been loaded via the KEEPER interface that are of
    a specified type.
    */
    pub fn ktotal(kind: &str) -> i32 {}
}

cspice_proc! {
    /**
    Convert from latitudinal coordinates to rectangular coordinates.
    */
    pub fn latrec(radius: f64, longitude: f64, latitude: f64) -> [f64; 3] {}
}

cspice_proc! {
    /**
       Multiply a 3x3 double precision matrix with a 3-dimensional double precision vector.
    */
    pub fn mxv(m1: [[f64; 3]; 3], vin: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Determines the occultation condition (not occulted, partially, etc.) of one target relative to
    another target as seen by an observer at a given time, with targets modeled as points,
    ellipsoids, or digital shapes (DSK)
    */
    #[allow(clippy::too_many_arguments)]
    pub fn occult(
        targ1: &str,
        shape1: &str,
        frame1: &str,
        targ2: &str,
        shape2: &str,
        frame2: &str,
        abcorr: &str,
        obsrvr: &str,
        et: f64,
    ) -> i32 {
    }
}

cspice_proc! {
    /**
    Return the matrix that transforms position vectors from one specified frame to another at a
    specified epoch.
    */
    pub fn pxform(from: &str, to: &str, et: f64) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Return the 3x3 matrix that transforms position vectors from one specified frame at a specified
    epoch to another specified frame at another specified epoch.
    */
    pub fn pxfrm2(from: &str, to: &str, etfrom: f64, etto: f64) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Convert range, right ascension, and declination to rectangular coordinates
     */
    pub fn radrec(range: f64, ra: f64, dec: f64) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Convert rectangular coordinates of a point to range, azimuth and elevation.
    */
    pub fn recazl(rectan: [f64; 3], azccw: bool, elplsz: bool) -> (f64, f64, f64) {}
}

cspice_proc! {
    /**
    Convert rectangular coordinates to range, right ascension, and declination.
    */
    pub fn recrad(rectan: [f64; 3]) -> (f64, f64, f64) {}
}

cspice_proc! {
    /**
    Reset the CSPICE error status to a value of "no error." As a result, the status routine, failed_c, will return a value of SPICEFALSE
    */
    pub fn reset() {}
}

cspice_proc! {
    /**
    Close a SPK file opened for read or write.
    */
    pub fn spkcls(handle: i32) {}
}

cspice_proc! {
    /**
    Create a new SPK file, returning the handle of the opened file
     */
    pub fn spkopn(fname: &str, ifname: &str, ncomch: i32) -> i32 {}
}

cspice_proc! {
    /**
    Write a type 9 segment to an SPK file.
     */
    #[allow(clippy::too_many_arguments)]
    pub fn spkw09(handle: i32, body: i32, center: i32, frame: &str, first: f64, last: f64, segid: &str, degree: i32, n: i32, states: &mut [[f64; 6]], epochs: &mut [f64]) {}
}

cspice_proc! {
    /**
    Return the position of a target body relative to an observing body, optionally corrected for
    light time (planetary aberration) and stellar aberration.
    */
    pub fn spkpos(targ: &str, et: f64, frame: &str, abcorr: &str, obs: &str) -> ([f64; 3], f64) {}
}

cspice_proc! {
    /**
      Return the state (position and velocity) of a target body
      relative to an observing body, optionally corrected for light
      time (planetary aberration) and stellar aberration.
    */
    pub fn spkezr(targ: &str, et: f64, frame: &str, abcorr: &str, obs: &str) -> ([f64; 6], f64) {}
}

cspice_proc! {
    /**
    Convert a string representing an epoch to a double precision value representing the number of
    TDB seconds past the J2000 epoch corresponding to the input epoch.
    */
    pub fn str2et(targ: &str) -> f64 {}
}

/**
This routine converts an input epoch represented in TDB seconds past the TDB epoch of J2000 to a
character string formatted to the specifications of a user's format picture.

This function has a [neat version][crate::neat::timout].
*/
pub fn timout(et: f64, pictur: &str, lenout: usize) -> String {
    let varout_0 = mallocstr!(lenout);
    unsafe {
        crate::c::timout_c(et, crate::cstr!(pictur), lenout as i32, varout_0);
    }
    crate::fcstr!(varout_0)
}

cspice_proc! {
    /**
    Unload a SPICE kernel.
    */
    pub fn unload(name: &str) {}
}

cspice_proc! {
    /**
    Find the separation angle in radians between two double precision, 3-dimensional vectors. This
    angle is defined as zero if either vector is zero.
    */
    #[return_output]
    pub fn vsep(v1: [f64; 3], v2: [f64; 3]) -> f64 {}
}

cspice_proc! {
    /**
    Compute the dot product of two double precision, 3-dimensional vectors.
     */
    #[return_output]
    pub fn vdot(v1: [f64; 3], v2: [f64; 3]) -> f64 {}
}

cspice_proc! {
    /**
    Compute the cross product of two 3-dimensional vectors.
     */
    pub fn vcrss(v1: [f64; 3], v2: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Transpose a 3x3 matrix.
     */
    pub fn xpose(m1: [[f64; 3]; 3]) -> [[f64; 3]; 3] {}
}
