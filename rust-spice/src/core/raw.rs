/*!
A Rust idiomatic CSPICE wrapper built with [procedural macros][`spice_derive`].

## Description

Every routine here mirrors its CSPICE counterpart one for one: the C inputs stay inputs, and the
values the C routine writes through pointers become the Rust return value, in the order CSPICE
declares them. Wrappers that need an explicit buffer size, or a caller allocated
[cell][crate::core::cell::Cell], have a friendlier counterpart in [`neat`][crate::core::neat].
*/

use crate::c::{SpiceChar, SpiceDouble, SpiceInt};
use crate::core::cell::Cell;
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
