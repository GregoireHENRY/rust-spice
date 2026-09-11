/*!
A Rust idiomatic CSPICE wrapper built with [procedural macros][`spice_derive`].

## Description

Every routine here mirrors its CSPICE counterpart one for one: the C inputs stay inputs, and the
values the C routine writes through pointers become the Rust return value, in the order CSPICE
declares them. Wrappers that need an explicit buffer size, or a caller allocated
[cell][crate::core::cell::Cell], have a friendlier counterpart in [`neat`][crate::core::neat].
*/

use crate::c::{SpiceChar, SpiceDouble, SpiceInt};
use crate::core::cell::{Cell, CellItem};
use crate::core::ffi::{from_cbuf, from_strided, to_cstring, to_strided};
use spice_derive::cspice_proc;

#[cfg(any(feature = "lock", doc))]
use {crate::core::lock::SpiceLock, spice_derive::impl_for};

/// A DLA segment descriptor.
#[allow(clippy::upper_case_acronyms)]
pub type DLADSC = crate::c::SpiceDLADescr;

/// A DSK segment descriptor.
#[allow(clippy::upper_case_acronyms)]
pub type DSKDSC = crate::c::SpiceDSKDescr;

/// A plane, as a unit normal and the constant of the plane equation.
#[allow(clippy::upper_case_acronyms)]
pub type PLANE = crate::c::SpicePlane;

/// An ellipse, as a centre and two generating vectors.
#[allow(clippy::upper_case_acronyms)]
pub type ELLIPSE = crate::c::SpiceEllipse;

/// The raw CSPICE cell descriptor; see [`Cell`] for the owning Rust type.
#[allow(clippy::upper_case_acronyms, dead_code)]
pub type CELL = crate::c::SpiceCell;

/* -------------------------------------------------------------------------------------------- */
/* Bodies, names and identifiers                                                                  */
/* -------------------------------------------------------------------------------------------- */

