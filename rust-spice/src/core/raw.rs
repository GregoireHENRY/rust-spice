/*!
A Rust idiomatic CSPICE wrapper built with [procedural macros][`spice_derive`].
*/

use crate::{c, cstr, fcstr, get_scalar, get_varr, init_scalar, malloc, mallocstr, mptr};
use spice_derive::{cspice_proc, return_output};
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

/**
Fetch from the kernel pool the double precision values of an item
associated with a body, where the body is specified by an integer ID
code.

This function has a [neat version][crate::neat::bodvcd].
*/
pub fn bodvcd(body: i32, item: &str, maxn: i32) -> (i32, Vec<f64>) {
    let mut varout_0 = init_scalar!();
    let varout_1 = malloc!(f64, maxn);
    unsafe {
        crate::c::bodvcd_c(
            body,
            cstr!(item),
            maxn,
            mptr!(varout_0),
            varout_1
        );
        (
            get_scalar!(varout_0),
            get_varr!(varout_1, get_scalar!(varout_0)),
        )
    }
}

/**
Fetch from the kernel pool the double precision values  
of an item associated with a body.

This function has a [neat version][crate::neat::bodvrd].
*/
pub fn bodvrd(body: &str, item: &str, maxn: i32) -> (i32, Vec<f64>) {
    let mut varout_0 = init_scalar!();
    let varout_1 = malloc!(f64, maxn);
    unsafe {
        crate::c::bodvrd_c(
            cstr!(body),
            cstr!(item),
            maxn,
            mptr!(varout_0),
            varout_1
        );
        (
            get_scalar!(varout_0),
            get_varr!(varout_1, get_scalar!(varout_0)),
        )
    }
}

cspice_proc! {
    /**
    Convert from cylindrical to latitudinal coordinates.
    */
    pub fn cyllat(r: f64, lon: f64, z: f64) -> (f64, f64, f64) {}
}

cspice_proc! {
    /**
    Convert from cylindrical to rectangular coordinates.
    */
    pub fn cylrec(r: f64, lon: f64, z: f64) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Convert from cylindrical to spherical coordinates.
    */
    pub fn cylsph(r: f64, lon: f64, z: f64) -> (f64, f64, f64) {}
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
    Convert geodetic coordinates to rectangular coordinates.
     */
    pub fn georec(lon: f64, lat: f64, alt: f64, re: f64, f: f64) -> [f64; 3] {}
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
    Convert rectangular coordinates to range, right ascension, and declination.
    */
    pub fn recrad(rectan: [f64; 3]) -> (f64, f64, f64) {}
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
    Add two 3 dimensional vectors.
    */
    pub fn vadd(v1: [f64; 3], v2: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Compute the cross product of two 3-dimensional vectors.
     */
    pub fn vcrss(v1: [f64; 3], v2: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Return the distance between two three-dimensional vectors.
    */
    #[return_output]
    pub fn vdist(v1: [f64; 3], v2: [f64; 3]) -> f64 {}
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
    Make one double precision 3-dimensional vector equal to 
    another.
     */
    pub fn vequ(v: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Find the unit vector along a double precision 3-dimensional vector.
     */
    pub fn vhat(v: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    This subroutine computes the vector linear combination
    a*v1 + b*v2 + c*v3 of double precision, 3-dimensional vectors. 
    */
    pub fn vlcom3(a: f64, v1: [f64; 3], b: f64, v2: [f64; 3], c: f64, v3: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Compute a vector linear combination of two double precision, 
    3-dimensional vectors. 
    */
    pub fn vlcom(a: f64, v1: [f64; 3], b: f64, v2: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Negate a double precision 3-dimensional vector.
    */
    pub fn vminus(v: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Compute the magnitude of a double precision, 3-dimensional vector.
    */
    #[return_output]
    pub fn vnorm(v: [f64; 3]) -> f64 {}
}

cspice_proc! {
    /**
    Pack three scalar components into a vector.
    */
    pub fn vpack(x: f64, y: f64, z: f64) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Find the component of a vector that is perpendicular to a second
    vector.  All vectors are 3-dimensional.
    */
    pub fn vperp(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    vproj_c finds the projection of one vector onto another vector.
    All vectors are 3-dimensional.
    */
    pub fn vproj(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Return the relative difference between two 3-dimensional vectors.
    */
    #[return_output]
    pub fn vrel(v1: [f64; 3], v2: [f64; 3]) -> f64 {}
}

cspice_proc! {
    /**
    Rotate a vector about a specified axis vector by a specified 
    angle and return the rotated vector.
    */
    pub fn vrotv(v: [f64; 3], axis: [f64; 3], theta: f64) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Multiply a scalar and a 3-dimensional double precision vector. 
    */
    pub fn vscl(s: f64, v: [f64; 3]) -> [f64; 3] {}
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
    Compute the difference between two 3-dimensional, double 
    precision vectors.
    */
    pub fn vsub(v1: [f64; 3], v2: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Multiply the transpose of a 3-dimensional column vector, 
    a 3x3 matrix, and a 3-dimensional column vector.
    */
    #[return_output]
    pub fn vtmv(v1: [f64; 3],  matrix: [[f64; 3]; 3], v2: [f64; 3]) -> f64 {}
}

cspice_proc! {
    /**
    Unpack three scalar components from a vector.
    */
    pub fn vupack(v: [f64; 3]) -> (f64, f64, f64) {}
}

cspice_proc! {
    /**
    Indicate whether a 3-vector is the zero vector.
    */
    #[return_output]
    pub fn vzero(v: [f64; 3]) -> bool {}
}


cspice_proc! {
    /**
    Transpose a 3x3 matrix.
     */
    pub fn xpose(m1: [[f64; 3]; 3]) -> [[f64; 3]; 3] {}
}
