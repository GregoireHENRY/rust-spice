/*!
A Rust idiomatic CSPICE wrapper built with [procedural macros][`spice_derive`].
*/

use crate::{c, get_scalar, get_vec_arr, init_scalar, mptr, ptr_vec_arr};
use spice_derive::{cspice_proc, return_output};

#[allow(clippy::upper_case_acronyms)]
pub type DLADSC = c::SpiceDLADescr;
#[allow(clippy::upper_case_acronyms)]
pub type DSKDSC = c::SpiceDSKDescr;

cspice_proc!(
    /**
    Close a DAS file.
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

/**
Fetch triangular plates from a type 2 DSK segment.
*/
pub fn dskp02(handle: i32, mut dladsc: DLADSC, start: i32, room: i32) -> (i32, Vec<[i32; 3]>) {
    let mut varout_0 = init_scalar!();
    let varout_1 = ptr_vec_arr!([i32; 3], room);
    unsafe {
        crate::c::dskp02_c(handle, &mut dladsc, start, room, mptr!(varout_0), varout_1);
        (
            get_scalar!(varout_0),
            get_vec_arr!(varout_1, get_scalar!(varout_0)),
        )
    }
}

/**
Fetch vertices from a type 2 DSK segment.
*/
pub fn dskv02(handle: i32, mut dladsc: DLADSC, start: i32, room: i32) -> (i32, Vec<[f64; 3]>) {
    let mut varout_0 = init_scalar!();
    let varout_1 = ptr_vec_arr!([f64; 3], room);
    unsafe {
        crate::c::dskv02_c(handle, &mut dladsc, start, room, mptr!(varout_0), varout_1);
        (
            get_scalar!(varout_0),
            get_vec_arr!(varout_1, get_scalar!(varout_0)),
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
    pub fn furnsh<S: Into<String>>(name: S) {}
);

cspice_proc!(
    /**
    Clear the KEEPER subsystem: unload all kernels, clear the kernel pool, and re-initialize the
    subsystem. Existing watches on kernel variables are retained.
    */
    pub fn kclear() {}
);

cspice_proc!(
    /**
    Return the current number of kernels that have been loaded via the KEEPER interface that are of
    a specified type.
    */
    pub fn kdata<S: Into<String>>(
        which: i32,
        kind: S,
        fillen: i32,
        typlen: i32,
        srclen: i32,
    ) -> (String, String, String, i32, bool) {
    }
);

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
    pub fn spkpos<S: Into<String>>(
        targ: S,
        et: f64,
        frame: S,
        abcorr: S,
        obs: S,
    ) -> ([f64; 3], f64) {
    }
);

cspice_proc!(
    /**
    Convert a string representing an epoch to a double precision value representing the number of
    TDB seconds past the J2000 epoch corresponding to the input epoch.
    */
    pub fn str2et<S: Into<String>>(targ: S) -> f64 {}
);

cspice_proc!(
    /**
    This routine converts an input epoch represented in TDB seconds past the TDB epoch of J2000 to a
    character string formatted to the specifications of a user's format picture.
    This function has a [neat version][crate::neat::timout].
    */
    pub fn timout<S: Into<String>>(et: f64, pictur: S, lenout: usize) -> String {}
);

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
