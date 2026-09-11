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
use crate::core::ffi::{from_cbuf, to_cstring};
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
    let lenvals = values
        .iter()
        .map(|value| value.as_ref().len() + 1)
        .max()
        .unwrap_or(1);

    // CSPICE reads the values out of one contiguous, fixed stride, char array.
    let mut buffer = vec![0 as SpiceChar; values.len().max(1) * lenvals];
    for (index, value) in values.iter().enumerate() {
        let slot = &mut buffer[index * lenvals..(index + 1) * lenvals];
        for (target, byte) in slot.iter_mut().zip(value.as_ref().as_bytes()) {
            *target = *byte as SpiceChar;
        }
    }

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
