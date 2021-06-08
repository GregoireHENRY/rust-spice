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

cspice_proc!(
    /**
    Translate the SPICE integer code of a body into a common name for that body.

    This function has a [neat version][crate::neat::bodyc2n].
    */
    pub fn bodc2n(code: i32, lenout: i32) -> (String, bool) {}
);

cspice_proc!(
    /**
    Translate the name of a body or object to the corresponding SPICE integer ID code.
    */
    pub fn bodn2c<S: Into<String>>(name: S) -> (i32, bool) {}
);

cspice_proc!(
    /**
    close a das file.
    */
    pub fn dascls(handle: i32) {}
);

cspice_proc!(
    /**
    Open a DAS file for reading.
    */
    pub fn dasopr<S: Into<String>>(fname: S) -> i32 {}
);

cspice_proc!(
    /**
    Begin a forward segment search in a DLA file.
    */
    pub fn dlabfs(handle: i32) -> (DLADSC, bool) {}
);

cspice_proc!(
    /**
    Return the DSK descriptor from a DSK segment identified  by a DAS handle and DLA descriptor.
    */
    pub fn dskgd(handle: i32, dladsc: DLADSC) -> DSKDSC {}
);

cspice_proc!(
    /**
    Compute the unit normal vector for a specified plate from a type 2 DSK segment.
    */
    pub fn dskn02(handle: i32, dladsc: DLADSC, plid: i32) -> [f64; 3] {}
);

cspice_proc!(
    /**
    Find the set of body ID codes of all objects for which topographic data are provided in a
    specified DSK file.
    */
    pub fn dskobj<S: Into<String>>(dsk: S) -> Cell {}
);

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

cspice_proc!(
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
);

cspice_proc!(
    /**
    Return plate model size parameters---plate count and vertex count---for a type 2 DSK segment.
    */
    pub fn dskz02(handle: i32, dladsc: DLADSC) -> (i32, i32) {}
);

cspice_proc!(
    /**
    Compute the illumination angles---phase, incidence, and emission---at a specified point on a
    target body. Return logical flags indicating whether the surface point is visible from the
    observer's position and whether the surface point is illuminated.

    The target body's surface is represented using topographic data provided by DSK files, or by a
    reference ellipsoid.

    The illumination source is a specified ephemeris object.
    */
    pub fn illumf<S: Into<String>>(
        method: S,
        target: S,
        ilusrc: S,
        et: f64,
        fixref: S,
        abcorr: S,
        obsrvr: S,
        spoint: [f64; 3],
    ) -> (f64, [f64; 3], f64, f64, f64, bool, bool) {
    }
);

cspice_proc!(
    /**
    Load one or more SPICE kernels into a program.
    */
    pub fn furnsh(name: &str) {}
);

cspice_proc!(
    /**
    Clear the KEEPER subsystem: unload all kernels, clear the kernel pool, and re-initialize the
    subsystem. Existing watches on kernel variables are retained.
    */
    pub fn kclear() {}
);

/**
Fetch vertices from a type 2 DSK segment.

This function has a [neat version][crate::neat::kdata].
*/
pub fn kdata<S: Into<String>>(
    which: i32,
    kind: S,
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
            cstr!(kind.into()),
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

cspice_proc!(
    /**
    Return the current number of kernels that have been loaded via the KEEPER interface that are of
    a specified type.
    */
    pub fn ktotal<S: Into<String>>(kind: S) -> i32 {}
);

cspice_proc!(
    /**
    Convert from latitudinal coordinates to rectangular coordinates.
    */
    pub fn latrec(radius: f64, longitude: f64, latitude: f64) -> [f64; 3] {}
);

cspice_proc!(
    /**
    Determines the occultation condition (not occulted, partially, etc.) of one target relative to
    another target as seen by an observer at a given time, with targets modeled as points,
    ellipsoids, or digital shapes (DSK)
    */
    #[allow(clippy::too_many_arguments)]
    pub fn occult<S: Into<String>>(
        targ1: S,
        shape1: S,
        frame1: S,
        targ2: S,
        shape2: S,
        frame2: S,
        abcorr: S,
        obsrvr: S,
        et: f64,
    ) -> i32 {
    }
);

cspice_proc!(
    /**
    Return the matrix that transforms position vectors from one specified frame to another at a
    specified epoch.
    */
    pub fn pxform<S: Into<String>>(from: S, to: S, et: f64) -> [[f64; 3]; 3] {}
);

cspice_proc!(
    /**
    Return the 3x3 matrix that transforms position vectors from one specified frame at a specified
    epoch to another specified frame at another specified epoch.
    */
    pub fn pxfrm2<S: Into<String>>(from: S, to: S, etfrom: f64, etto: f64) -> [[f64; 3]; 3] {}
);

cspice_proc!(
    /**
    Convert rectangular coordinates to range, right ascension, and declination.
    */
    pub fn recrad(rectan: [f64; 3]) -> (f64, f64, f64) {}
);

cspice_proc!(
    /**
    Return the position of a target body relative to an observing body, optionally corrected for
    light time (planetary aberration) and stellar aberration.
    */
    pub fn spkpos(targ: &str, et: f64, frame: &str, abcorr: &str, obs: &str) -> ([f64; 3], f64) {}
);

cspice_proc!(
    /**
    Convert a string representing an epoch to a double precision value representing the number of
    TDB seconds past the J2000 epoch corresponding to the input epoch.
    */
    pub fn str2et(targ: &str) -> f64 {}
);

/**
This routine converts an input epoch represented in TDB seconds past the TDB epoch of J2000 to a
character string formatted to the specifications of a user's format picture.

This function has a [neat version][crate::neat::timout].
*/
pub fn timout<S: Into<String>>(et: f64, pictur: S, lenout: usize) -> String {
    let varout_0 = mallocstr!(lenout);
    unsafe {
        crate::c::timout_c(et, crate::cstr!(pictur.into()), lenout as i32, varout_0);
    }
    crate::fcstr!(varout_0)
}

cspice_proc!(
    /**
    Unload a SPICE kernel.
    */
    pub fn unload<S: Into<String>>(name: S) {}
);

cspice_proc!(
    /**
    Find the separation angle in radians between two double precision, 3-dimensional vectors. This
    angle is defined as zero if either vector is zero.
    */
    #[return_output]
    pub fn vsep(v1: [f64; 3], v2: [f64; 3]) -> f64 {}
);
