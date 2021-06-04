/*!
An idiomatic interface in Rust to CSPICE.

## Description

Below you will find the index of the CSPICE functions that are wrapped with an idiomatic Rust interface.

It takes a long time to correctly wrap all functions of the API. Raise an issue to ask a specific
function to be implemented and we will do it immediately. Pull requests are warmly welcomed to help
speed up this process (do not forget to include a proper documentation and a test).

In the meantime, if you are in a rush and need quickly to use a function not implemented with the
Rust interface, use the unsafe C functions [here][crate::c#functions]. You can find some inspiration in
the source of this lib to deal with the FFI types and unsafe code.

## Bindings

CSPICE | **rust-spice** | Description
-------|--------------|------------
[bodc2n_c][bodc2n_c link] | *TODO*
[bodfnd_c][bodfnd_c link] | *TODO*
[bodn2c_c][bodn2c_c link] | *TODO*
[bodvrd_c][bodvrd_c link] | *TODO*
[ckcov_c][ckcov_c link] | *TODO*
[ckgp_c][ckgp_c link] | *TODO*
[ckgpav_c][ckgpav_c link] | *TODO*
[ckobj_c][ckobj_c link] | *TODO*
[dascls_c][dascls_c link] | [`raw::dascls`] | DAS, close file
[dasopr_c][dasopr_c link] | [`raw::dasopr`] | DAS, open for read
[dlabfs_c][dlabfs_c link] | [`raw::dlabfs`] | DLA, begin forward search
[dskgd_c][dskgd_c link] | [`raw::dskgd`] | DSK, return DSK segment descriptor
[dskn02_c][dskn02_c link] | [`raw::dskn02`] | DSK, type 2, compute normal vector for plate
[dskobj_c][dskobj_c link] | *TODO*
[dskp02_c][dskp02_c link] | *TODO*
[dsksrf_c][dsksrf_c link] | *TODO*
[dskv02_c][dskv02_c link] | *TODO*
[dskx02_c][dskx02_c link] | [`raw::dskx02`] | DSK, ray-surface intercept, type 2
[dskz02_c][dskz02_c link] | *TODO*
[furnsh_c][furnsh_c link] | [`raw::furnsh`] | Furnish a program with SPICE kernels
[gcpool_c][gcpool_c link] | *TODO*
[gdpool_c][gdpool_c link] | *TODO*
[getfov_c][getfov_c link] | *TODO*
[gipool_c][gipool_c link] | *TODO*
[kclear_c][kclear_c link] | [`raw::kclear`] | Keeper clear
[latrec_c][latrec_c link] | [`raw::latrec`] | Latitudinal to rectangular coordinates
[latsrf_c][latsrf_c link] | *TODO*
[occult_c][occult_c link] | [`raw::occult`] | Find occultation type at time
[pckcov_c][pckcov_c link] | *TODO*
[pxform_c][pxform_c link] | [`raw::pxform`] | Position Transformation Matrix
[pxfrm2_c][pxfrm2_c link] | [`raw::pxfrm2`] | Position Transform Matrix, Different Epochs
[sce2c_c][sce2c_c link] | *TODO*
[sce2s_c][sce2s_c link] | *TODO*
[scencd_c][scencd_c link] | *TODO*
[scdecd_c][scdecd_c link] | *TODO*
[scs2e_c][scs2e_c link] | *TODO*
[sct2e_c][sct2e_c link] | *TODO*
[spkcov_c][spkcov_c link] | *TODO*
[spkcpo_c][spkcpo_c link] | *TODO*
[spkcpt_c][spkcpt_c link] | *TODO*
[spkcvo_c][spkcvo_c link] | *TODO*
[spkcvt_c][spkcvt_c link] | *TODO*
[spkezr_c][spkezr_c link] | *TODO*
[spkobj_c][spkobj_c link] | *TODO*
[spkpos_c][spkpos_c link] | [`raw::spkpos`] | S/P Kernel, position
[srfc2s_c][srfc2s_c link] | *TODO*
[srfcss_c][srfcss_c link] | *TODO*
[srfnrm_c][srfnrm_c link] | *TODO*
[srfs2c_c][srfs2c_c link] | *TODO*
[srfscc_c][srfscc_c link] | *TODO*
[str2et_c][str2et_c link] | [`raw::str2et`] | String to ET
[sxform_c][sxform_c link] | *TODO*
[recrad_c][recrad_c link] | [`raw::recrad`] | Rectangular coordinates to RA and DEC
[timout_c][timout_c link] | [`neat::timout`] | Time Output
[unload_c][unload_c link] | [`raw::unload`] | Unload a kernel
[vsep_c][vsep_c link] | [`raw::vsep`] | Angular separation of vectors, 3 dimensions

[bodc2n_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bodc2n_c.html
[bodfnd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bodfnd_c.html
[bodn2c_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bodn2c_c.html
[bodvrd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bodvrd_c.html
[ckcov_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckcov_c.html
[ckgp_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckgp_c.html
[ckgpav_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckgpav_c.html
[ckobj_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckobj_c.html
[dascls_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dascls_c.html
[dasopr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dasopr_c.html
[dlabfs_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dasopr_c.html
[dskgd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskgd_c.html
[dskn02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskn02_c.html
[dskobj_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskobj_c.html
[dskp02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskp02_c.html
[dsksrf_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dsksrf_c.html
[dskv02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskv02_c.html
[dskx02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskx02_c.html
[dskz02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskz02_c.html
[furnsh_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/furnsh_c.html
[gcpool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gcpool_c.html
[gdpool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gdpool_c.html
[getfov_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/getfov_c.html
[gipool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gipool_c.html
[kclear_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/kclear_c.html
[latrec_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/latrec_c.html
[latsrf_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/latsrf_c.html
[occult_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/occult_c.html
[pxform_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pxform_c.html
[pckcov_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pckcov_c.html
[pckfrm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pckfrm_c.html
[pxfrm2_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pxfrm2_c.html
[scdecd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/scdecd_c.html
[sce2c_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sce2c_c.html
[sce2s_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sce2s_c.html
[scencd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/scencd_c.html
[scs2e_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/scs2e_c.html
[sct2e_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sct2e_c.html
[spkcpo_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkcpo_c.html
[spkcov_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkcov_c.html
[spkcpt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkcpt_c.html
[spkcvo_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkcvo_c.html
[spkcvt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkcvt_c.html
[spkezr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkezr_c.html
[spkobj_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkobj_c.html
[spkpos_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkpos_c.html
[srfc2s_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/srfc2s_c.html
[srfcss_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/srfcss_c.html
[srfnrm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/srfnrm_c.html
[srfs2c_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/srfs2c_c.html
[srfscc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/srfscc_c.html
[str2et_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/str2et_c.html
[sxform_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sxform_c.html
[recrad_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/recrad_c.html
[timout_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/timout_c.html
[unload_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/unload_c.html
[vsep_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vsep_c.html
*/

pub mod neat;
pub mod raw;

pub use self::neat::timout;
pub use self::raw::{
    dascls, dasopr, dlabfs, dskgd, dskn02, dskx02, furnsh, kclear, latrec, pxform, pxfrm2, recrad,
    spkpos, str2et, unload, vsep, DLADSC, DSKDSC,
};

/**
Default date format.
*/
pub const TIME_FORMAT: &str = "YYYY-MON-DD HR:MN:SC ::RND";

/**
Size of the default date format.
*/
pub const TIME_FORMAT_SIZE: usize = TIME_FORMAT.len();

/**
Convert [`String`] to [`*mut i8`][`std::os::raw::c_char`].
*/
#[macro_export]
macro_rules! cstr {
    ($s:expr) => {{
        std::ffi::CString::new($s).unwrap().into_raw()
    }};
    () => {{
        std::ffi::CString::new(String::new()).unwrap().into_raw()
    }};
}

/**
Get [`String`] from [`*mut i8`][`std::os::raw::c_char`].
*/
#[macro_export]
macro_rules! fcstr {
    ($s:expr) => {{
        unsafe { std::ffi::CStr::from_ptr($s).to_str().unwrap().to_string() }
    }};
}