cspice_proc! {
    /**
    Translate the SPICE integer code of a body into a common name for that body.

    This function has a [neat version][crate::neat::bodc2n].
    */
    pub fn bodc2n(code: i32, #[lenout] lenout: i32) -> (String, bool) {}
}

cspice_proc! {
    /**
    Translate a body ID code to either the corresponding name or, if no name exists, the string
    representation of the code.

    This function has a [neat version][crate::neat::bodc2s].
    */
    pub fn bodc2s(code: i32, #[lenout] lenout: i32) -> String {}
}

cspice_proc! {
    /**
    Determine whether values exist for some item for any body in the kernel pool.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn bodfnd(body: i32, item: &str) -> bool {}
}

cspice_proc! {
    /**
    Translate the name of a body or object to the corresponding SPICE integer ID code.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn bodn2c(name: &str) -> (i32, bool) {}
}

cspice_proc! {
    /**
    Translate a string containing a body name or ID code to an integer code.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn bods2c(name: &str) -> (i32, bool) {}
}

/**
Fetch from the kernel pool the double precision values of an item associated with a body.

At most `maxn` values are returned; the vector is truncated to the number actually found, and is
empty when the item is not in the pool.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn bodvrd(bodynm: &str, item: &str, maxn: usize) -> Vec<f64> {
    let bodynm = to_cstring(bodynm);
    let item = to_cstring(item);
    let mut dim = 0;
    let mut values = vec![0.0; maxn];
    unsafe {
        crate::c::bodvrd_c(
            bodynm.as_ptr() as *mut SpiceChar,
            item.as_ptr() as *mut SpiceChar,
            maxn as SpiceInt,
            &mut dim,
            values.as_mut_ptr(),
        )
    };
    values.truncate(dim.max(0) as usize);
    values
}

/**
Fetch from the kernel pool the double precision values of an item associated with a body.

Deprecated by CSPICE in favour of [`bodvcd`] and [`bodvrd`]: it takes no bound on how much it may
write, so the caller has to know how many values to make room for.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn bodvar(body: i32, item: &str, maxn: usize) -> Vec<f64> {
    let item = to_cstring(item);
    let mut dim = 0;
    let mut values = vec![0.0; maxn];
    unsafe {
        crate::c::bodvar_c(
            body,
            item.as_ptr() as *mut SpiceChar,
            &mut dim,
            values.as_mut_ptr(),
        )
    };
    values.truncate(dim.max(0) as usize);
    values
}

/**
Fetch from the kernel pool the double precision values of an item associated with a body, using the
body's integer ID code.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn bodvcd(bodyid: i32, item: &str, maxn: usize) -> Vec<f64> {
    let item = to_cstring(item);
    let mut dim = 0;
    let mut values = vec![0.0; maxn];
    unsafe {
        crate::c::bodvcd_c(
            bodyid,
            item.as_ptr() as *mut SpiceChar,
            maxn as SpiceInt,
            &mut dim,
            values.as_mut_ptr(),
        )
    };
    values.truncate(dim.max(0) as usize);
    values
}

/* -------------------------------------------------------------------------------------------- */
/* C-kernels                                                                                      */
/* -------------------------------------------------------------------------------------------- */

cspice_proc! {
    /**
    Find the coverage window for a specified object in a specified CK file.

    This function has a [neat version][crate::neat::ckcov].
    */
    #[allow(clippy::too_many_arguments)]
    pub fn ckcov(
        ck: &str,
        idcode: i32,
        needav: bool,
        level: &str,
        tol: f64,
        timsys: &str,
        cover: &mut Cell<f64>
    ) {}
}

cspice_proc! {
    /**
    Get pointing (attitude) for a specified spacecraft clock time.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn ckgp(inst: i32, sclkdp: f64, tol: f64, frame: &str) -> ([[f64; 3]; 3], f64, bool) {}
}

cspice_proc! {
    /**
    Get pointing (attitude) and angular velocity for a spacecraft clock time.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn ckgpav(
        inst: i32,
        sclkdp: f64,
        tol: f64,
        frame: &str
    ) -> ([[f64; 3]; 3], [f64; 3], f64, bool) {
    }
}

cspice_proc! {
    /**
    Find the set of ID codes of all objects in a specified CK file.

    This function has a [neat version][crate::neat::ckobj].
    */
    pub fn ckobj(ck: &str, ids: &mut Cell<i32>) {}
}

cspice_proc! {
    /**
    Open a new CK file, returning the handle of the opened file.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn ckopn(fname: &str, ifname: &str, ncomch: i32) -> i32 {}
}

cspice_proc! {
    /**
    Close an open CK file.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn ckcls(handle: i32) {}
}

/**
Add a type 3 segment to a CK file.

# Panics

Panics if `nrec` or `nints` is negative, or larger than the arrays it counts: CSPICE reads exactly
that many records and has no way of knowing how long they are.
*/
#[allow(clippy::too_many_arguments)]
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn ckw03(
    handle: i32,
    begtim: f64,
    endtim: f64,
    inst: i32,
    frame: &str,
    avflag: bool,
    segid: &str,
    nrec: i32,
    sclkdp: &[f64],
    quats: &[[f64; 4]],
    avvs: &[[f64; 3]],
    nints: i32,
    starts: &[f64],
) {
    let records = usize::try_from(nrec).expect("the number of records cannot be negative");
    let intervals = usize::try_from(nints).expect("the number of intervals cannot be negative");
    assert!(
        records <= sclkdp.len() && records <= quats.len(),
        "ckw03 was asked for {records} records but got {} clock times and {} quaternions",
        sclkdp.len(),
        quats.len()
    );
    assert!(
        !avflag || records <= avvs.len(),
        "ckw03 was asked for {records} angular velocities but got {}",
        avvs.len()
    );
    assert!(
        intervals <= starts.len(),
        "ckw03 was asked for {intervals} intervals but got {} start times",
        starts.len()
    );

    let frame = to_cstring(frame);
    let segid = to_cstring(segid);

    unsafe {
        crate::c::ckw03_c(
            handle,
            begtim,
            endtim,
            inst,
            frame.as_ptr() as *mut SpiceChar,
            avflag as crate::c::SpiceBoolean,
            segid.as_ptr() as *mut SpiceChar,
            nrec,
            sclkdp.as_ptr() as *mut SpiceDouble,
            quats.as_ptr() as *mut [SpiceDouble; 4],
            avvs.as_ptr() as *mut [SpiceDouble; 3],
            nints,
            starts.as_ptr() as *mut SpiceDouble,
        );
    }
}

/* -------------------------------------------------------------------------------------------- */
/* DAS, DLA and DSK                                                                               */
/* -------------------------------------------------------------------------------------------- */

cspice_proc! {
    /**
    Close a DAS file.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dascls(handle: i32) {}
}

cspice_proc! {
    /**
    Open a DAS file for reading.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dasopr(fname: &str) -> i32 {}
}

cspice_proc! {
    /**
    Begin a forward segment search in a DLA file.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dlabfs(handle: i32) -> (DLADSC, bool) {}
}

cspice_proc! {
    /**
    Begin a backward segment search in a DLA file.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dlabbs(handle: i32) -> (DLADSC, bool) {}
}

cspice_proc! {
    /**
    Find the segment following a specified segment in a DLA file.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dlafns(handle: i32, dladsc: DLADSC) -> (DLADSC, bool) {}
}

cspice_proc! {
    /**
    Return the DSK descriptor from a DSK segment identified by a DAS handle and DLA descriptor.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dskgd(handle: i32, dladsc: DLADSC) -> DSKDSC {}
}

cspice_proc! {
    /**
    Compute the unit normal vector for a specified plate from a type 2 DSK segment.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dskn02(handle: i32, dladsc: DLADSC, plid: i32) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Find the set of body ID codes of all objects for which topographic data are provided in a
    specified DSK file.

    This function has a [neat version][crate::neat::dskobj].
    */
    pub fn dskobj(dsk: &str, bodids: &mut Cell<i32>) {}
}

cspice_proc! {
    /**
    Find the set of surface ID codes for all surfaces associated with a given body in a specified
    DSK file.

    This function has a [neat version][crate::neat::dsksrf].
    */
    pub fn dsksrf(dsk: &str, bodyid: i32, srfids: &mut Cell<i32>) {}
}

/**
Fetch triangular plates from a type 2 DSK segment.

This function has a [neat version][crate::neat::dskp02].
*/
pub fn dskp02(handle: i32, dladsc: DLADSC, start: usize, room: usize) -> Vec<[i32; 3]> {
    let mut dladsc = dladsc;
    let mut n = 0;
    let mut plates = vec![[0; 3]; room];

    unsafe {
        crate::c::dskp02_c(
            handle,
            &mut dladsc,
            start as SpiceInt,
            room as SpiceInt,
            &mut n,
            plates.as_mut_ptr(),
        );
    }

    plates.truncate(n.max(0) as usize);
    plates
}

/**
Fetch vertices from a type 2 DSK segment.

This function has a [neat version][crate::neat::dskv02].
*/
pub fn dskv02(handle: i32, dladsc: DLADSC, start: usize, room: usize) -> Vec<[f64; 3]> {
    let mut dladsc = dladsc;
    let mut n = 0;
    let mut vrtces = vec![[0.0; 3]; room];

    unsafe {
        crate::c::dskv02_c(
            handle,
            &mut dladsc,
            start as SpiceInt,
            room as SpiceInt,
            &mut n,
            vrtces.as_mut_ptr(),
        );
    }

    vrtces.truncate(n.max(0) as usize);
    vrtces
}

cspice_proc! {
    /**
    Determine the plate ID and body-fixed coordinates of the intersection of a specified ray with
    the surface defined by a type 2 DSK plate model.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dskx02(
        handle: i32,
        dladsc: DLADSC,
        vertex: [f64; 3],
        raydir: [f64; 3]
    ) -> (i32, [f64; 3], bool) {
    }
}

cspice_proc! {
    /**
    Return plate model size parameters---vertex count and plate count---for a type 2 DSK segment.

    Vertices first, plates second.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dskz02(handle: i32, dladsc: DLADSC) -> (i32, i32) {}
}

/// Size of the `dc` output of [`dskxsi`].
pub const DSKXSI_DCSIZE: usize = 1;
/// Size of the `ic` output of [`dskxsi`]; for a type 2 segment `ic[0]` is the plate ID.
pub const DSKXSI_ICSIZE: usize = 1;

/**
Compute a ray-surface intercept using data provided by multiple loaded DSK segments, and return
information about the source of the data defining the surface on which the intercept was found.

`srflst` is the list of surface IDs to consider; leave it empty to let CSPICE choose.
*/
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn dskxsi(
    pri: bool,
    target: &str,
    srflst: &[i32],
    et: f64,
    fixref: &str,
    vertex: [f64; 3],
    raydir: [f64; 3],
) -> (
    [f64; 3],
    i32,
    DLADSC,
    DSKDSC,
    [f64; DSKXSI_DCSIZE],
    [i32; DSKXSI_ICSIZE],
    bool,
) {
    let target = to_cstring(target);
    let fixref = to_cstring(fixref);

    let mut xpt = [0.0; 3];
    let mut handle = 0;
    // SAFETY: both descriptors are plain old data, for which all-zero is a valid value.
    let mut dladsc: DLADSC = unsafe { std::mem::zeroed() };
    let mut dskdsc: DSKDSC = unsafe { std::mem::zeroed() };
    let mut dc = [0.0; DSKXSI_DCSIZE];
    let mut ic = [0; DSKXSI_ICSIZE];
    let mut found = 0;

    unsafe {
        crate::c::dskxsi_c(
            pri as crate::c::SpiceBoolean,
            target.as_ptr() as *mut SpiceChar,
            srflst.len() as SpiceInt,
            srflst.as_ptr() as *mut SpiceInt,
            et,
            fixref.as_ptr() as *mut SpiceChar,
            vertex.as_ptr() as *mut SpiceDouble,
            raydir.as_ptr() as *mut SpiceDouble,
            DSKXSI_DCSIZE as SpiceInt,
            DSKXSI_ICSIZE as SpiceInt,
            xpt.as_mut_ptr(),
            &mut handle,
            &mut dladsc,
            &mut dskdsc,
            dc.as_mut_ptr(),
            ic.as_mut_ptr(),
            &mut found,
        );
    }

    (xpt, handle, dladsc, dskdsc, dc, ic, found != 0)
}

/**
Compute ray-surface intercepts for a set of rays, using data provided by multiple loaded DSK
segments.

Returns one intercept and one flag per ray.
*/
#[allow(clippy::too_many_arguments)]
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn dskxv(
    pri: bool,
    target: &str,
    srflst: &[i32],
    et: f64,
    fixref: &str,
    vtxarr: &[[f64; 3]],
    dirarr: &[[f64; 3]],
) -> (Vec<[f64; 3]>, Vec<bool>) {
    let rays = vtxarr.len().min(dirarr.len());
    let target = to_cstring(target);
    let fixref = to_cstring(fixref);

    let mut xptarr = vec![[0.0; 3]; rays];
    let mut fndarr = vec![0 as crate::c::SpiceBoolean; rays];

    unsafe {
        crate::c::dskxv_c(
            pri as crate::c::SpiceBoolean,
            target.as_ptr() as *mut SpiceChar,
            srflst.len() as SpiceInt,
            srflst.as_ptr() as *mut SpiceInt,
            et,
            fixref.as_ptr() as *mut SpiceChar,
            rays as SpiceInt,
            vtxarr.as_ptr() as *mut [f64; 3],
            dirarr.as_ptr() as *mut [f64; 3],
            xptarr.as_mut_ptr(),
            fndarr.as_mut_ptr(),
        );
    }

    (xptarr, fndarr.into_iter().map(|flag| flag != 0).collect())
}

cspice_proc! {
    /**
    Open a new DSK file for subsequent write operations.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dskopn(fname: &str, ifname: &str, ncomch: i32) -> i32 {}
}

cspice_proc! {
    /**
    Close a DSK file, optionally segregating it for faster access.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dskcls(handle: i32, optmiz: bool) {}
}

/// Keyword of the plate expansion fraction, for [`dskgtl`] and [`dskstl`].
pub const DSK_KEYXFR: i32 = 1;
/// Keyword of the greedy segment selection factor.
pub const DSK_KEYSGR: i32 = 2;
/// Keyword of the segment pad margin.
pub const DSK_KEYSPM: i32 = 3;
/// Keyword of the surface point membership margin.
pub const DSK_KEYPTM: i32 = 4;
/// Keyword of the angular rounding margin.
pub const DSK_KEYAMG: i32 = 5;
/// Keyword of the longitude alias margin.
pub const DSK_KEYLAL: i32 = 6;

cspice_proc! {
    /**
    Retrieve the value of a specified DSK tolerance or margin parameter.

    The keyword is one of the `DSK_KEY*` constants of this module.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dskgtl(keywrd: i32) -> f64 {}
}

cspice_proc! {
    /**
    Set the value of a specified DSK tolerance or margin parameter.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dskstl(keywrd: i32, dpval: f64) {}
}

/// Size of the double precision component of a type 2 DSK spatial index.
pub const DSK02_SPADSZ: usize = 10;

/**
Make a spatial index for a DSK type 2 segment, returning its double precision and integer
components.

`spxisz` is the size to allocate for the integer component; see the
[C documentation](https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskmi2_c.html) for how to
size it and the work array.
*/
#[allow(clippy::too_many_arguments)]
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn dskmi2(
    vrtces: &[[f64; 3]],
    plates: &[[i32; 3]],
    finscl: f64,
    corscl: i32,
    worksz: usize,
    voxpsz: usize,
    voxlsz: usize,
    makvtl: bool,
    spxisz: usize,
) -> (Vec<f64>, Vec<i32>) {
    let mut work = vec![[0; 2]; worksz.max(1)];
    let mut spaixd = vec![0.0; DSK02_SPADSZ];
    let mut spaixi = vec![0; spxisz.max(1)];

    unsafe {
        crate::c::dskmi2_c(
            vrtces.len() as SpiceInt,
            vrtces.as_ptr() as *mut [f64; 3],
            plates.len() as SpiceInt,
            plates.as_ptr() as *mut [SpiceInt; 3],
            finscl,
            corscl,
            worksz as SpiceInt,
            voxpsz as SpiceInt,
            voxlsz as SpiceInt,
            makvtl as SpiceInt,
            spxisz as SpiceInt,
            work.as_mut_ptr(),
            spaixd.as_mut_ptr(),
            spaixi.as_mut_ptr(),
        );
    }

    (spaixd, spaixi)
}

/// Number of coordinate system parameters a DSK descriptor holds.
pub const DSK_NSYPAR: usize = 10;

/**
Write a type 2 segment to a DSK file.

# Panics

Panics if `corpar` holds fewer than [`DSK_NSYPAR`] values: CSPICE reads that many whatever the
coordinate system.
*/
#[allow(clippy::too_many_arguments)]
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn dskw02(
    handle: i32,
    center: i32,
    surfce: i32,
    dclass: i32,
    frame: &str,
    corsys: i32,
    corpar: &[f64],
    mncor1: f64,
    mxcor1: f64,
    mncor2: f64,
    mxcor2: f64,
    mncor3: f64,
    mxcor3: f64,
    first: f64,
    last: f64,
    vrtces: &[[f64; 3]],
    plates: &[[i32; 3]],
    spaixd: &[f64],
    spaixi: &[i32],
) {
    assert!(
        corpar.len() >= DSK_NSYPAR,
        "dskw02 needs {DSK_NSYPAR} coordinate parameters but got {}",
        corpar.len()
    );

    let frame = to_cstring(frame);

    unsafe {
        crate::c::dskw02_c(
            handle,
            center,
            surfce,
            dclass,
            frame.as_ptr() as *mut SpiceChar,
            corsys,
            corpar.as_ptr() as *mut SpiceDouble,
            mncor1,
            mxcor1,
            mncor2,
            mxcor2,
            mncor3,
            mxcor3,
            first,
            last,
            vrtces.len() as SpiceInt,
            vrtces.as_ptr() as *mut [f64; 3],
            plates.len() as SpiceInt,
            plates.as_ptr() as *mut [SpiceInt; 3],
            spaixd.as_ptr() as *mut SpiceDouble,
            spaixi.as_ptr() as *mut SpiceInt,
        );
    }
}

/* -------------------------------------------------------------------------------------------- */
/* Kernel pool                                                                                    */
/* -------------------------------------------------------------------------------------------- */

cspice_proc! {
    /**
    Load one or more SPICE kernels into a program.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn furnsh(name: &str) {}
}

cspice_proc! {
    /**
    Unload a SPICE kernel.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn unload(name: &str) {}
}

cspice_proc! {
    /**
    Clear the KEEPER subsystem: unload all kernels, clear the kernel pool, and re-initialize the
    subsystem. Existing watches on kernel variables are retained.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn kclear() {}
}

cspice_proc! {
    /**
    Return the current number of kernels that have been loaded via the KEEPER interface that are of
    a specified type.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn ktotal(kind: &str) -> i32 {}
}

cspice_proc! {
    /**
    Load the variables contained in a text kernel into the kernel pool.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn ldpool(filename: &str) {}
}

cspice_proc! {
    /**
    Remove all variables from the kernel pool.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn clpool() {}
}

cspice_proc! {
    /**
    Confirm the existence of a kernel variable in the kernel pool.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn expool(name: &str) -> bool {}
}

cspice_proc! {
    /**
    Return the data about a kernel pool variable: whether it exists, how many components it has,
    and whether it is numeric (`"N"`) or character (`"C"`).
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dtpool(name: &str) -> (bool, i32, String) {}
}

/**
Return the file name, type, source and handle of a loaded kernel.

This function has a [neat version][crate::neat::kdata].
*/
#[allow(clippy::too_many_arguments)]
pub fn kdata(
    which: i32,
    kind: &str,
    fillen: i32,
    typlen: i32,
    srclen: i32,
) -> (String, String, String, i32, bool) {
    let kind = to_cstring(kind);
    let mut file = vec![0 as SpiceChar; fillen.max(1) as usize];
    let mut filtyp = vec![0 as SpiceChar; typlen.max(1) as usize];
    let mut source = vec![0 as SpiceChar; srclen.max(1) as usize];
    let mut handle = 0;
    let mut found = 0;

    unsafe {
        crate::c::kdata_c(
            which,
            kind.as_ptr() as *mut SpiceChar,
            fillen,
            typlen,
            srclen,
            file.as_mut_ptr(),
            filtyp.as_mut_ptr(),
            source.as_mut_ptr(),
            &mut handle,
            &mut found,
        );
    }

    (
        from_cbuf(&file),
        from_cbuf(&filtyp),
        from_cbuf(&source),
        handle,
        found != 0,
    )
}

/**
Return the type, source and handle of a loaded kernel, given its name.

This function has a [neat version][crate::neat::kinfo].
*/
pub fn kinfo(file: &str, typlen: i32, srclen: i32) -> (String, String, i32, bool) {
    let file = to_cstring(file);
    let mut filtyp = vec![0 as SpiceChar; typlen.max(1) as usize];
    let mut source = vec![0 as SpiceChar; srclen.max(1) as usize];
    let mut handle = 0;
    let mut found = 0;

    unsafe {
        crate::c::kinfo_c(
            file.as_ptr() as *mut SpiceChar,
            typlen,
            srclen,
            filtyp.as_mut_ptr(),
            source.as_mut_ptr(),
            &mut handle,
            &mut found,
        );
    }

    (from_cbuf(&filtyp), from_cbuf(&source), handle, found != 0)
}

/**
Return the double precision values of a kernel pool variable.

The vector is empty when the variable is absent from the pool.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn gdpool(name: &str, start: usize, room: usize) -> Vec<f64> {
    let name = to_cstring(name);
    let mut n = 0;
    let mut values = vec![0.0; room];
    let mut found = 0;

    unsafe {
        crate::c::gdpool_c(
            name.as_ptr() as *mut SpiceChar,
            start as SpiceInt,
            room as SpiceInt,
            &mut n,
            values.as_mut_ptr(),
            &mut found,
        )
    }

    values.truncate(if found != 0 { n.max(0) as usize } else { 0 });
    values
}

/**
Return the integer values of a kernel pool variable.

The vector is empty when the variable is absent from the pool.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn gipool(name: &str, start: usize, room: usize) -> Vec<i32> {
    let name = to_cstring(name);
    let mut n = 0;
    let mut values = vec![0; room];
    let mut found = 0;

    unsafe {
        crate::c::gipool_c(
            name.as_ptr() as *mut SpiceChar,
            start as SpiceInt,
            room as SpiceInt,
            &mut n,
            values.as_mut_ptr(),
            &mut found,
        )
    }

    values.truncate(if found != 0 { n.max(0) as usize } else { 0 });
    values
}

/**
Return the character values of a kernel pool variable.

The vector is empty when the variable is absent from the pool.

This function has a [neat version][crate::neat::gcpool].
*/
pub fn gcpool(name: &str, start: usize, room: usize, lenout: usize) -> Vec<String> {
    let name = to_cstring(name);
    let lenout = lenout.max(1);
    let mut n = 0;
    let mut values = vec![0 as SpiceChar; room * lenout];
    let mut found = 0;

    unsafe {
        crate::c::gcpool_c(
            name.as_ptr() as *mut SpiceChar,
            start as SpiceInt,
            room as SpiceInt,
            lenout as SpiceInt,
            &mut n,
            values.as_mut_ptr().cast(),
            &mut found,
        )
    }

    let count = if found != 0 { n.max(0) as usize } else { 0 };
    (0..count)
        .map(|index| from_cbuf(&values[index * lenout..(index + 1) * lenout]))
        .collect()
}

/**
Set a watch on a set of kernel pool variables for a named agent.

[`cvpool`] then reports whether any of them have been updated since the agent last asked.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn swpool<S: AsRef<str>>(agent: &str, names: &[S]) {
    let agent = to_cstring(agent);
    let (buffer, lenvals) = to_strided(names);
    unsafe {
        crate::c::swpool_c(
            agent.as_ptr() as *mut SpiceChar,
            names.len() as SpiceInt,
            lenvals as SpiceInt,
            buffer.as_ptr().cast(),
        )
    }
}

/**
Return the names of the kernel pool variables matching a template.

The vector is empty when nothing matches.

This function has a [neat version][crate::neat::gnpool].
*/
pub fn gnpool(name: &str, start: usize, room: usize, lenout: usize) -> Vec<String> {
    let name = to_cstring(name);
    let lenout = lenout.max(1);
    let mut buffer = vec![0 as SpiceChar; room.max(1) * lenout];
    let mut n = 0;
    let mut found = 0;

    unsafe {
        crate::c::gnpool_c(
            name.as_ptr() as *mut SpiceChar,
            start as SpiceInt,
            room as SpiceInt,
            lenout as SpiceInt,
            &mut n,
            buffer.as_mut_ptr().cast(),
            &mut found,
        )
    };

    let count = if found != 0 { n.max(0) as usize } else { 0 };
    from_strided(&buffer, lenout, count)
}

/**
Insert double precision values into the kernel pool.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn pdpool(name: &str, values: &[f64]) {
    let name = to_cstring(name);
    unsafe {
        crate::c::pdpool_c(
            name.as_ptr() as *mut SpiceChar,
            values.len() as SpiceInt,
            values.as_ptr() as *mut SpiceDouble,
        )
    }
}

/**
Insert integer values into the kernel pool.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn pipool(name: &str, values: &[i32]) {
    let name = to_cstring(name);
    unsafe {
        crate::c::pipool_c(
            name.as_ptr() as *mut SpiceChar,
            values.len() as SpiceInt,
            values.as_ptr() as *mut SpiceInt,
        )
    }
}

/**
Insert character values into the kernel pool.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn pcpool<S: AsRef<str>>(name: &str, values: &[S]) {
    let name = to_cstring(name);
    let (buffer, lenvals) = to_strided(values);

    unsafe {
        crate::c::pcpool_c(
            name.as_ptr() as *mut SpiceChar,
            values.len() as SpiceInt,
            lenvals as SpiceInt,
            buffer.as_ptr().cast(),
        )
    }
}

/* -------------------------------------------------------------------------------------------- */
/* Time                                                                                           */
/* -------------------------------------------------------------------------------------------- */

cspice_proc! {
    /**
    Convert a string representing an epoch to a double precision value representing the number of
    TDB seconds past the J2000 epoch corresponding to the input epoch.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn str2et(targ: &str) -> f64 {}
}

cspice_proc! {
    /**
    Convert an input time from UTC seconds past the J2000 epoch to ephemeris seconds past J2000.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn utc2et(utcstr: &str) -> f64 {}
}

cspice_proc! {
    /**
    Convert an input epoch, in ephemeris seconds past J2000, to a UTC string.

    This function has a [neat version][crate::neat::et2utc].
    */
    pub fn et2utc(et: f64, format: &str, prec: i32, #[lenout] lenout: i32) -> String {}
}

cspice_proc! {
    /**
    Convert an input epoch represented in TDB seconds past the TDB epoch of J2000 to a character
    string formatted to the specifications of a user's format picture.

    This function has a [neat version][crate::neat::timout].
    */
    pub fn timout(et: f64, pictur: &str, #[lenout] lenout: i32) -> String {}
}

cspice_proc! {
    /**
    Parse a time string and return the number of seconds past the J2000 epoch on a formal calendar,
    together with the error message CSPICE produced, empty when the parse succeeded.

    This function has a [neat version][crate::neat::tparse].
    */
    pub fn tparse(string: &str, #[lenout] lenout: i32) -> (f64, String) {}
}

cspice_proc! {
    /**
    Return the value of Delta ET (ET-UTC) for an input epoch.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn deltet(epoch: f64, eptype: &str) -> f64 {}
}

cspice_proc! {
    /**
    Transform time from one uniform scale to another. The uniform time scales are TAI, GPS, TT, TDT,
    TDB, ET, JED, JDTDB, JDTDT.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn unitim(epoch: f64, insys: &str, outsys: &str) -> f64 {}
}

cspice_proc! {
    /**
    Convert ephemeris seconds past J2000 to continuous encoded spacecraft clock ticks.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn sce2c(sc: i32, et: f64) -> f64 {}
}

cspice_proc! {
    /**
    Convert ephemeris seconds past J2000 to a spacecraft clock string.

    This function has a [neat version][crate::neat::sce2s].
    */
    pub fn sce2s(sc: i32, et: f64, #[lenout] sclklen: i32) -> String {}
}

cspice_proc! {
    /**
    Encode a character representation of spacecraft clock time into a double precision number.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn scencd(sc: i32, sclkch: &str) -> f64 {}
}

cspice_proc! {
    /**
    Convert a double precision encoding of spacecraft clock time into a character representation.

    This function has a [neat version][crate::neat::scdecd].
    */
    pub fn scdecd(sc: i32, sclkdp: f64, #[lenout] sclklen: i32) -> String {}
}

cspice_proc! {
    /**
    Convert a spacecraft clock string to ephemeris seconds past J2000.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn scs2e(sc: i32, sclkch: &str) -> f64 {}
}

cspice_proc! {
    /**
    Convert encoded spacecraft clock ticks to ephemeris seconds past J2000.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn sct2e(sc: i32, sclkdp: f64) -> f64 {}
}

cspice_proc! {
    /**
    Compute L_s, the planetocentric longitude of the sun, as seen from a specified body.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn lspcn(body: &str, et: f64, abcorr: &str) -> f64 {}
}

/* -------------------------------------------------------------------------------------------- */
/* Frames                                                                                         */
/* -------------------------------------------------------------------------------------------- */

cspice_proc! {
    /**
    Return the matrix that transforms position vectors from one specified frame to another at a
    specified epoch.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn pxform(from: &str, to: &str, et: f64) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Return the 3x3 matrix that transforms position vectors from one specified frame at a specified
    epoch to another specified frame at another specified epoch.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn pxfrm2(from: &str, to: &str, etfrom: f64, etto: f64) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Return the 6x6 state transformation matrix from one frame to another at a specified epoch.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn sxform(from: &str, to: &str, et: f64) -> [[f64; 6]; 6] {}
}

cspice_proc! {
    /**
    Look up the frame ID code associated with a frame name; zero when the name is not recognised.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn namfrm(frname: &str) -> i32 {}
}

cspice_proc! {
    /**
    Look up the name of a reference frame associated with an ID code.

    This function has a [neat version][crate::neat::frmnam].
    */
    pub fn frmnam(frcode: i32, #[lenout] lenout: i32) -> String {}
}

/* -------------------------------------------------------------------------------------------- */
/* Coordinates                                                                                    */
/* -------------------------------------------------------------------------------------------- */

cspice_proc! {
    /**
    Convert geodetic coordinates to rectangular coordinates.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn georec(lon: f64, lat: f64, alt: f64, re: f64, f: f64) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Convert rectangular coordinates to geodetic coordinates, as longitude, latitude and altitude.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn recgeo(rectan: [f64; 3], re: f64, f: f64) -> (f64, f64, f64) {}
}

cspice_proc! {
    /**
    Convert from latitudinal coordinates to rectangular coordinates.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn latrec(radius: f64, longitude: f64, latitude: f64) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Convert from rectangular coordinates to latitudinal coordinates, as radius, longitude and
    latitude.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn reclat(rectan: [f64; 3]) -> (f64, f64, f64) {}
}

cspice_proc! {
    /**
    Convert from spherical coordinates to rectangular coordinates.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn sphrec(r: f64, colat: f64, lon: f64) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Convert from rectangular coordinates to spherical coordinates, as radius, colatitude and
    longitude.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn recsph(rectan: [f64; 3]) -> (f64, f64, f64) {}
}

cspice_proc! {
    /**
    Convert from cylindrical coordinates to rectangular coordinates.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn cylrec(r: f64, lon: f64, z: f64) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Convert from rectangular coordinates to cylindrical coordinates, as radius, longitude and `z`.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn reccyl(rectan: [f64; 3]) -> (f64, f64, f64) {}
}

cspice_proc! {
    /**
    Convert planetographic coordinates to rectangular coordinates.
    */
    #[allow(clippy::too_many_arguments)]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn pgrrec(body: &str, lon: f64, lat: f64, alt: f64, re: f64, f: f64) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Convert rectangular coordinates to planetographic coordinates, as longitude, latitude and
    altitude.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn recpgr(body: &str, rectan: [f64; 3], re: f64, f: f64) -> (f64, f64, f64) {}
}

cspice_proc! {
    /**
    Convert range, right ascension, and declination to rectangular coordinates.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn radrec(range: f64, ra: f64, dec: f64) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Convert rectangular coordinates to range, right ascension, and declination.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn recrad(rectan: [f64; 3]) -> (f64, f64, f64) {}
}

cspice_proc! {
    /**
    Convert planetocentric latitude and longitude of a surface point on a specified body to
    rectangular coordinates.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn srfrec(body: i32, lon: f64, lat: f64) -> [f64; 3] {}
}

/* -------------------------------------------------------------------------------------------- */
/* Vectors and matrices                                                                           */

cspice_proc! {
    /**
    Pack three scalars into a vector.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn vpack(x: f64, y: f64, z: f64) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Unpack a vector into three scalars.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn vupack(v: [f64; 3]) -> (f64, f64, f64) {}
}

cspice_proc! {
    /**
    Whether a vector is the zero vector.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn vzero(v: [f64; 3]) -> bool {}
}

cspice_proc! {
    /**
    The component of `a` perpendicular to `b`.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn vperp(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    The projection of `a` onto `b`.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn vproj(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Rotate a vector about an axis by a given angle.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn vrotv(v: [f64; 3], axis: [f64; 3], theta: f64) -> [f64; 3] {}
}

cspice_proc! {
    /**
    The linear combination `a * v1 + b * v2`.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn vlcom(a: f64, v1: [f64; 3], b: f64, v2: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    The linear combination `a * v1 + b * v2 + c * v3`.
    */
    #[allow(clippy::too_many_arguments)]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn vlcom3(
        a: f64,
        v1: [f64; 3],
        b: f64,
        v2: [f64; 3],
        c: f64,
        v3: [f64; 3]
    ) -> [f64; 3] {
    }
}

cspice_proc! {
    /**
    The quadratic form `v1 * matrix * v2`.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn vtmv(v1: [f64; 3], matrix: [[f64; 3]; 3], v2: [f64; 3]) -> f64 {}
}

cspice_proc! {
    /**
    The 3x3 identity matrix.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn ident() -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Copy a 3x3 matrix.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn mequ(m1: [[f64; 3]; 3]) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Transpose a 6x6 matrix.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn xpose6(m1: [[f64; 6]; 6]) -> [[f64; 6]; 6] {}
}

/* -------------------------------------------------------------------------------------------- */
/* Vectors and matrices of arbitrary dimension                                                    */
/* -------------------------------------------------------------------------------------------- */

/*
CSPICE takes the dimensions as separate arguments and writes the result through a bare pointer, so
these cannot go through the macro: the length of the output is only known at run time. Each wrapper
takes the dimensions from the slices it is given and sizes the result itself.
*/

/// Generate the wrappers of the `v*g` routines that map one or two vectors to another.
macro_rules! vector_g {
    ($($name:ident($($arg:ident),*) => $cname:ident, $doc:expr);* $(;)?) => {$(
        #[doc = $doc]
        ///
        /// # Panics
        ///
        /// Panics if the vectors have different lengths.
        #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
        pub fn $name($($arg: &[f64]),*) -> Vec<f64> {
            let ndim = same_length(&[$($arg),*]);
            let mut vout = vec![0.0; ndim];
            unsafe {
                crate::c::$cname(
                    $($arg.as_ptr() as *mut SpiceDouble,)*
                    ndim as SpiceInt,
                    vout.as_mut_ptr(),
                );
            }
            vout
        }
    )*};
}

/// Generate the wrappers of the `v*g` routines that reduce one or two vectors to a scalar.
macro_rules! scalar_g {
    ($($name:ident($($arg:ident),*) -> $ret:ty => $cname:ident, $conv:expr, $doc:expr);* $(;)?) => {$(
        #[doc = $doc]
        ///
        /// # Panics
        ///
        /// Panics if the vectors have different lengths.
        #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
        pub fn $name($($arg: &[f64]),*) -> $ret {
            let ndim = same_length(&[$($arg),*]);
            let returned = unsafe {
                crate::c::$cname($($arg.as_ptr() as *mut SpiceDouble,)* ndim as SpiceInt)
            };
            #[allow(clippy::redundant_closure_call)]
            ($conv)(returned)
        }
    )*};
}

/// The common length of a set of slices.
fn same_length(slices: &[&[f64]]) -> usize {
    let ndim = slices[0].len();
    assert!(
        slices.iter().all(|slice| slice.len() == ndim),
        "the vectors must all have the same length, got {:?}",
        slices.iter().map(|slice| slice.len()).collect::<Vec<_>>()
    );
    ndim
}

vector_g! {
    vaddg(v1, v2) => vaddg_c, "Add two vectors of arbitrary dimension.";
    vsubg(v1, v2) => vsubg_c, "Subtract one vector of arbitrary dimension from another.";
    vhatg(v1) => vhatg_c, "The unit vector along a vector of arbitrary dimension.";
    vequg(vin) => vequg_c, "Copy a vector of arbitrary dimension.";
    vminug(vin) => vminug_c, "Negate a vector of arbitrary dimension.";
    vprojg(a, b) => vprojg_c, "The projection of one vector onto another, in arbitrary dimension.";
}

scalar_g! {
    vdotg(v1, v2) -> f64 => vdotg_c, |value| value,
        "The dot product of two vectors of arbitrary dimension.";
    vnormg(v1) -> f64 => vnormg_c, |value| value,
        "The magnitude of a vector of arbitrary dimension.";
    vdistg(v1, v2) -> f64 => vdistg_c, |value| value,
        "The distance between two vectors of arbitrary dimension.";
    vrelg(v1, v2) -> f64 => vrelg_c, |value| value,
        "The relative difference between two vectors of arbitrary dimension.";
    vsepg(v1, v2) -> f64 => vsepg_c, |value| value,
        "The angular separation of two vectors of arbitrary dimension.";
    vzerog(v) -> bool => vzerog_c, |value: crate::c::SpiceBoolean| value != 0,
        "Whether a vector of arbitrary dimension is the zero vector.";
}

/**
Scale a vector of arbitrary dimension.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn vsclg(s: f64, v1: &[f64]) -> Vec<f64> {
    let mut vout = vec![0.0; v1.len()];
    unsafe {
        crate::c::vsclg_c(
            s,
            v1.as_ptr() as *mut SpiceDouble,
            v1.len() as SpiceInt,
            vout.as_mut_ptr(),
        )
    };
    vout
}

/**
Normalize a vector of arbitrary dimension, returning the unit vector and the original magnitude.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn unormg(v1: &[f64]) -> (Vec<f64>, f64) {
    let mut vout = vec![0.0; v1.len()];
    let mut vmag = 0.0;
    unsafe {
        crate::c::unormg_c(
            v1.as_ptr() as *mut SpiceDouble,
            v1.len() as SpiceInt,
            vout.as_mut_ptr(),
            &mut vmag,
        )
    };
    (vout, vmag)
}

/**
The linear combination `a * v1 + b * v2`, in arbitrary dimension.

# Panics

Panics if the vectors have different lengths.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn vlcomg(a: f64, v1: &[f64], b: f64, v2: &[f64]) -> Vec<f64> {
    let ndim = same_length(&[v1, v2]);
    let mut sum = vec![0.0; ndim];
    unsafe {
        crate::c::vlcomg_c(
            ndim as SpiceInt,
            a,
            v1.as_ptr() as *mut SpiceDouble,
            b,
            v2.as_ptr() as *mut SpiceDouble,
            sum.as_mut_ptr(),
        )
    };
    sum
}

/**
Copy `ndim` elements of an array.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn moved(arrfrm: &[f64]) -> Vec<f64> {
    let mut arrto = vec![0.0; arrfrm.len()];
    unsafe {
        crate::c::moved_c(
            arrfrm.as_ptr() as *mut SpiceDouble,
            arrfrm.len() as SpiceInt,
            arrto.as_mut_ptr(),
        )
    };
    arrto
}

/**
The quadratic form `v1 * matrix * v2`, in arbitrary dimension.

`matrix` is `nrow` by `ncol`, stored row by row.

# Panics

Panics if the lengths do not match the dimensions.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn vtmvg(v1: &[f64], matrix: &[f64], v2: &[f64], nrow: usize, ncol: usize) -> f64 {
    assert_matrix(matrix, nrow, ncol);
    assert!(
        v1.len() == nrow && v2.len() == ncol,
        "vtmvg needs a vector of {nrow} and one of {ncol}, got {} and {}",
        v1.len(),
        v2.len()
    );

    unsafe {
        crate::c::vtmvg_c(
            v1.as_ptr().cast(),
            matrix.as_ptr().cast(),
            v2.as_ptr().cast(),
            nrow as SpiceInt,
            ncol as SpiceInt,
        )
    }
}

/// Check that a slice holds a `rows` by `cols` matrix, stored row by row.
fn assert_matrix(matrix: &[f64], rows: usize, cols: usize) {
    assert!(
        matrix.len() == rows * cols,
        "a {rows}x{cols} matrix needs {} elements, got {}",
        rows * cols,
        matrix.len()
    );
}

/*
CSPICE names the dimensions of these after what they mean rather than after their position, and
the meaning differs between the three. Spelling them out, and asserting the shapes, is the only way
a caller can tell what to pass: getting `mtxmg` wrong the other way round produces a plausible
looking matrix rather than an error.
*/

/**
Multiply a `nr1` by `nc1r2` matrix with a `nc1r2` by `nc2` one, giving `nr1` by `nc2`.

The matrices are stored row by row.

# Panics

Panics if the slice lengths do not match the dimensions.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn mxmg(m1: &[f64], m2: &[f64], nr1: usize, nc1r2: usize, nc2: usize) -> Vec<f64> {
    assert_matrix(m1, nr1, nc1r2);
    assert_matrix(m2, nc1r2, nc2);
    let mut mout = vec![0.0; nr1 * nc2];
    unsafe {
        crate::c::mxmg_c(
            m1.as_ptr().cast(),
            m2.as_ptr().cast(),
            nr1 as SpiceInt,
            nc1r2 as SpiceInt,
            nc2 as SpiceInt,
            mout.as_mut_ptr().cast(),
        )
    };
    mout
}

/**
Multiply the transpose of a `nr1r2` by `nc1` matrix with a `nr1r2` by `nc2` one, giving `nc1` by
`nc2`.

The matrices are stored row by row.

# Panics

Panics if the slice lengths do not match the dimensions.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn mtxmg(m1: &[f64], m2: &[f64], nc1: usize, nr1r2: usize, nc2: usize) -> Vec<f64> {
    assert_matrix(m1, nr1r2, nc1);
    assert_matrix(m2, nr1r2, nc2);
    let mut mout = vec![0.0; nc1 * nc2];
    unsafe {
        crate::c::mtxmg_c(
            m1.as_ptr().cast(),
            m2.as_ptr().cast(),
            nc1 as SpiceInt,
            nr1r2 as SpiceInt,
            nc2 as SpiceInt,
            mout.as_mut_ptr().cast(),
        )
    };
    mout
}

/**
Multiply a `nr1` by `nc1c2` matrix with the transpose of a `nr2` by `nc1c2` one, giving `nr1` by
`nr2`.

The matrices are stored row by row.

# Panics

Panics if the slice lengths do not match the dimensions.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn mxmtg(m1: &[f64], m2: &[f64], nr1: usize, nc1c2: usize, nr2: usize) -> Vec<f64> {
    assert_matrix(m1, nr1, nc1c2);
    assert_matrix(m2, nr2, nc1c2);
    let mut mout = vec![0.0; nr1 * nr2];
    unsafe {
        crate::c::mxmtg_c(
            m1.as_ptr().cast(),
            m2.as_ptr().cast(),
            nr1 as SpiceInt,
            nc1c2 as SpiceInt,
            nr2 as SpiceInt,
            mout.as_mut_ptr().cast(),
        )
    };
    mout
}

/**
Multiply a matrix by a vector, in arbitrary dimension.

`m1` is `nrow1` by `nc1r2`, stored row by row.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn mxvg(m1: &[f64], v2: &[f64], nrow1: usize, nc1r2: usize) -> Vec<f64> {
    assert_matrix(m1, nrow1, nc1r2);
    assert_matrix(v2, nc1r2, 1);
    let mut vout = vec![0.0; nrow1];
    unsafe {
        crate::c::mxvg_c(
            m1.as_ptr().cast(),
            v2.as_ptr().cast(),
            nrow1 as SpiceInt,
            nc1r2 as SpiceInt,
            vout.as_mut_ptr().cast(),
        )
    };
    vout
}

/**
Multiply the transpose of a matrix by a vector, in arbitrary dimension.

`m1` is `nr1r2` by `ncol1`, stored row by row.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn mtxvg(m1: &[f64], v2: &[f64], ncol1: usize, nr1r2: usize) -> Vec<f64> {
    assert_matrix(m1, nr1r2, ncol1);
    assert_matrix(v2, nr1r2, 1);
    let mut vout = vec![0.0; ncol1];
    unsafe {
        crate::c::mtxvg_c(
            m1.as_ptr().cast(),
            v2.as_ptr().cast(),
            ncol1 as SpiceInt,
            nr1r2 as SpiceInt,
            vout.as_mut_ptr().cast(),
        )
    };
    vout
}

/**
Transpose a matrix of arbitrary dimension, stored row by row.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn xposeg(matrix: &[f64], nrow: usize, ncol: usize) -> Vec<f64> {
    assert_matrix(matrix, nrow, ncol);
    let mut xposem = vec![0.0; nrow * ncol];
    unsafe {
        crate::c::xposeg_c(
            matrix.as_ptr().cast(),
            nrow as SpiceInt,
            ncol as SpiceInt,
            xposem.as_mut_ptr().cast(),
        )
    };
    xposem
}

/**
Copy a matrix of arbitrary dimension, stored row by row.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn mequg(m1: &[f64], nr: usize, nc: usize) -> Vec<f64> {
    assert_matrix(m1, nr, nc);
    let mut mout = vec![0.0; nr * nc];
    unsafe {
        crate::c::mequg_c(
            m1.as_ptr().cast(),
            nr as SpiceInt,
            nc as SpiceInt,
            mout.as_mut_ptr().cast(),
        )
    };
    mout
}

/* -------------------------------------------------------------------------------------------- */
/* Coordinate conversions between non-rectangular systems                                         */
/* -------------------------------------------------------------------------------------------- */

cspice_proc! {
    /**
    Convert from latitudinal to cylindrical coordinates.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn latcyl(radius: f64, lon: f64, lat: f64) -> (f64, f64, f64) {}
}

cspice_proc! {
    /**
    Convert from cylindrical to latitudinal coordinates.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn cyllat(r: f64, lonc: f64, z: f64) -> (f64, f64, f64) {}
}

cspice_proc! {
    /**
    Convert from latitudinal to spherical coordinates.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn latsph(radius: f64, lon: f64, lat: f64) -> (f64, f64, f64) {}
}

cspice_proc! {
    /**
    Convert from spherical to latitudinal coordinates.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn sphlat(r: f64, colat: f64, lons: f64) -> (f64, f64, f64) {}
}

cspice_proc! {
    /**
    Convert from cylindrical to spherical coordinates.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn cylsph(r: f64, lonc: f64, z: f64) -> (f64, f64, f64) {}
}

cspice_proc! {
    /**
    Convert from spherical to cylindrical coordinates.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn sphcyl(radius: f64, colat: f64, slon: f64) -> (f64, f64, f64) {}
}

cspice_proc! {
    /**
    Convert from range, azimuth and elevation to rectangular coordinates.

    `azccw` says whether azimuth increases counterclockwise, `elplsz` whether elevation is positive
    toward `+z`.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn azlrec(range: f64, az: f64, el: f64, azccw: bool, elplsz: bool) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Convert from rectangular coordinates to range, azimuth and elevation.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn recazl(rectan: [f64; 3], azccw: bool, elplsz: bool) -> (f64, f64, f64) {}
}

/* -------------------------------------------------------------------------------------------- */
/* Jacobians of the coordinate conversions                                                        */
/* -------------------------------------------------------------------------------------------- */

cspice_proc! {
    /**
    Jacobian of the transformation from rectangular to latitudinal coordinates.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dlatdr(x: f64, y: f64, z: f64) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Jacobian of the transformation from latitudinal to rectangular coordinates.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn drdlat(r: f64, lon: f64, lat: f64) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Jacobian of the transformation from rectangular to spherical coordinates.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dsphdr(x: f64, y: f64, z: f64) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Jacobian of the transformation from spherical to rectangular coordinates.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn drdsph(r: f64, colat: f64, lon: f64) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Jacobian of the transformation from rectangular to cylindrical coordinates.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dcyldr(x: f64, y: f64, z: f64) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Jacobian of the transformation from cylindrical to rectangular coordinates.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn drdcyl(r: f64, lon: f64, z: f64) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Jacobian of the transformation from rectangular to geodetic coordinates.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dgeodr(x: f64, y: f64, z: f64, re: f64, f: f64) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Jacobian of the transformation from geodetic to rectangular coordinates.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn drdgeo(lon: f64, lat: f64, alt: f64, re: f64, f: f64) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Jacobian of the transformation from rectangular to planetographic coordinates.
    */
    #[allow(clippy::too_many_arguments)]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dpgrdr(body: &str, x: f64, y: f64, z: f64, re: f64, f: f64) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Jacobian of the transformation from planetographic to rectangular coordinates.
    */
    #[allow(clippy::too_many_arguments)]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn drdpgr(body: &str, lon: f64, lat: f64, alt: f64, re: f64, f: f64) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Jacobian of the transformation from rectangular to azimuth/elevation coordinates.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dazldr(x: f64, y: f64, z: f64, azccw: bool, elplsz: bool) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Jacobian of the transformation from azimuth/elevation to rectangular coordinates.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn drdazl(range: f64, az: f64, el: f64, azccw: bool, elplsz: bool) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Transform a state between two coordinate systems.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn xfmsta(
        input_state: [f64; 6],
        input_coord_sys: &str,
        output_coord_sys: &str,
        body: &str
    ) -> [f64; 6] {
    }
}

/* -------------------------------------------------------------------------------------------- */

cspice_proc! {
    /**
    Add two 3-dimensional vectors.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn vadd(v1: [f64; 3], v2: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Subtract one 3-dimensional vector from another.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn vsub(v1: [f64; 3], v2: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Multiply a 3-dimensional vector by a scalar.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn vscl(s: f64, v1: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Copy a 3-dimensional vector.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn vequ(vin: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Negate a 3-dimensional vector.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn vminus(v1: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Find the unit vector along a 3-dimensional vector.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn vhat(v1: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Normalize a 3-dimensional vector, returning the unit vector and the original magnitude.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn unorm(v1: [f64; 3]) -> ([f64; 3], f64) {}
}

cspice_proc! {
    /**
    Compute the magnitude of a 3-dimensional vector.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn vnorm(v1: [f64; 3]) -> f64 {}
}

cspice_proc! {
    /**
    Compute the distance between two 3-dimensional vectors.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn vdist(v1: [f64; 3], v2: [f64; 3]) -> f64 {}
}

cspice_proc! {
    /**
    Compute the relative difference between two 3-dimensional vectors.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn vrel(v1: [f64; 3], v2: [f64; 3]) -> f64 {}
}

cspice_proc! {
    /**
    Compute the dot product of two double precision, 3-dimensional vectors.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn vdot(v1: [f64; 3], v2: [f64; 3]) -> f64 {}
}

cspice_proc! {
    /**
    Compute the cross product of two 3-dimensional vectors.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn vcrss(v1: [f64; 3], v2: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Find the separation angle in radians between two double precision, 3-dimensional vectors. This
    angle is defined as zero if either vector is zero.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn vsep(v1: [f64; 3], v2: [f64; 3]) -> f64 {}
}

cspice_proc! {
    /**
    Multiply a 3x3 double precision matrix with a 3-dimensional double precision vector.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn mxv(m1: [[f64; 3]; 3], vin: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Multiply the transpose of a 3x3 matrix with a 3-dimensional vector.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn mtxv(m1: [[f64; 3]; 3], vin: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Multiply two 3x3 matrices.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn mxm(m1: [[f64; 3]; 3], m2: [[f64; 3]; 3]) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Multiply a 3x3 matrix by the transpose of another.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn mxmt(m1: [[f64; 3]; 3], m2: [[f64; 3]; 3]) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Multiply the transpose of a 3x3 matrix by another 3x3 matrix.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn mtxm(m1: [[f64; 3]; 3], m2: [[f64; 3]; 3]) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Transpose a 3x3 matrix.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn xpose(m1: [[f64; 3]; 3]) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Invert a 3x3 matrix.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn invert(m1: [[f64; 3]; 3]) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Compute the determinant of a 3x3 matrix.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn det(m1: [[f64; 3]; 3]) -> f64 {}
}

cspice_proc! {
    /**
    Compute the trace of a 3x3 matrix.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn trace(matrix: [[f64; 3]; 3]) -> f64 {}
}

cspice_proc! {
    /**
    Find the transformation to the right-handed frame having a given vector as a specified axis and
    a second given vector lying in a specified coordinate plane.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn twovec(axdef: [f64; 3], indexa: i32, plndef: [f64; 3], indexp: i32) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Calculate the 3x3 rotation matrix generated by a rotation of a specified angle about a specified
    axis. This rotation is thought of as rotating the coordinate system.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn rotate(angle: f64, iaxis: i32) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Apply a rotation of `angle` radians about axis `iaxis` to a matrix.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn rotmat(m1: [[f64; 3]; 3], angle: f64, iaxis: i32) -> [[f64; 3]; 3] {}
}

/* -------------------------------------------------------------------------------------------- */
/* Rotations                                                                                      */
/* -------------------------------------------------------------------------------------------- */

cspice_proc! {
    /**
    Find the unit quaternion corresponding to a specified rotation matrix.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn m2q(r: [[f64; 3]; 3]) -> [f64; 4] {}
}

cspice_proc! {
    /**
    Find the rotation matrix corresponding to a specified unit quaternion.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn q2m(q: [f64; 4]) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Multiply two quaternions.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn qxq(q1: [f64; 4], q2: [f64; 4]) -> [f64; 4] {}
}

cspice_proc! {
    /**
    Construct a rotation matrix that rotates vectors by a specified angle about a specified axis.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn axisar(axis: [f64; 3], angle: f64) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Compute the axis of the rotation a matrix represents, and the angle about that axis.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn raxisa(matrix: [[f64; 3]; 3]) -> ([f64; 3], f64) {}
}

cspice_proc! {
    /**
    Construct a rotation matrix from a set of Euler angles.
    */
    #[allow(clippy::too_many_arguments)]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn eul2m(
        angle3: f64,
        angle2: f64,
        angle1: f64,
        axis3: i32,
        axis2: i32,
        axis1: i32
    ) -> [[f64; 3]; 3] {
    }
}

cspice_proc! {
    /**
    Factor a rotation matrix as a product of three rotations about specified coordinate axes,
    returning the three angles in the order `angle3, angle2, angle1`.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn m2eul(r: [[f64; 3]; 3], axis3: i32, axis2: i32, axis1: i32) -> (f64, f64, f64) {}
}

cspice_proc! {
    /**
    Construct a state transformation matrix from a set of Euler angles and their derivatives.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn eul2xf(eulang: [f64; 6], axisa: i32, axisb: i32, axisc: i32) -> [[f64; 6]; 6] {}
}

cspice_proc! {
    /**
    Factor a state transformation matrix into Euler angles and their derivatives.

    The boolean says whether the factorisation is unique.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn xf2eul(xform: [[f64; 6]; 6], axisa: i32, axisb: i32, axisc: i32) -> ([f64; 6], bool) {}
}

cspice_proc! {
    /**
    Split a state transformation matrix into a rotation and the angular velocity of that rotation.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn xf2rav(xform: [[f64; 6]; 6]) -> ([[f64; 3]; 3], [f64; 3]) {}
}

cspice_proc! {
    /**
    Build a state transformation matrix from a rotation and an angular velocity.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn rav2xf(rot: [[f64; 3]; 3], av: [f64; 3]) -> [[f64; 6]; 6] {}
}

cspice_proc! {
    /**
    Compute the inverse of a 3x3 matrix whose rows are orthogonal but not necessarily unit length.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn invort(m: [[f64; 3]; 3]) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Decide whether a matrix is a rotation matrix, to within the given norm and determinant
    tolerances.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn isrot(m: [[f64; 3]; 3], ntol: f64, dtol: f64) -> bool {}
}

/* -------------------------------------------------------------------------------------------- */
/* SPK                                                                                            */
/* -------------------------------------------------------------------------------------------- */

cspice_proc! {
    /**
    Return the position of a target body relative to an observing body, optionally corrected for
    light time (planetary aberration) and stellar aberration.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn spkpos(targ: &str, et: f64, frame: &str, abcorr: &str, obs: &str) -> ([f64; 3], f64) {}
}

cspice_proc! {
    /**
    Return the state (position and velocity) of a target body relative to an observing body,
    optionally corrected for light time (planetary aberration) and stellar aberration.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn spkezr(targ: &str, et: f64, frame: &str, abcorr: &str, obs: &str) -> ([f64; 6], f64) {}
}

cspice_proc! {
    /**
    Return the state of a target body relative to an observing body, both given by their ID codes.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn spkez(targ: i32, et: f64, frame: &str, abcorr: &str, obs: i32) -> ([f64; 6], f64) {}
}

cspice_proc! {
    /**
    Return the position of a target body relative to an observing body, both given by their ID
    codes.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn spkezp(targ: i32, et: f64, frame: &str, abcorr: &str, obs: i32) -> ([f64; 3], f64) {}
}

cspice_proc! {
    /**
    Compute the geometric state of a target body relative to an observing body.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn spkgeo(targ: i32, et: f64, frame: &str, obs: i32) -> ([f64; 6], f64) {}
}

cspice_proc! {
    /**
    Return the state of a specified target relative to an observer that is described by a constant
    position in a specified reference frame.
    */
    #[allow(clippy::too_many_arguments)]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn spkcpo(
        target: &str,
        et: f64,
        outref: &str,
        refloc: &str,
        abcorr: &str,
        obssta: [f64; 6],
        obsctr: &str,
        obsref: &str
    ) -> ([f64; 6], f64) {
    }
}

cspice_proc! {
    /**
    Return the state of a target that is described by a constant position in a specified reference
    frame, relative to a specified observer.
    */
    #[allow(clippy::too_many_arguments)]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn spkcpt(
        trgpos: [f64; 3],
        trgctr: &str,
        trgref: &str,
        et: f64,
        outref: &str,
        refloc: &str,
        abcorr: &str,
        obsrvr: &str
    ) -> ([f64; 6], f64) {
    }
}

cspice_proc! {
    /**
    Return the state of a specified target relative to an observer that is described by a constant
    velocity in a specified reference frame.
    */
    #[allow(clippy::too_many_arguments)]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn spkcvo(
        target: &str,
        et: f64,
        outref: &str,
        refloc: &str,
        abcorr: &str,
        obssta: [f64; 6],
        obsepc: f64,
        obsctr: &str,
        obsref: &str
    ) -> ([f64; 6], f64) {
    }
}

cspice_proc! {
    /**
    Return the state of a target that is described by a constant velocity in a specified reference
    frame, relative to a specified observer.
    */
    #[allow(clippy::too_many_arguments)]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn spkcvt(
        trgsta: [f64; 6],
        trgepc: f64,
        trgctr: &str,
        trgref: &str,
        et: f64,
        outref: &str,
        refloc: &str,
        abcorr: &str,
        obsrvr: &str
    ) -> ([f64; 6], f64) {
    }
}

cspice_proc! {
    /**
    Find the set of ID codes of all objects in a specified SPK file.

    This function has a [neat version][crate::neat::spkobj].
    */
    pub fn spkobj(spk: &str, ids: &mut Cell<i32>) {}
}

cspice_proc! {
    /**
    Find the coverage window for a specified ephemeris object in a specified SPK file.

    This function has a [neat version][crate::neat::spkcov].
    */
    pub fn spkcov(spk: &str, idcode: i32, cover: &mut Cell<f64>) {}
}

cspice_proc! {
    /**
    Create a new SPK file, returning the handle of the opened file.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn spkopn(fname: &str, ifname: &str, ncomch: i32) -> i32 {}
}

cspice_proc! {
    /**
    Open an existing SPK file for subsequent write, returning its handle.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn spkopa(fname: &str) -> i32 {}
}

cspice_proc! {
    /**
    Close a SPK file opened for read or write.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn spkcls(handle: i32) {}
}

/**
Write a type 9 segment to an SPK file.

# Panics

Panics if `n` is negative, or larger than `states` or `epochs`: CSPICE reads exactly `n` records
from each and has no way of knowing how long they are.
*/
#[allow(clippy::too_many_arguments)]
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn spkw09(
    handle: i32,
    body: i32,
    center: i32,
    frame: &str,
    first: f64,
    last: f64,
    segid: &str,
    degree: i32,
    n: i32,
    states: &[[f64; 6]],
    epochs: &[f64],
) {
    let count = usize::try_from(n).expect("the number of states cannot be negative");
    assert!(
        count <= states.len() && count <= epochs.len(),
        "spkw09 was asked for {count} records but got {} states and {} epochs",
        states.len(),
        epochs.len()
    );

    let frame = to_cstring(frame);
    let segid = to_cstring(segid);

    unsafe {
        crate::c::spkw09_c(
            handle,
            body,
            center,
            frame.as_ptr() as *mut SpiceChar,
            first,
            last,
            segid.as_ptr() as *mut SpiceChar,
            degree,
            n,
            states.as_ptr() as *mut [SpiceDouble; 6],
            epochs.as_ptr() as *mut SpiceDouble,
        );
    }
}

/* -------------------------------------------------------------------------------------------- */
/* Surface geometry                                                                               */
/* -------------------------------------------------------------------------------------------- */

cspice_proc! {
    /**
    Compute, for a given observer and a ray emanating from the observer, the surface intercept of
    the ray on a target body at a specified epoch, optionally corrected for light time and stellar
    aberration.

    The surface of the target body may be represented by a triaxial ellipsoid or by topographic data
    provided by DSK files.

    This routine supersedes `srfxpt`.
    */
    #[allow(clippy::too_many_arguments)]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn sincpt(
        method: &str,
        target: &str,
        et: f64,
        fixref: &str,
        abcorr: &str,
        obsrvr: &str,
        dref: &str,
        dvec: [f64; 3]
    ) -> ([f64; 3], f64, [f64; 3], bool) {
    }
}

cspice_proc! {
    /**
    Compute the rectangular coordinates of the sub-observer point on a target body at a specified
    epoch, optionally corrected for light time and stellar aberration.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn subpnt(
        method: &str,
        target: &str,
        et: f64,
        fixref: &str,
        abcorr: &str,
        obsrvr: &str
    ) -> ([f64; 3], f64, [f64; 3]) {
    }
}

cspice_proc! {
    /**
    Compute the rectangular coordinates of the sub-solar point on a target body at a specified
    epoch, optionally corrected for light time and stellar aberration.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn subslr(
        method: &str,
        target: &str,
        et: f64,
        fixref: &str,
        abcorr: &str,
        obsrvr: &str
    ) -> ([f64; 3], f64, [f64; 3]) {
    }
}

cspice_proc! {
    /**
    Find the illumination angles---phase, solar incidence, and emission---at a specified surface
    point of a target body.
    */
    #[allow(clippy::too_many_arguments)]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn ilumin(
        method: &str,
        target: &str,
        et: f64,
        fixref: &str,
        abcorr: &str,
        obsrvr: &str,
        spoint: [f64; 3]
    ) -> (f64, [f64; 3], f64, f64, f64) {
    }
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
    #[allow(clippy::too_many_arguments)]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn illumf(
        method: &str,
        target: &str,
        ilusrc: &str,
        et: f64,
        fixref: &str,
        abcorr: &str,
        obsrvr: &str,
        spoint: [f64; 3]
    ) -> (f64, [f64; 3], f64, f64, f64, bool, bool) {
    }
}

cspice_proc! {
    /**
    Determine the intersection of a line-of-sight vector with the surface of an ellipsoid.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn surfpt(positn: [f64; 3], u: [f64; 3], a: f64, b: f64, c: f64) -> ([f64; 3], bool) {}
}

cspice_proc! {
    /**
    Locate the point on the surface of an ellipsoid that is nearest to a specified position, and
    the altitude of that position above the ellipsoid.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn nearpt(positn: [f64; 3], a: f64, b: f64, c: f64) -> ([f64; 3], f64) {}
}

cspice_proc! {
    /**
    Determine the occultation condition (not occulted, partially, etc.) of one target relative to
    another target as seen by an observer at a given time, with targets modeled as points,
    ellipsoids, or digital shapes (DSK).
    */
    #[allow(clippy::too_many_arguments)]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn occult(
        targ1: &str,
        shape1: &str,
        frame1: &str,
        targ2: &str,
        shape2: &str,
        frame2: &str,
        abcorr: &str,
        obsrvr: &str,
        et: f64
    ) -> i32 {
    }
}

cspice_proc! {
    /**
    Compute the apparent phase angle for a target, observer, illuminator set of ephemeris objects.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn phaseq(et: f64, target: &str, illumn: &str, obsrvr: &str, abcorr: &str) -> f64 {}
}

/**
Map an array of surface points on a target body to the corresponding unit length outward surface
normal vectors.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn srfnrm(
    method: &str,
    target: &str,
    et: f64,
    fixref: &str,
    srfpts: &[[f64; 3]],
) -> Vec<[f64; 3]> {
    let method = to_cstring(method);
    let target = to_cstring(target);
    let fixref = to_cstring(fixref);
    let mut normls = vec![[0.0; 3]; srfpts.len()];

    unsafe {
        crate::c::srfnrm_c(
            method.as_ptr() as *mut SpiceChar,
            target.as_ptr() as *mut SpiceChar,
            et,
            fixref.as_ptr() as *mut SpiceChar,
            srfpts.len() as SpiceInt,
            srfpts.as_ptr() as *mut [f64; 3],
            normls.as_mut_ptr(),
        );
    }

    normls
}

/**
Map an array of planetocentric longitude/latitude pairs to the corresponding surface points on a
target body.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn latsrf(
    method: &str,
    target: &str,
    et: f64,
    fixref: &str,
    lonlat: &[[f64; 2]],
) -> Vec<[f64; 3]> {
    let method = to_cstring(method);
    let target = to_cstring(target);
    let fixref = to_cstring(fixref);
    let mut srfpts = vec![[0.0; 3]; lonlat.len()];

    unsafe {
        crate::c::latsrf_c(
            method.as_ptr() as *mut SpiceChar,
            target.as_ptr() as *mut SpiceChar,
            et,
            fixref.as_ptr() as *mut SpiceChar,
            lonlat.len() as SpiceInt,
            lonlat.as_ptr() as *mut [f64; 2],
            srfpts.as_mut_ptr(),
        );
    }

    srfpts
}

/**
Compute a set of points on the umbral or penumbral terminator of a specified target body, where the
target shape is modelled as an ellipsoid.
*/
#[allow(clippy::too_many_arguments)]
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn edterm(
    trmtyp: &str,
    source: &str,
    target: &str,
    et: f64,
    fixfrm: &str,
    abcorr: &str,
    obsrvr: &str,
    npts: usize,
) -> (f64, [f64; 3], Vec<[f64; 3]>) {
    let trmtyp = to_cstring(trmtyp);
    let source = to_cstring(source);
    let target = to_cstring(target);
    let fixfrm = to_cstring(fixfrm);
    let abcorr = to_cstring(abcorr);
    let obsrvr = to_cstring(obsrvr);

    let mut trgepc = 0.0;
    let mut obspos = [0.0; 3];
    let mut termpts = vec![[0.0; 3]; npts];

    unsafe {
        crate::c::edterm_c(
            trmtyp.as_ptr() as *mut SpiceChar,
            source.as_ptr() as *mut SpiceChar,
            target.as_ptr() as *mut SpiceChar,
            et,
            fixfrm.as_ptr() as *mut SpiceChar,
            abcorr.as_ptr() as *mut SpiceChar,
            obsrvr.as_ptr() as *mut SpiceChar,
            npts as SpiceInt,
            &mut trgepc,
            obspos.as_mut_ptr(),
            termpts.as_mut_ptr(),
        );
    }

    (trgepc, obspos, termpts)
}

/**
Find limb points on a target body, on cuts of a half-plane pencil rotating about the observer-target
vector.

Returns, per cut, the number of points found, then the points themselves, the epoch associated with
each, and the tangent vector from the observer to each.
*/
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn limbpt(
    method: &str,
    target: &str,
    et: f64,
    fixref: &str,
    abcorr: &str,
    corloc: &str,
    obsrvr: &str,
    refvec: [f64; 3],
    rolstp: f64,
    ncuts: usize,
    schstp: f64,
    soltol: f64,
    maxn: usize,
) -> (Vec<i32>, Vec<[f64; 3]>, Vec<f64>, Vec<[f64; 3]>) {
    let method = to_cstring(method);
    let target = to_cstring(target);
    let fixref = to_cstring(fixref);
    let abcorr = to_cstring(abcorr);
    let corloc = to_cstring(corloc);
    let obsrvr = to_cstring(obsrvr);

    let mut npts = vec![0; ncuts.max(1)];
    let mut points = vec![[0.0; 3]; maxn];
    let mut epochs = vec![0.0; maxn];
    let mut tangts = vec![[0.0; 3]; maxn];

    unsafe {
        crate::c::limbpt_c(
            method.as_ptr() as *mut SpiceChar,
            target.as_ptr() as *mut SpiceChar,
            et,
            fixref.as_ptr() as *mut SpiceChar,
            abcorr.as_ptr() as *mut SpiceChar,
            corloc.as_ptr() as *mut SpiceChar,
            obsrvr.as_ptr() as *mut SpiceChar,
            refvec.as_ptr() as *mut SpiceDouble,
            rolstp,
            ncuts as SpiceInt,
            schstp,
            soltol,
            maxn as SpiceInt,
            npts.as_mut_ptr(),
            points.as_mut_ptr(),
            epochs.as_mut_ptr(),
            tangts.as_mut_ptr(),
        );
    }

    truncate_cuts(npts, points, epochs, tangts)
}

/**
Find terminator points on a target body, on cuts of a half-plane pencil rotating about the
observer-target vector.

Shaped like [`limbpt`], with the illumination source named separately.
*/
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn termpt(
    method: &str,
    ilusrc: &str,
    target: &str,
    et: f64,
    fixref: &str,
    abcorr: &str,
    corloc: &str,
    obsrvr: &str,
    refvec: [f64; 3],
    rolstp: f64,
    ncuts: usize,
    schstp: f64,
    soltol: f64,
    maxn: usize,
) -> (Vec<i32>, Vec<[f64; 3]>, Vec<f64>, Vec<[f64; 3]>) {
    let method = to_cstring(method);
    let ilusrc = to_cstring(ilusrc);
    let target = to_cstring(target);
    let fixref = to_cstring(fixref);
    let abcorr = to_cstring(abcorr);
    let corloc = to_cstring(corloc);
    let obsrvr = to_cstring(obsrvr);

    let mut npts = vec![0; ncuts.max(1)];
    let mut points = vec![[0.0; 3]; maxn];
    let mut epochs = vec![0.0; maxn];
    let mut tangts = vec![[0.0; 3]; maxn];

    unsafe {
        crate::c::termpt_c(
            method.as_ptr() as *mut SpiceChar,
            ilusrc.as_ptr() as *mut SpiceChar,
            target.as_ptr() as *mut SpiceChar,
            et,
            fixref.as_ptr() as *mut SpiceChar,
            abcorr.as_ptr() as *mut SpiceChar,
            corloc.as_ptr() as *mut SpiceChar,
            obsrvr.as_ptr() as *mut SpiceChar,
            refvec.as_ptr() as *mut SpiceDouble,
            rolstp,
            ncuts as SpiceInt,
            schstp,
            soltol,
            maxn as SpiceInt,
            npts.as_mut_ptr(),
            points.as_mut_ptr(),
            epochs.as_mut_ptr(),
            tangts.as_mut_ptr(),
        );
    }

    truncate_cuts(npts, points, epochs, tangts)
}

/// Cut the per-cut output arrays down to the number of points CSPICE actually wrote.
#[allow(clippy::type_complexity)]
fn truncate_cuts(
    npts: Vec<SpiceInt>,
    mut points: Vec<[f64; 3]>,
    mut epochs: Vec<f64>,
    mut tangts: Vec<[f64; 3]>,
) -> (Vec<i32>, Vec<[f64; 3]>, Vec<f64>, Vec<[f64; 3]>) {
    let found = npts.iter().map(|count| count.max(&0)).sum::<SpiceInt>() as usize;
    points.truncate(found);
    epochs.truncate(found);
    tangts.truncate(found);
    (npts, points, epochs, tangts)
}

/**
Return the field-of-view (FOV) parameters for a specified instrument. The instrument is specified by
its NAIF ID code.

This function has a [neat version][crate::neat::getfov].
*/
pub fn getfov(
    instid: i32,
    room: usize,
    shapelen: usize,
    framelen: usize,
) -> (String, String, [f64; 3], Vec<[f64; 3]>) {
    let mut shape = vec![0 as SpiceChar; shapelen.max(1)];
    let mut frame = vec![0 as SpiceChar; framelen.max(1)];
    let mut bsight = [0.0; 3];
    let mut n = 0;
    let mut bounds = vec![[0.0; 3]; room];

    unsafe {
        crate::c::getfov_c(
            instid,
            room as SpiceInt,
            shapelen as SpiceInt,
            framelen as SpiceInt,
            shape.as_mut_ptr(),
            frame.as_mut_ptr(),
            bsight.as_mut_ptr(),
            &mut n,
            bounds.as_mut_ptr(),
        )
    };

    bounds.truncate(n.max(0) as usize);
    (from_cbuf(&shape), from_cbuf(&frame), bsight, bounds)
}

/* -------------------------------------------------------------------------------------------- */
/* Planes and ellipses                                                                            */
/* -------------------------------------------------------------------------------------------- */

cspice_proc! {
    /**
    Build a plane from a normal vector and a constant.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn nvc2pl(normal: [f64; 3], constant: f64) -> PLANE {}
}

cspice_proc! {
    /**
    Build a plane from a normal vector and a point.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn nvp2pl(normal: [f64; 3], point: [f64; 3]) -> PLANE {}
}

cspice_proc! {
    /**
    Build a plane from a point and two spanning vectors.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn psv2pl(point: [f64; 3], span1: [f64; 3], span2: [f64; 3]) -> PLANE {}
}

cspice_proc! {
    /**
    Take a plane apart into a unit normal vector and a constant.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn pl2nvc(plane: PLANE) -> ([f64; 3], f64) {}
}

cspice_proc! {
    /**
    Take a plane apart into a unit normal vector and a point.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn pl2nvp(plane: PLANE) -> ([f64; 3], [f64; 3]) {}
}

cspice_proc! {
    /**
    Take a plane apart into a point and two spanning vectors.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn pl2psv(plane: PLANE) -> ([f64; 3], [f64; 3], [f64; 3]) {}
}

cspice_proc! {
    /**
    Build an ellipse from a centre and two generating vectors.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn cgv2el(center: [f64; 3], vec1: [f64; 3], vec2: [f64; 3]) -> ELLIPSE {}
}

cspice_proc! {
    /**
    Take an ellipse apart into its centre and its semi-major and semi-minor axes.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn el2cgv(ellipse: ELLIPSE) -> ([f64; 3], [f64; 3], [f64; 3]) {}
}

cspice_proc! {
    /**
    Find the semi-axes of the ellipse two vectors generate.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn saelgv(vec1: [f64; 3], vec2: [f64; 3]) -> ([f64; 3], [f64; 3]) {}
}

cspice_proc! {
    /**
    Find the intersection of an ellipsoid and a plane, when there is one.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn inedpl(a: f64, b: f64, c: f64, plane: PLANE) -> (ELLIPSE, bool) {}
}

cspice_proc! {
    /**
    Find the intersection of an ellipse and a plane: how many points there are, and the points.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn inelpl(ellips: ELLIPSE, plane: PLANE) -> (i32, [f64; 3], [f64; 3]) {}
}

cspice_proc! {
    /**
    Find the intersection of a ray and a plane: how many points there are, and the point.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn inrypl(vertex: [f64; 3], dir: [f64; 3], plane: PLANE) -> (i32, [f64; 3]) {}
}

cspice_proc! {
    /**
    Find the limb of an ellipsoid as seen from a viewing point.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn edlimb(a: f64, b: f64, c: f64, viewpt: [f64; 3]) -> ELLIPSE {}
}

cspice_proc! {
    /**
    Project an ellipse orthogonally onto a plane.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn pjelpl(elin: ELLIPSE, plane: PLANE) -> ELLIPSE {}
}

cspice_proc! {
    /**
    Project a vector orthogonally onto a plane.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn vprjp(vin: [f64; 3], plane: PLANE) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Invert an orthogonal projection: find the vector of `invpl` that projects onto `vin`.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn vprjpi(vin: [f64; 3], projpl: PLANE, invpl: PLANE) -> ([f64; 3], bool) {}
}

cspice_proc! {
    /**
    Find the point of an ellipse nearest a specified point, and the distance between them.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn npelpt(point: [f64; 3], ellips: ELLIPSE) -> ([f64; 3], f64) {}
}

cspice_proc! {
    /**
    Find the point of an ellipsoid nearest a line, and the distance between them.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn npedln(
        a: f64,
        b: f64,
        c: f64,
        linept: [f64; 3],
        linedr: [f64; 3]
    ) -> ([f64; 3], f64) {
    }
}

cspice_proc! {
    /**
    Find the point of a line nearest a specified point, and the distance between them.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn nplnpt(linpt: [f64; 3], lindir: [f64; 3], point: [f64; 3]) -> ([f64; 3], f64) {}
}

cspice_proc! {
    /**
    The outward normal of an ellipsoid at a point on its surface.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn surfnm(a: f64, b: f64, c: f64, point: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Find the state of the intersection of a ray with an ellipsoid, given the state of the ray.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn surfpv(
        stvrtx: [f64; 6],
        stdir: [f64; 6],
        a: f64,
        b: f64,
        c: f64
    ) -> ([f64; 6], bool) {
    }
}

/* -------------------------------------------------------------------------------------------- */
/* Surfaces                                                                                       */
/* -------------------------------------------------------------------------------------------- */

cspice_proc! {
    /**
    Translate a surface ID code, together with a body ID code, to the corresponding surface name.

    The boolean tells whether the returned string is a name rather than the string representation of
    the code.

    This function has a [neat version][crate::neat::srfc2s].
    */
    pub fn srfc2s(code: i32, bodyid: i32, #[lenout] srflen: i32) -> (String, bool) {}
}

cspice_proc! {
    /**
    Translate a surface ID code, together with a body string, to the corresponding surface name.

    This function has a [neat version][crate::neat::srfcss].
    */
    pub fn srfcss(code: i32, bodstr: &str, #[lenout] srflen: i32) -> (String, bool) {}
}

cspice_proc! {
    /**
    Translate a surface string, together with a body string, to the corresponding surface ID code.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn srfs2c(srfstr: &str, bodstr: &str) -> (i32, bool) {}
}

cspice_proc! {
    /**
    Translate a surface string, together with a body ID code, to the corresponding surface ID code.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn srfscc(surfce: &str, bodyid: i32) -> (i32, bool) {}
}

/* -------------------------------------------------------------------------------------------- */
/* PCK                                                                                            */
/* -------------------------------------------------------------------------------------------- */

cspice_proc! {
    /**
    Find the coverage window for a specified reference frame in a specified binary PCK file.

    This function has a [neat version][crate::neat::pckcov].
    */
    pub fn pckcov(pck: &str, idcode: i32, cover: &mut Cell<f64>) {}
}

cspice_proc! {
    /**
    Find the set of reference frame class ID codes of all frames in a specified binary PCK file.

    This function has a [neat version][crate::neat::pckfrm].
    */
    pub fn pckfrm(pck: &str, ids: &mut Cell<i32>) {}
}

/* -------------------------------------------------------------------------------------------- */
/* Two-body orbits                                                                                */
/* -------------------------------------------------------------------------------------------- */

cspice_proc! {
    /**
    Determine the state of a body from a set of elliptic, hyperbolic, or parabolic orbital elements.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn conics(elts: [f64; 8], et: f64) -> [f64; 6] {}
}

cspice_proc! {
    /**
    Determine the set of osculating conic orbital elements that corresponds to the state of a body
    at some epoch.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn oscelt(state: [f64; 6], et: f64, mu: f64) -> [f64; 8] {}
}

cspice_proc! {
    /**
    Like [`oscelt`], with the true anomaly, semi-major axis and orbital period appended.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn oscltx(state: [f64; 6], et: f64, mu: f64) -> [f64; 20] {}
}

cspice_proc! {
    /**
    Propagate a two body solution from one epoch to another.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn prop2b(gm: f64, pvinit: [f64; 6], dt: f64) -> [f64; 6] {}
}

/* -------------------------------------------------------------------------------------------- */
/* Windows                                                                                        */
/* -------------------------------------------------------------------------------------------- */

cspice_proc! {
    /**
    Insert the interval `[left, right]` into a double precision window.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn wninsd(left: f64, right: f64, window: &mut Cell<f64>) {}
}

cspice_proc! {
    /**
    Fetch the endpoints of the `n`th interval of a double precision window, counting from zero.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn wnfetd(window: &mut Cell<f64>, n: i32) -> (f64, f64) {}
}

cspice_proc! {
    /**
    Return the number of intervals in a double precision window.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn wncard(window: &mut Cell<f64>) -> i32 {}
}

cspice_proc! {
    /**
    Place the complement of a window, relative to the interval `[left, right]`, into `result`.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn wncomd(left: f64, right: f64, window: &mut Cell<f64>, result: &mut Cell<f64>) {}
}

cspice_proc! {
    /**
    Contract each interval of a window by `left` at its start and `right` at its end.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn wncond(left: f64, right: f64, window: &mut Cell<f64>) {}
}

cspice_proc! {
    /**
    Expand each interval of a window by `left` at its start and `right` at its end.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn wnexpd(left: f64, right: f64, window: &mut Cell<f64>) {}
}

cspice_proc! {
    /**
    Place the difference of two windows into `c`.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn wndifd(a: &mut Cell<f64>, b: &mut Cell<f64>, c: &mut Cell<f64>) {}
}

cspice_proc! {
    /**
    Place the intersection of two windows into `c`.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn wnintd(a: &mut Cell<f64>, b: &mut Cell<f64>, c: &mut Cell<f64>) {}
}

cspice_proc! {
    /**
    Place the union of two windows into `c`.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn wnunid(a: &mut Cell<f64>, b: &mut Cell<f64>, c: &mut Cell<f64>) {}
}

cspice_proc! {
    /**
    Whether a point belongs to a window.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn wnelmd(point: f64, window: &mut Cell<f64>) -> bool {}
}

cspice_proc! {
    /**
    Whether an interval is included in a window.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn wnincd(left: f64, right: f64, window: &mut Cell<f64>) -> bool {}
}

cspice_proc! {
    /**
    Compare two windows; `op` is one of `"="`, `"<>"`, `"<="`, `"<"`, `">="` or `">"`.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn wnreld(a: &mut Cell<f64>, op: &str, b: &mut Cell<f64>) -> bool {}
}

cspice_proc! {
    /**
    Replace each interval of a window by one of its endpoints; `side` is `'L'` or `'R'`.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn wnextd(side: char, window: &mut Cell<f64>) {}
}

cspice_proc! {
    /**
    Fill the gaps shorter than `sml` between adjacent intervals of a window.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn wnfild(sml: f64, window: &mut Cell<f64>) {}
}

cspice_proc! {
    /**
    Drop the intervals of a window shorter than `sml`.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn wnfltd(sml: f64, window: &mut Cell<f64>) {}
}

cspice_proc! {
    /**
    Summarize a window: total measure, average and standard deviation of the interval lengths, and
    the indices of the shortest and longest intervals.

    The two indices point at the *left endpoints* in the flat endpoint array, so they run
    `0, 2, 4, ...` rather than `0, 1, 2, ...`; the `n`th interval is reported as `2 * n`. Ties go
    to the first interval of that length.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    #[allow(clippy::type_complexity)]
    pub fn wnsumd(window: &mut Cell<f64>) -> (f64, f64, f64, i32, i32) {}
}

cspice_proc! {
    /**
    Validate a double precision window built by writing into a cell directly.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn wnvald(size: i32, n: i32, window: &mut Cell<f64>) {}
}

/* -------------------------------------------------------------------------------------------- */
/* Sets and cells                                                                                 */
/* -------------------------------------------------------------------------------------------- */

cspice_proc! {
    /**
    Place the union of two sets into `c`.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn union<T: CellItem>(a: &mut Cell<T>, b: &mut Cell<T>, c: &mut Cell<T>) {}
}

cspice_proc! {
    /**
    Place the intersection of two sets into `c`.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn inter<T: CellItem>(a: &mut Cell<T>, b: &mut Cell<T>, c: &mut Cell<T>) {}
}

cspice_proc! {
    /**
    Place the difference of two sets into `c`.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn diff<T: CellItem>(a: &mut Cell<T>, b: &mut Cell<T>, c: &mut Cell<T>) {}
}

cspice_proc! {
    /**
    Copy the contents of one cell into another.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn copy<T: CellItem>(a: &mut Cell<T>, b: &mut Cell<T>) {}
}

cspice_proc! {
    /**
    The cardinality of a cell; [`Cell::len`] reports the same number without a CSPICE call.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn card<T: CellItem>(cell: &mut Cell<T>) -> i32 {}
}

cspice_proc! {
    /**
    Set the cardinality of a cell.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn scard<T: CellItem>(card: i32, cell: &mut Cell<T>) {}
}

cspice_proc! {
    /**
    The size of a cell; [`Cell::capacity`] reports the same number without a CSPICE call.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn size<T: CellItem>(cell: &mut Cell<T>) -> i32 {}
}

cspice_proc! {
    /**
    Set the size of a cell.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn ssize<T: CellItem>(size: i32, cell: &mut Cell<T>) {}
}

cspice_proc! {
    /**
    Turn a cell holding `n` items into a set, by sorting it and removing the duplicates.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn valid<T: CellItem>(size: i32, n: i32, a: &mut Cell<T>) {}
}

cspice_proc! {
    /**
    Whether an integer belongs to a set.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn elemi(item: i32, set: &mut Cell<i32>) -> bool {}
}

cspice_proc! {
    /**
    Whether a double precision number belongs to a set.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn elemd(item: f64, set: &mut Cell<f64>) -> bool {}
}

cspice_proc! {
    /**
    Whether a string belongs to a set.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn elemc(item: &str, set: &mut Cell<String>) -> bool {}
}

cspice_proc! {
    /**
    Insert an integer into a set.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn insrti(item: i32, set: &mut Cell<i32>) {}
}

cspice_proc! {
    /**
    Insert a double precision number into a set.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn insrtd(item: f64, set: &mut Cell<f64>) {}
}

cspice_proc! {
    /**
    Insert a string into a set.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn insrtc(item: &str, set: &mut Cell<String>) {}
}

cspice_proc! {
    /**
    Remove an integer from a set.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn removi(item: i32, set: &mut Cell<i32>) {}
}

cspice_proc! {
    /**
    Remove a double precision number from a set.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn removd(item: f64, set: &mut Cell<f64>) {}
}

cspice_proc! {
    /**
    Remove a string from a set.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn removc(item: &str, set: &mut Cell<String>) {}
}

cspice_proc! {
    /**
    Append an integer to a cell; [`Cell::push`] is the idiomatic form.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn appndi(item: i32, cell: &mut Cell<i32>) {}
}

cspice_proc! {
    /**
    Append a double precision number to a cell; [`Cell::push`] is the idiomatic form.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn appndd(item: f64, cell: &mut Cell<f64>) {}
}

cspice_proc! {
    /**
    Append a string to a cell; [`Cell::push`] is the idiomatic form.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn appndc(item: &str, cell: &mut Cell<String>) {}
}

/* -------------------------------------------------------------------------------------------- */
/* Geometry finder                                                                                */
/* -------------------------------------------------------------------------------------------- */

cspice_proc! {
    /**
    Determine the time windows, within a confinement window, when one body is occulted by or in
    transit across another, as seen from an observer.

    This function has a [neat version][crate::neat::gfoclt].
    */
    #[allow(clippy::too_many_arguments)]
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
        result: &mut Cell<f64>
    ) {
    }
}

/* -------------------------------------------------------------------------------------------- */
/* Two-line elements                                                                              */
/* -------------------------------------------------------------------------------------------- */

/// Number of elements a two-line element set is parsed into.
pub const TLE_NELTS: usize = 10;

/// Number of geophysical constants the SGP4 propagator needs.
pub const TLE_NGEOPHS: usize = 8;

/**
Parse the two lines of a NORAD two-line element set into the epoch and the element set CSPICE uses.

`frstyr` is the first year of the century the two digit years in the set belong to; 1957, the start
of the space age, is the usual choice.

# Panics

Panics if `lines` does not hold exactly two lines.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn getelm<S: AsRef<str>>(frstyr: i32, lines: &[S]) -> (f64, [f64; TLE_NELTS]) {
    assert!(
        lines.len() == 2,
        "a two-line element set has two lines, got {}",
        lines.len()
    );

    // CSPICE reads the pair out of one contiguous, fixed stride, char array.
    let lineln = lines
        .iter()
        .map(|line| line.as_ref().len())
        .max()
        .unwrap_or(0)
        + 1;
    let mut buffer = vec![0 as SpiceChar; 2 * lineln];
    for (index, line) in lines.iter().enumerate() {
        let slot = &mut buffer[index * lineln..(index + 1) * lineln];
        for (target, byte) in slot.iter_mut().zip(line.as_ref().as_bytes()) {
            *target = *byte as SpiceChar;
        }
    }

    let mut epoch = 0.0;
    let mut elems = [0.0; TLE_NELTS];

    unsafe {
        crate::c::getelm_c(
            frstyr,
            lineln as SpiceInt,
            buffer.as_ptr().cast(),
            &mut epoch,
            elems.as_mut_ptr(),
        );
    }

    (epoch, elems)
}

cspice_proc! {
    /**
    Evaluate a two-line element set with the SGP4 propagator.

    `geophs` holds the eight geophysical constants the propagator needs, in the order
    `J2, J3, J4, KE, QO, SO, ER, AE`; they normally come from a geophysical constants kernel.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn evsgp4(et: f64, geophs: [f64; 8], elems: [f64; 10]) -> [f64; 6] {}
}

/* -------------------------------------------------------------------------------------------- */
/* Searching, sorting and ordering                                                                */
/* -------------------------------------------------------------------------------------------- */

/// Generate the wrappers that search a slice for a value.
macro_rules! search {
    ($($name:ident($ty:ty) => $cname:ident, $doc:expr);* $(;)?) => {$(
        #[doc = $doc]
        ///
        /// Returns the index found, or `-1`.
        #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
        pub fn $name(value: $ty, array: &[$ty]) -> i32 {
            unsafe { crate::c::$cname(value, array.len() as SpiceInt, array.as_ptr() as *mut _) }
        }
    )*};
}

search! {
    bsrchd(f64) => bsrchd_c, "Binary search a sorted array of doubles.";
    bsrchi(i32) => bsrchi_c, "Binary search a sorted array of integers.";
    lstled(f64) => lstled_c, "Last index of the doubles less than or equal to a value.";
    lstlei(i32) => lstlei_c, "Last index of the integers less than or equal to a value.";
    lstltd(f64) => lstltd_c, "Last index of the doubles strictly less than a value.";
    lstlti(i32) => lstlti_c, "Last index of the integers strictly less than a value.";
}

/// Generate the wrappers that sort a slice in place.
macro_rules! sort_in_place {
    ($($name:ident($ty:ty) => $cname:ident, $doc:expr);* $(;)?) => {$(
        #[doc = $doc]
        #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
        pub fn $name(array: &mut [$ty]) {
            unsafe { crate::c::$cname(array.len() as SpiceInt, array.as_mut_ptr()) }
        }
    )*};
}

sort_in_place! {
    shelld(f64) => shelld_c, "Sort an array of doubles in place.";
    shelli(i32) => shelli_c, "Sort an array of integers in place.";
}

/// Generate the wrappers that report the order of a slice without moving it.
macro_rules! order_of {
    ($($name:ident($ty:ty) => $cname:ident, $doc:expr);* $(;)?) => {$(
        #[doc = $doc]
        #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
        pub fn $name(array: &[$ty]) -> Vec<i32> {
            let mut iorder = vec![0; array.len()];
            unsafe {
                crate::c::$cname(
                    array.as_ptr() as *mut _,
                    array.len() as SpiceInt,
                    iorder.as_mut_ptr(),
                )
            };
            iorder
        }
    )*};
}

order_of! {
    orderd(f64) => orderd_c, "The order vector that sorts an array of doubles.";
    orderi(i32) => orderi_c, "The order vector that sorts an array of integers.";
}

/// Generate the wrappers that apply an order vector to a slice in place.
macro_rules! reorder {
    ($($name:ident($ty:ty) => $cname:ident, $doc:expr);* $(;)?) => {$(
        #[doc = $doc]
        ///
        /// # Panics
        ///
        /// Panics if the order vector is not as long as the array.
        #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
        pub fn $name(iorder: &[i32], array: &mut [$ty]) {
            assert!(
                iorder.len() == array.len(),
                "the order vector has {} entries for an array of {}",
                iorder.len(),
                array.len()
            );
            unsafe {
                crate::c::$cname(
                    iorder.as_ptr() as *mut SpiceInt,
                    array.len() as SpiceInt,
                    array.as_mut_ptr(),
                )
            };
        }
    )*};
}

reorder! {
    reordd(f64) => reordd_c, "Reorder an array of doubles in place.";
    reordi(i32) => reordi_c, "Reorder an array of integers in place.";
    reordl(crate::c::SpiceBoolean) => reordl_c, "Reorder an array of logical flags in place.";
}

cspice_proc! {
    /**
    Bracket a double precision number between two endpoints.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn brcktd(number: f64, end1: f64, end2: f64) -> f64 {}
}

cspice_proc! {
    /**
    Bracket an integer between two endpoints.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn brckti(number: i32, end1: i32, end2: i32) -> i32 {}
}

/**
The sum of an array of doubles.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn sumad(array: &[f64]) -> f64 {
    unsafe { crate::c::sumad_c(array.as_ptr() as *mut SpiceDouble, array.len() as SpiceInt) }
}

/**
The sum of an array of integers.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn sumai(array: &[i32]) -> i32 {
    unsafe { crate::c::sumai_c(array.as_ptr() as *mut SpiceInt, array.len() as SpiceInt) }
}

/**
Whether an array is an order vector: a permutation of `0 .. n`.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn isordv(array: &[i32]) -> bool {
    unsafe { crate::c::isordv_c(array.as_ptr() as *mut SpiceInt, array.len() as SpiceInt) != 0 }
}

/// Generate the wrappers that search a strided array of strings.
macro_rules! search_strings {
    ($($name:ident => $cname:ident, $doc:expr);* $(;)?) => {$(
        #[doc = $doc]
        ///
        /// Returns the index found, or `-1`.
        #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
        pub fn $name<S: AsRef<str>>(value: &str, array: &[S]) -> i32 {
            let value = to_cstring(value);
            let (buffer, stride) = to_strided(array);
            unsafe {
                crate::c::$cname(
                    value.as_ptr() as *mut SpiceChar,
                    array.len() as SpiceInt,
                    stride as SpiceInt,
                    buffer.as_ptr().cast(),
                )
            }
        }
    )*};
}

search_strings! {
    bsrchc => bsrchc_c, "Binary search a sorted array of strings.";
    esrchc => esrchc_c, "Search an array of strings, ignoring case and trailing blanks.";
    lstlec => lstlec_c, "Last index of the strings less than or equal to a value.";
    lstltc => lstltc_c, "Last index of the strings strictly less than a value.";
}

/**
Binary search an array of integers ordered by an order vector.

Returns the index found, or `-1`.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn bschoi(value: i32, array: &[i32], order: &[i32]) -> i32 {
    unsafe {
        crate::c::bschoi_c(
            value,
            array.len() as SpiceInt,
            array.as_ptr() as *mut SpiceInt,
            order.as_ptr() as *mut SpiceInt,
        )
    }
}

/**
Binary search an array of strings ordered by an order vector.

Returns the index found, or `-1`.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn bschoc<S: AsRef<str>>(value: &str, array: &[S], order: &[i32]) -> i32 {
    let value = to_cstring(value);
    let (buffer, stride) = to_strided(array);
    unsafe {
        crate::c::bschoc_c(
            value.as_ptr() as *mut SpiceChar,
            array.len() as SpiceInt,
            stride as SpiceInt,
            buffer.as_ptr().cast(),
            order.as_ptr() as *mut SpiceInt,
        )
    }
}

/**
The order vector that sorts an array of strings.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn orderc<S: AsRef<str>>(array: &[S]) -> Vec<i32> {
    let (buffer, stride) = to_strided(array);
    let mut iorder = vec![0; array.len()];
    unsafe {
        crate::c::orderc_c(
            stride as SpiceInt,
            buffer.as_ptr().cast(),
            array.len() as SpiceInt,
            iorder.as_mut_ptr(),
        )
    };
    iorder
}

/**
Apply an order vector to an array of strings.

# Panics

Panics if the order vector is not as long as the array.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn reordc<S: AsRef<str>>(iorder: &[i32], array: &[S]) -> Vec<String> {
    assert!(
        iorder.len() == array.len(),
        "the order vector has {} entries for an array of {}",
        iorder.len(),
        array.len()
    );
    let (mut buffer, stride) = to_strided(array);
    unsafe {
        crate::c::reordc_c(
            iorder.as_ptr() as *mut SpiceInt,
            array.len() as SpiceInt,
            stride as SpiceInt,
            buffer.as_mut_ptr().cast(),
        )
    };
    from_strided(&buffer, stride, array.len())
}

/**
Sort an array of strings.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn shellc<S: AsRef<str>>(array: &[S]) -> Vec<String> {
    let (mut buffer, stride) = to_strided(array);
    unsafe {
        crate::c::shellc_c(
            array.len() as SpiceInt,
            stride as SpiceInt,
            buffer.as_mut_ptr().cast(),
        )
    };
    from_strided(&buffer, stride, array.len())
}

/* -------------------------------------------------------------------------------------------- */
/* Strings                                                                                        */
/* -------------------------------------------------------------------------------------------- */

cspice_proc! {
    /**
    Convert a string to lower case.
    */
    pub fn lcase(input: &str, #[lenout] lenout: i32) -> String {}
}

cspice_proc! {
    /**
    Convert a string to upper case.
    */
    pub fn ucase(input: &str, #[lenout] lenout: i32) -> String {}
}

cspice_proc! {
    /**
    Compress runs of a delimiter down to `n` of them.
    */
    pub fn cmprss(delim: char, n: i32, input: &str, #[lenout] lenout: i32) -> String {}
}

cspice_proc! {
    /**
    Whether two strings are equivalent, ignoring case and leading and trailing blanks.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn eqstr(a: &str, b: &str) -> bool {}
}

cspice_proc! {
    /**
    Match a string against a template, ignoring case; `wstr` matches any run of characters and
    `wchr` any single one.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn matchi(string: &str, templ: &str, wstr: char, wchr: char) -> bool {}
}

cspice_proc! {
    /**
    Match a string against a template, respecting case.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn matchw(string: &str, templ: &str, wstr: char, wchr: char) -> bool {}
}

cspice_proc! {
    /**
    Replace a marker in a string with a string.
    */
    pub fn repmc(input: &str, marker: &str, value: &str, #[lenout] lenout: i32) -> String {}
}

cspice_proc! {
    /**
    Replace a marker in a string with a double precision number.
    */
    pub fn repmd(
        input: &str,
        marker: &str,
        value: f64,
        sigdig: i32,
        #[lenout] lenout: i32
    ) -> String {
    }
}

cspice_proc! {
    /**
    Replace a marker in a string with a formatted double precision number; `format` is `'E'` or
    `'F'`.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    #[allow(clippy::too_many_arguments)]
    pub fn repmf(
        input: &str,
        marker: &str,
        value: f64,
        sigdig: i32,
        format: char,
        #[lenout] lenout: i32
    ) -> String {
    }
}

cspice_proc! {
    /**
    Replace a marker in a string with an integer.
    */
    pub fn repmi(input: &str, marker: &str, value: i32, #[lenout] lenout: i32) -> String {}
}

cspice_proc! {
    /**
    Replace a marker in a string with the text of a boolean; `rtcase` picks the capitalisation.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn repml(
        input: &str,
        marker: &str,
        value: bool,
        rtcase: char,
        #[lenout] outlen: i32
    ) -> String {
    }
}

cspice_proc! {
    /**
    Replace a marker in a string with an ordinal number; `strcase` picks the capitalisation.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    #[cname(repmot_c)]
    pub fn repmot(
        input: &str,
        marker: &str,
        value: i32,
        strcase: char,
        #[lenout] lenout: i32
    ) -> String {
    }
}

/**
Split a string at the first run of blanks, into the first word and the rest.
*/
pub fn nextwd(string: &str, nexlen: usize, reslen: usize) -> (String, String) {
    let string = to_cstring(string);
    let mut next = vec![0 as SpiceChar; nexlen.max(1)];
    let mut rest = vec![0 as SpiceChar; reslen.max(1)];

    unsafe {
        crate::c::nextwd_c(
            string.as_ptr() as *mut SpiceChar,
            nexlen as SpiceInt,
            reslen as SpiceInt,
            next.as_mut_ptr(),
            rest.as_mut_ptr(),
        )
    };

    (from_cbuf(&next), from_cbuf(&rest))
}

/**
Split a list on a single delimiter.

This function has a [neat version][crate::neat::lparse].
*/
pub fn lparse(list: &str, delim: &str, nmax: usize, lenout: usize) -> Vec<String> {
    let list = to_cstring(list);
    let delim = to_cstring(delim);
    let lenout = lenout.max(1);
    let mut buffer = vec![0 as SpiceChar; nmax.max(1) * lenout];
    let mut n = 0;

    unsafe {
        crate::c::lparse_c(
            list.as_ptr() as *mut SpiceChar,
            delim.as_ptr() as *mut SpiceChar,
            nmax as SpiceInt,
            lenout as SpiceInt,
            &mut n,
            buffer.as_mut_ptr().cast(),
        )
    };

    from_strided(&buffer, lenout, n.max(0) as usize)
}

/**
Split a list on any of a set of delimiters, treating runs of them as one.

This function has a [neat version][crate::neat::lparsm].
*/
pub fn lparsm(list: &str, delims: &str, nmax: usize, lenout: usize) -> Vec<String> {
    let list = to_cstring(list);
    let delims = to_cstring(delims);
    let lenout = lenout.max(1);
    let mut buffer = vec![0 as SpiceChar; nmax.max(1) * lenout];
    let mut n = 0;

    unsafe {
        crate::c::lparsm_c(
            list.as_ptr() as *mut SpiceChar,
            delims.as_ptr() as *mut SpiceChar,
            nmax as SpiceInt,
            lenout as SpiceInt,
            &mut n,
            buffer.as_mut_ptr().cast(),
        )
    };

    from_strided(&buffer, lenout, n.max(0) as usize)
}

/* ---------------------------------------------------------------------------------------------- */
/* Units, numbers and text                                                                        */
/* ---------------------------------------------------------------------------------------------- */

cspice_proc! {
    /**
    Take a measurement X, the units associated with X, and units to which X should be converted; return Y --- the value of the measurement in the output units.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn convrt(x: f64, input: &str, output: &str) -> f64 {}
}

cspice_proc! {
    /**
    Return the value of the largest (positive) number representable in a double precision variable.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dpmax() -> f64 {}
}

cspice_proc! {
    /**
    Return the value of the smallest (negative) number representable in a double precision variable.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dpmin() -> f64 {}
}

cspice_proc! {
    /**
    Convert a double precision number to an equivalent character string using a base 16 "scientific notation."
    */
    pub fn dp2hx(number: f64, #[lenout] lenout: i32) -> (String, i32) {}
}

cspice_proc! {
    /**
    Convert a string representing a double precision number in a base 16 "scientific notation" into its equivalent double precision number.
    */
    pub fn hx2dp(string: &str, #[lenout] lenout: i32) -> (f64, bool, String) {}
}

cspice_proc! {
    /**
    Find the first occurrence in a string of a character belonging to a collection of characters, starting at a specified location, searching forward.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn cpos(string: &str, chars: &str, start: i32) -> i32 {}
}

cspice_proc! {
    /**
    Find the first occurrence in a string of a character belonging to a collection of characters, starting at a specified location, searching in reverse.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn cposr(string: &str, chars: &str, start: i32) -> i32 {}
}

cspice_proc! {
    /**
    Find the first occurrence in a string of a substring, starting at a specified location, searching forward.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn pos(string: &str, substr: &str, start: i32) -> i32 {}
}

cspice_proc! {
    /**
    Find the first occurrence in a string of a substring, starting at a specified location, searching backward.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn posr(string: &str, substr: &str, start: i32) -> i32 {}
}

cspice_proc! {
    /**
    Convert from an ephemeris epoch measured in seconds past the epoch of J2000 to a calendar string format using a formal calendar free of leapseconds.
    */
    pub fn etcal(et: f64, #[lenout] lenout: i32) -> String {}
}

cspice_proc! {
    /**
    Restrict the set of strings that are recognized by SPICE time parsing routines to those that have standard values for all time components.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn tparch(kind: &str) {}
}

cspice_proc! {
    /**
    Determine if a kernel pool variable is present and if so that it has the correct size and type.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn badkpv(caller: &str, name: &str, comp: &str, size: i32, divby: i32, kind: char) -> bool {}
}

/* ---------------------------------------------------------------------------------------------- */
/* Bodies, frames and the kernel pool                                                             */
/* ---------------------------------------------------------------------------------------------- */

cspice_proc! {
    /**
    Return the frame name, frame ID, and center associated with a given frame class and class ID.
    */
    pub fn ccifrm(frclss: i32, clssid: i32, #[lenout] lenout: i32) -> (i32, String, i32, bool) {}
}

cspice_proc! {
    /**
    Retrieve frame ID code and name to associate with a frame center.
    */
    pub fn cidfrm(cent: i32, #[lenout] lenout: i32) -> (i32, String, bool) {}
}

cspice_proc! {
    /**
    Retrieve frame ID code and name to associate with an object.
    */
    pub fn cnmfrm(cname: &str, #[lenout] lenout: i32) -> (i32, String, bool) {}
}

cspice_proc! {
    /**
    Return a SPICE set containing the frame IDs of all built-in frames of a specified class.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn bltfrm(frmcls: i32, idset: &mut Cell<i32>) {}
}

cspice_proc! {
    /**
    Indicate whether or not any watched kernel variables that have a specified agent on their notification list have been updated.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn cvpool(agent: &str) -> bool {}
}

/* ---------------------------------------------------------------------------------------------- */
/* Assorted routines                                                                              */
/* ---------------------------------------------------------------------------------------------- */

cspice_proc! {
    /**
    Return the azimuth/elevation coordinates of a specified target relative to an "observer," where the observer has constant position in a specified reference frame. The observer's position is provided by the calling program rather than by loaded SPK files.
    */
    #[allow(clippy::too_many_arguments)]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn azlcpo(method: &str, target: &str, et: f64, abcorr: &str, azccw: bool, elplsz: bool, obspos: [f64; 3], obsctr: &str, obsref: &str) -> ([f64; 6], f64) {}
}

cspice_proc! {
    /**
    Define a body name/ID code pair for later translation via bodn2c_c or bodc2n_c.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn boddef(name: &str, code: i32) {}
}

cspice_proc! {
    /**
    Inform the CSPICE error handling mechanism of entry into a routine.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn chkin(module: &str) {}
}

cspice_proc! {
    /**
    Inform the CSPICE error handling mechanism of exit from a routine.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn chkout(module: &str) {}
}

cspice_proc! {
    /**
    Compute the state (position and velocity) of an ellipsoid surface point nearest to the position component of a specified state.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dnearp(state: [f64; 6], a: f64, b: f64, c: f64) -> ([f64; 6], [f64; 2], bool) {}
}

cspice_proc! {
    /**
    Compute the unit vector parallel to the cross product of two 3-dimensional vectors and the derivative of this unit vector.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn ducrss(s1: [f64; 6], s2: [f64; 6]) -> [f64; 6] {}
}

cspice_proc! {
    /**
    Compute the cross product of two 3-dimensional vectors and the derivative of this cross product.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dvcrss(s1: [f64; 6], s2: [f64; 6]) -> [f64; 6] {}
}

cspice_proc! {
    /**
    Compute the derivative of the dot product of two double precision position vectors.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dvdot(s1: [f64; 6], s2: [f64; 6]) -> f64 {}
}

cspice_proc! {
    /**
    Find the unit vector corresponding to a state vector and the derivative of the unit vector.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dvhat(s1: [f64; 6]) -> [f64; 6] {}
}

cspice_proc! {
    /**
    Calculate the derivative of the norm of a 3-vector.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dvnorm(state: [f64; 6]) -> f64 {}
}

cspice_proc! {
    /**
    Delete a variable from the kernel pool.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dvpool(name: &str) {}
}

cspice_proc! {
    /**
    Calculate the time derivative of the separation angle between two input states, S1 and S2.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dvsep(s1: [f64; 6], s2: [f64; 6]) -> f64 {}
}

cspice_proc! {
    /**
    Return the unique point on an ellipsoid's surface where the outward normal direction is a given vector.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn ednmpt(a: f64, b: f64, c: f64, normal: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Scale a point so that it lies on the surface of a specified triaxial ellipsoid that is centered at the origin and aligned with the Cartesian coordinate axes.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn edpnt(p: [f64; 3], a: f64, b: f64, c: f64) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Compute the state (position and velocity) of an object whose trajectory is described via equinoctial elements relative to some fixed plane (usually the equatorial plane of some planet).
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn eqncpv(et: f64, epoch: f64, eqel: [f64; 9], rapol: f64, decpol: f64) -> [f64; 6] {}
}

cspice_proc! {
    /**
    Whether a routine should return immediately because the toolkit is in an error state.

    Named `return_c` rather than `return`, which is a keyword in Rust. It is the only routine in
    the toolkit whose name has to change, and SpiceyPy renames it the same way for the same reason.
    */
    #[return_output]
    #[cname(return_c)]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn return_c() -> bool {}
}

cspice_proc! {
    /**
    Signal an error, with the short message given.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn sigerr(message: &str) {}
}

cspice_proc! {
    /**
    Substitute a double precision number for the first occurrence of a marker in the long error
    message being built.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn errdp(marker: &str, number: f64) {}
}

cspice_proc! {
    /**
    Substitute an integer for the first occurrence of a marker in the long error message being
    built.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn errint(marker: &str, number: i32) {}
}

cspice_proc! {
    /**
    Substitute a character string for the first occurrence of a marker in the current long error message.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn errch(marker: &str, string: &str) {}
}

cspice_proc! {
    /**
    Close a file designated by a Fortran-style integer logical unit.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn ftncls(unit: i32) {}
}

cspice_proc! {
    /**
    Deprecated: This routine has been superseded by the CSPICE routine ilumin_c. This routine is supported for purposes of backward compatibility only.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn illum(target: &str, et: f64, abcorr: &str, obsrvr: &str, spoint: [f64; 3]) -> (f64, f64, f64) {}
}

cspice_proc! {
    /**
    Return the inverse of a state transformation matrix.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn invstm(mat: [[f64; 6]; 6]) -> [[f64; 6]; 6] {}
}

cspice_proc! {
    /**
    Return a boolean value indicating whether a string contains only white space characters.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn iswhsp(string: &str) -> bool {}
}

cspice_proc! {
    /**
    Return the zero based index of the last non-blank character in a character string.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn lastnb(string: &str) -> i32 {}
}

cspice_proc! {
    /**
    Compute the transmission (or reception) time of a signal at a specified target, given the reception (or transmission) time at a specified observer. Also return the elapsed time between transmission and reception.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn ltime(etobs: f64, obs: i32, dir: &str, targ: i32) -> (f64, f64) {}
}

cspice_proc! {
    /**
    Scan a string from a specified starting position for the end of a decimal number.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn lx4dec(string: &str, first: i32) -> (i32, i32) {}
}

cspice_proc! {
    /**
    Scan a string from a specified starting position for the end of a number.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn lx4num(string: &str, first: i32) -> (i32, i32) {}
}

cspice_proc! {
    /**
    Scan a string from a specified starting position for the end of a signed integer.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn lx4sgn(string: &str, first: i32) -> (i32, i32) {}
}

cspice_proc! {
    /**
    Scan a string from a specified starting position for the end of an unsigned integer.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn lx4uns(string: &str, first: i32) -> (i32, i32) {}
}

cspice_proc! {
    /**
    Scan (lex) a quoted string.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn lxqstr(string: &str, qchar: char, first: i32) -> (i32, i32) {}
}

cspice_proc! {
    /**
    Find the first occurrence in a string of a character NOT belonging to a collection of characters, starting at a specified location, searching forward.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn ncpos(string: &str, chars: &str, start: i32) -> i32 {}
}

cspice_proc! {
    /**
    Find the first occurrence in a string of a character NOT belonging to a collection of characters, starting at a specified location, searching in reverse.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn ncposr(string: &str, chars: &str, start: i32) -> i32 {}
}

cspice_proc! {
    /**
    Expand a triangular plate by a specified amount. The expanded plate is co-planar with, and has the same orientation as, the original. The centroids of the two plates coincide.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn pltexp(iverts: [[f64; 3]; 3], delta: f64) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Parse a string as a double precision number, encapsulating error handling.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn prsdp(string: &str) -> f64 {}
}

cspice_proc! {
    /**
    Parse a string as an integer, encapsulating error handling.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn prsint(string: &str) -> i32 {}
}

cspice_proc! {
    /**
    Derive angular velocity from a unit quaternion and its derivative with respect to time.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn qdq2av(q: [f64; 4], dq: [f64; 4]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Transform a vector to a new coordinate system rotated by `angle' radians about axis `iaxis'. This transformation rotates `v1' by -angle radians about the specified axis.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn rotvec(v1: [f64; 3], angle: f64, iaxis: i32) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Find the roots of a quadratic equation.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn rquad(a: f64, b: f64, c: f64) -> ([f64; 2], [f64; 2]) {}
}

cspice_proc! {
    /**
    Convert ephemeris seconds past J2000 (ET) to integral encoded spacecraft clock (`ticks'). For conversion to fractional ticks, (required for C-kernel production), see the routine sce2c_c.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn sce2t(sc: i32, et: f64) -> f64 {}
}

cspice_proc! {
    /**
    Convert a spacecraft clock format string to number of "ticks".
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn sctiks(sc: i32, clkstr: &str) -> f64 {}
}

cspice_proc! {
    /**
    Set the value of the current long error message.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn setmsg(msg: &str) {}
}

cspice_proc! {
    /**
    Deprecated: This routine has been superseded by the CSPICE routine sincpt_c. This routine is supported for purposes of backward compatibility only.
    */
    #[allow(clippy::too_many_arguments)]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    #[allow(clippy::type_complexity)]
    pub fn srfxpt(method: &str, target: &str, et: f64, abcorr: &str, obsrvr: &str, dref: &str, dvec: [f64; 3]) -> ([f64; 3], f64, f64, [f64; 3], bool) {}
}

cspice_proc! {
    /**
    Correct the apparent position of an object for stellar aberration.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn stelab(pobj: [f64; 3], vobs: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Correct the position of a target for the stellar aberration effect on radiation transmitted from a specified observer to the target.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn stlabx(pobj: [f64; 3], vobs: [f64; 3]) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Deprecated: This routine has been superseded by the CSPICE routine subpnt_c. This routine is supported for purposes of backward compatibility only.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn subpt(method: &str, target: &str, et: f64, abcorr: &str, obsrvr: &str) -> ([f64; 3], f64) {}
}

cspice_proc! {
    /**
    Deprecated: This routine has been superseded by the CSPICE routine subslr_c. This routine is supported for purposes of backward compatibility only.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn subsol(method: &str, target: &str, et: f64, abcorr: &str, obsrvr: &str) -> [f64; 3] {}
}

cspice_proc! {
    /**
    Compute, for a given observer, ray emanating from the observer, and target, the "tangent point": the point on the ray nearest to the target's surface. Also compute the point on the target's surface nearest to the tangent point.
    */
    #[allow(clippy::too_many_arguments)]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    #[allow(clippy::type_complexity)]
    pub fn tangpt(method: &str, target: &str, et: f64, fixref: &str, abcorr: &str, corloc: &str, obsrvr: &str, dref: &str, dvec: [f64; 3]) -> ([f64; 3], f64, f64, [f64; 3], f64, [f64; 3]) {}
}

cspice_proc! {
    /**
    Return a 3x3 matrix that transforms positions in inertial coordinates to positions in body-equator-and-prime-meridian coordinates.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn tipbod(frame: &str, body: i32, et: f64) -> [[f64; 3]; 3] {}
}

cspice_proc! {
    /**
    Return a 6x6 matrix that transforms states in inertial coordinates to states in body-equator-and-prime-meridian coordinates.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn tisbod(frame: &str, body: i32, et: f64) -> [[f64; 6]; 6] {}
}

cspice_proc! {
    /**
    Find the position rotation matrix from a Text Kernel (TK) frame with the specified frame class ID to its base frame.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn tkfram(frcode: i32) -> ([[f64; 3]; 3], i32, bool) {}
}

cspice_proc! {
    /**
    Return the number of modules in the traceback representation.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn trcdep() -> i32 {}
}

cspice_proc! {
    /**
    Compute the angular separation in radians between two spherical or point objects.
    */
    #[allow(clippy::too_many_arguments)]
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn trgsep(et: f64, targ1: &str, shape1: &str, frame1: &str, targ2: &str, shape2: &str, frame2: &str, obsrvr: &str, abcorr: &str) -> f64 {}
}

cspice_proc! {
    /**
    Set the lower bound on the 100 year range
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn tsetyr(year: i32) {}
}

cspice_proc! {
    /**
    Find the state transformation from a base frame to the right-handed frame defined by two state vectors: one state vector defining a specified axis and a second state vector defining a specified coordinate plane.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn twovxf(axdef: [f64; 6], indexa: i32, plndef: [f64; 6], indexp: i32) -> [[f64; 6]; 6] {}
}

cspice_proc! {
    /**
    Compute the normalized cross product of two 3-vectors.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn ucrss(v1: [f64; 3], v2: [f64; 3]) -> [f64; 3] {}
}

/* ---------------------------------------------------------------------------------------------- */
/* DAF, DAS and the kernel file layers                                                            */
/* ---------------------------------------------------------------------------------------------- */

cspice_proc! {
    /**
    Find the position rotation matrix from a C-kernel (CK) frame with the specified frame class ID (CK ID) to the base frame of the highest priority CK segment containing orientation data for this CK frame at the time requested.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn ckfrot(inst: i32, et: f64) -> ([[f64; 3]; 3], i32, bool) {}
}

cspice_proc! {
    /**
    Find the state transformation matrix from a C-kernel (CK) frame with the specified frame class ID (CK ID) to the base frame of the highest priority CK segment containing orientation and angular velocity data for this CK frame at the time requested.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn ckfxfm(inst: i32, et: f64) -> ([[f64; 6]; 6], i32, bool) {}
}

cspice_proc! {
    /**
    Load a CK pointing file for use by the CK readers. Return that file's handle, to be used by other CK routines to refer to the file.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn cklpf(fname: &str) -> i32 {}
}

cspice_proc! {
    /**
    Return (depending upon the user's request) the ID code of either the spacecraft or spacecraft clock associated with a C-Kernel ID code.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn ckmeta(ckid: i32, meta: &str) -> i32 {}
}

cspice_proc! {
    /**
    Unload a CK pointing file so that it will no longer be searched by the readers.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn ckupf(handle: i32) {}
}

cspice_proc! {
    /**
    Begin a backward search for arrays in a DAF.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dafbbs(handle: i32) {}
}

cspice_proc! {
    /**
    Begin a forward search for arrays in a DAF.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dafbfs(handle: i32) {}
}

cspice_proc! {
    /**
    Close the DAF associated with a given handle.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dafcls(handle: i32) {}
}

cspice_proc! {
    /**
    Select a DAF that already has a search in progress as the one to continue searching.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dafcs(handle: i32) {}
}

cspice_proc! {
    /**
    Delete the entire comment area of a specified DAF file.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dafdc(handle: i32) {}
}

cspice_proc! {
    /**
    Find the next (forward) array in the current DAF.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn daffna() -> bool {}
}

cspice_proc! {
    /**
    Find the previous (backward) array in the current DAF.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn daffpa() -> bool {}
}

cspice_proc! {
    /**
    Return (get) the handle of the DAF currently being searched.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dafgh() -> i32 {}
}

/**
Read a contiguous run of double precision words from a DAF record.

Returns the `end - begin + 1` words, and whether the record was found.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn dafgsr(handle: i32, recno: i32, begin: i32, end: i32) -> (Vec<f64>, bool) {
    let words = (end - begin + 1).max(0) as usize;
    let mut data = vec![0.0; words.max(1)];
    let mut found = 0;
    unsafe { crate::c::dafgsr_c(handle, recno, begin, end, data.as_mut_ptr(), &mut found) };
    data.truncate(words);
    (data, found != 0)
}

cspice_proc! {
    /**
    Return the summary format associated with a handle.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dafhsf(handle: i32) -> (i32, i32) {}
}

cspice_proc! {
    /**
    Open a DAF for subsequent read requests.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dafopr(fname: &str) -> i32 {}
}

cspice_proc! {
    /**
    Open a DAF for subsequent write requests.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dafopw(fname: &str) -> i32 {}
}

cspice_proc! {
    /**
    Delete the entire comment area of a previously opened binary DAS file.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dasdc(handle: i32) {}
}

cspice_proc! {
    /**
    Return a file summary for a specified DAS file.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    #[allow(clippy::type_complexity)]
    pub fn dashfs(handle: i32) -> (i32, i32, i32, i32, i32, [i32; 3], [i32; 3], [i32; 3]) {}
}

cspice_proc! {
    /**
    Return last DAS logical addresses of character, double precision and integer type that are currently in use in a specified DAS file.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn daslla(handle: i32) -> (i32, i32, i32) {}
}

cspice_proc! {
    /**
    Close the DAS file associated with a given handle, without flushing buffered data or segregating the file.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dasllc(handle: i32) {}
}

cspice_proc! {
    /**
    Open a new DAS file and set the file type.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dasonw(fname: &str, ftype: &str, ifname: &str, ncomr: i32) -> i32 {}
}

cspice_proc! {
    /**
    Open a scratch DAS file for writing.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dasops() -> i32 {}
}

cspice_proc! {
    /**
    Open a DAS file for writing.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dasopw(fname: &str) -> i32 {}
}

cspice_proc! {
    /**
    Write out all buffered records of a specified DAS file.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn daswbr(handle: i32) {}
}

cspice_proc! {
    /**
    Begin a new segment in a DLA file.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dlabns(handle: i32) {}
}

cspice_proc! {
    /**
    End a new segment in a DLA file.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dlaens(handle: i32) {}
}

cspice_proc! {
    /**
    Open a new DLA file and set the file type.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dlaopn(fname: &str, ftype: &str, ifname: &str, ncomch: i32) -> i32 {}
}

cspice_proc! {
    /**
    Close an open PCK file.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn pckcls(handle: i32) {}
}

cspice_proc! {
    /**
    Load a binary PCK file for use by the readers. Return the handle of the loaded file which is used by other PCK routines to refer to the file.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn pcklof(fname: &str) -> i32 {}
}

cspice_proc! {
    /**
    Create a new PCK file, returning the handle of the opened file.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn pckopn(name: &str, ifname: &str, ncomch: i32) -> i32 {}
}

cspice_proc! {
    /**
    Unload a binary PCK file so that it will no longer be searched by the readers.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn pckuof(handle: i32) {}
}

/* -------------------------------------------------------------------------------------------- */
/* DAF and DAS, the array files underneath the kernels                                            */
/* -------------------------------------------------------------------------------------------- */

/// Largest DAF summary, in double precision words.
pub const DAF_MAXSUM: usize = 125;

/// Read a run of double precision words from a DAF, for the two routines that do it.
macro_rules! daf_read {
    ($($name:ident => $cname:ident, $doc:expr);* $(;)?) => {$(
        #[doc = $doc]
        ///
        /// Returns the `end - begin + 1` words, addressed from one.
        #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
        pub fn $name(handle: i32, begin: i32, end: i32) -> Vec<f64> {
            let words = (end - begin + 1).max(0) as usize;
            let mut data = vec![0.0; words.max(1)];
            unsafe { crate::c::$cname(handle, begin, end, data.as_mut_ptr()) };
            data.truncate(words);
            data
        }
    )*};
}

daf_read! {
    dafgda => dafgda_c, "Read double precision data from the current array of a DAF.";
    dafrda => dafrda_c, "Read double precision data from a DAF; superseded by `dafgda`.";
}

cspice_proc! {
    /**
    Return the name of the current array in the current DAF.
    */
    pub fn dafgn(#[lenout] lenout: i32) -> String {}
}

/**
Return the summary of the current array in the current DAF.

At most [`DAF_MAXSUM`] words are returned, which is the largest a DAF summary can be.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn dafgs() -> Vec<f64> {
    let mut summary = vec![0.0; DAF_MAXSUM];
    unsafe { crate::c::dafgs_c(summary.as_mut_ptr()) };
    summary
}

/**
Pack the double precision and integer components of a DAF summary into one array.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn dafps(dc: &[f64], ic: &[i32]) -> Vec<f64> {
    let words = dc.len() + (ic.len() + 1) / 2 + 1;
    let mut summary = vec![0.0; words.max(1)];
    unsafe {
        crate::c::dafps_c(
            dc.len() as SpiceInt,
            ic.len() as SpiceInt,
            dc.as_ptr() as *mut SpiceDouble,
            ic.as_ptr() as *mut SpiceInt,
            summary.as_mut_ptr(),
        )
    };
    summary
}

/**
Unpack a DAF summary into its `nd` double precision and `ni` integer components.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn dafus(summary: &[f64], nd: usize, ni: usize) -> (Vec<f64>, Vec<i32>) {
    let mut dc = vec![0.0; nd.max(1)];
    let mut ic = vec![0; ni.max(1)];
    unsafe {
        crate::c::dafus_c(
            summary.as_ptr() as *mut SpiceDouble,
            nd as SpiceInt,
            ni as SpiceInt,
            dc.as_mut_ptr(),
            ic.as_mut_ptr(),
        )
    };
    dc.truncate(nd);
    ic.truncate(ni);
    (dc, ic)
}

/**
Replace the summary of the current array in the current DAF.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn dafrs(summary: &[f64]) {
    unsafe { crate::c::dafrs_c(summary.as_ptr() as *mut SpiceDouble) }
}

/**
Read the file record of a DAF: the summary sizes, the internal file name, and the pointers to the
first and last summary records and the first free address.
*/
#[allow(clippy::type_complexity)]
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn dafrfr(handle: i32, lenout: usize) -> (i32, i32, String, i32, i32, i32) {
    let mut ifname = vec![0 as SpiceChar; lenout.max(1)];
    let (mut nd, mut ni, mut fward, mut bward, mut free) = (0, 0, 0, 0, 0);
    unsafe {
        crate::c::dafrfr_c(
            handle,
            lenout as SpiceInt,
            &mut nd,
            &mut ni,
            ifname.as_mut_ptr(),
            &mut fward,
            &mut bward,
            &mut free,
        )
    };
    (nd, ni, from_cbuf(&ifname), fward, bward, free)
}

/**
Add comment lines to the comment area of a DAF.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn dafac<S: AsRef<str>>(handle: i32, buffer: &[S]) {
    let (packed, lenvals) = to_strided(buffer);
    unsafe {
        crate::c::dafac_c(
            handle,
            buffer.len() as SpiceInt,
            lenvals as SpiceInt,
            packed.as_ptr().cast(),
        )
    }
}

/**
Read comment lines from the comment area of a DAF.

Returns at most `bufsiz` lines, and whether the comment area has been read to the end.

This function has a [neat version][crate::neat::dafec].
*/
pub fn dafec(handle: i32, bufsiz: usize, lenout: usize) -> (Vec<String>, bool) {
    let lenout = lenout.max(1);
    let mut buffer = vec![0 as SpiceChar; bufsiz.max(1) * lenout];
    let (mut n, mut done) = (0, 0);
    unsafe {
        crate::c::dafec_c(
            handle,
            bufsiz as SpiceInt,
            lenout as SpiceInt,
            &mut n,
            buffer.as_mut_ptr().cast(),
            &mut done,
        )
    };
    (from_strided(&buffer, lenout, n.max(0) as usize), done != 0)
}

/// Read a run of words from a DAS, for the double precision and integer cases.
macro_rules! das_read {
    ($($name:ident($ty:ty) => $cname:ident, $doc:expr);* $(;)?) => {$(
        #[doc = $doc]
        ///
        /// Returns the `last - first + 1` words, addressed from one.
        #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
        pub fn $name(handle: i32, first: i32, last: i32) -> Vec<$ty> {
            let words = (last - first + 1).max(0) as usize;
            let mut data = vec![<$ty>::default(); words.max(1)];
            unsafe { crate::c::$cname(handle, first, last, data.as_mut_ptr()) };
            data.truncate(words);
            data
        }
    )*};
}

das_read! {
    dasrdd(f64) => dasrdd_c, "Read double precision data from a DAS.";
    dasrdi(i32) => dasrdi_c, "Read integer data from a DAS.";
}

/// Update a run of words in a DAS, for the double precision and integer cases.
macro_rules! das_update {
    ($($name:ident($ty:ty) => $cname:ident, $doc:expr);* $(;)?) => {$(
        #[doc = $doc]
        #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
        pub fn $name(handle: i32, first: i32, last: i32, data: &[$ty]) {
            unsafe { crate::c::$cname(handle, first, last, data.as_ptr() as *mut _) }
        }
    )*};
}

das_update! {
    dasudd(f64) => dasudd_c, "Update double precision data in a DAS.";
    dasudi(i32) => dasudi_c, "Update integer data in a DAS.";
}

/// Append data to a DAS, for the double precision and integer cases.
macro_rules! das_add {
    ($($name:ident($ty:ty) => $cname:ident, $doc:expr);* $(;)?) => {$(
        #[doc = $doc]
        #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
        pub fn $name(handle: i32, data: &[$ty]) {
            unsafe {
                crate::c::$cname(handle, data.len() as SpiceInt, data.as_ptr() as *mut _)
            }
        }
    )*};
}

das_add! {
    dasadd(f64) => dasadd_c, "Append double precision data to a DAS.";
    dasadi(i32) => dasadi_c, "Append integer data to a DAS.";
}

/**
Append characters to a DAS, taking the substring `bpos ..= epos` of each line.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn dasadc<S: AsRef<str>>(handle: i32, bpos: i32, epos: i32, data: &[S]) {
    let (packed, datlen) = to_strided(data);
    unsafe {
        crate::c::dasadc_c(
            handle,
            data.len() as SpiceInt,
            bpos,
            epos,
            datlen as SpiceInt,
            packed.as_ptr().cast(),
        )
    }
}

/**
Add comment lines to the comment area of a DAS.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn dasac<S: AsRef<str>>(handle: i32, buffer: &[S]) {
    let (packed, buflen) = to_strided(buffer);
    unsafe {
        crate::c::dasac_c(
            handle,
            buffer.len() as SpiceInt,
            buflen as SpiceInt,
            packed.as_ptr().cast(),
        )
    }
}

/**
Read comment lines from the comment area of a DAS.

This function has a [neat version][crate::neat::dasec].
*/
pub fn dasec(handle: i32, bufsiz: usize, buflen: usize) -> (Vec<String>, bool) {
    let buflen = buflen.max(1);
    let mut buffer = vec![0 as SpiceChar; bufsiz.max(1) * buflen];
    let (mut n, mut done) = (0, 0);
    unsafe {
        crate::c::dasec_c(
            handle,
            bufsiz as SpiceInt,
            buflen as SpiceInt,
            &mut n,
            buffer.as_mut_ptr().cast(),
            &mut done,
        )
    };
    (from_strided(&buffer, buflen, n.max(0) as usize), done != 0)
}

cspice_proc! {
    /**
    Return the name of the file a DAS handle refers to.
    */
    pub fn dashfn(handle: i32, #[lenout] namlen: i32) -> String {}
}

/**
Read the file record of a DAS: the ID word, the internal file name, and the reserved and comment
area sizes.
*/
#[allow(clippy::type_complexity)]
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn dasrfr(handle: i32, idwlen: usize, ifnlen: usize) -> (String, String, i32, i32, i32, i32) {
    let mut idword = vec![0 as SpiceChar; idwlen.max(1)];
    let mut ifname = vec![0 as SpiceChar; ifnlen.max(1)];
    let (mut nresvr, mut nresvc, mut ncomr, mut ncomc) = (0, 0, 0, 0);
    unsafe {
        crate::c::dasrfr_c(
            handle,
            idwlen as SpiceInt,
            ifnlen as SpiceInt,
            idword.as_mut_ptr(),
            ifname.as_mut_ptr(),
            &mut nresvr,
            &mut nresvc,
            &mut ncomr,
            &mut ncomc,
        )
    };
    (
        from_cbuf(&idword),
        from_cbuf(&ifname),
        nresvr,
        nresvc,
        ncomr,
        ncomc,
    )
}

cspice_proc! {
    /**
    Find the segment preceding a specified segment in a DLA file.
    */
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn dlafps(handle: i32, descr: DLADSC) -> (DLADSC, bool) {}
}

/* -------------------------------------------------------------------------------------------- */
/* Constants                                                                                      */
/* -------------------------------------------------------------------------------------------- */

/// Generate the wrappers of the CSPICE routines that just return a constant.
macro_rules! constants {
    ($($name:ident => $doc:expr),* $(,)?) => {$(
        cspice_proc! {
            #[doc = $doc]
            #[return_output]
            #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
            pub fn $name() -> f64 {}
        }
    )*};
}

constants! {
    b1900 => "Julian Date of the epoch B1900.",
    b1950 => "Julian Date of the epoch B1950.",
    clight => "Speed of light in a vacuum, in km/s.",
    dpr => "Number of degrees per radian.",
    halfpi => "Half the value of pi.",
    j1900 => "Julian Date of 1899 DEC 31 12:00:00 (1900 JAN 0.5).",
    j1950 => "Julian Date of 1950 JAN 01 00:00:00 (1950 JAN 1.0).",
    j2000 => "Julian Date of 2000 JAN 01 12:00:00 (2000 JAN 1.5).",
    j2100 => "Julian Date of 2100 JAN 01 12:00:00 (2100 JAN 1.5).",
    jyear => "Number of seconds in a Julian year.",
    pi => "Value of pi.",
    rpd => "Number of radians per degree.",
    spd => "Number of seconds in a day.",
    twopi => "Twice the value of pi.",
    tyear => "Number of seconds in a tropical year.",
}

cspice_proc! {
    /**
    Return the version of the CSPICE toolkit; pass `"TOOLKIT"` as the item.
    */
    #[return_output]
    #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
    pub fn tkvrsn(item: &str) -> String {}
}
