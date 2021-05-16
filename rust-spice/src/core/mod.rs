/*!
# A Rust idiomatic interface to CSPICE.

Below you will find the index of the CSPICE functions that are wrapped with an idiomatic Rust interface.

It takes a long time to correctly wrap all functions of the API. Raise an issue to ask a specific
function to be implemented and we will do it immediately. Pull requests are warmly welcomed to help
speed up this process (do not forget to include a proper documentation and a test).

In the meantime, if you are in a rush and need quickly to use a function not implemented with the
Rust interface, use the unsafe C functions [here][c#functions]. You can find some inspiration in
the source of this lib to deal with the FFI types and unsafe code.

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
[dskobj_c][dskobj_c link] | *TODO*
[dskp02_c][dskp02_c link] | *TODO*
[dsksrf_c][dsksrf_c link] | *TODO*
[dskv02_c][dskv02_c link] | *TODO*
[dskz02_c][dskz02_c link] | *TODO*
[furnsh_c][furnsh_c link] | [`raw::furnsh`] | loads an individual kernel or a collection of kernels
[gcpool_c][gcpool_c link] | *TODO*
[gdpool_c][gdpool_c link] | *TODO*
[getfov_c][getfov_c link] | *TODO*
[gipool_c][gipool_c link] | *TODO*
[latsrf_c][latsrf_c link] | *TODO*
[occult_c][occult_c link] | [`raw::occult`] | find occultation type at time
[pckcov_c][pckcov_c link] | *TODO*
[pxform_c][pxform_c link] | [`raw::pxform`] | returns the 3x3 matrix rotating a position vector one frame to another
[pxfrm2_c][pxfrm2_c link] | [`raw::pxfrm2`] | returns the 3x3 matrix rotating a position vector from one frame at a specified epoch to another frame at a different epoch
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
[spkpos_c][spkpos_c link] | [`raw::spkpos`] | returns the position of a target body relative to an observing body
[srfc2s_c][srfc2s_c link] | *TODO*
[srfcss_c][srfcss_c link] | *TODO*
[srfnrm_c][srfnrm_c link] | *TODO*
[srfs2c_c][srfs2c_c link] | *TODO*
[srfscc_c][srfscc_c link] | *TODO*
[str2et_c][str2et_c link] | [`raw::str2et`] | converts a time string to ET seconds past J2000
[sxform_c][sxform_c link] | *TODO*
[timout_c][timout_c link] | [`neat::timout`] | converts ET seconds past J2000 to a time string
[unload_c][unload_c link] | [`raw::unload`] | unloads an individual kernel or a collection of kernels

[bodc2n_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bodc2n_c.html
[bodfnd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bodfnd_c.html
[bodn2c_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bodn2c_c.html
[bodvrd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bodvrd_c.html
[ckcov_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckcov_c.html
[ckgp_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckgp_c.html
[ckgpav_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckgpav_c.html
[ckobj_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckobj_c.html
[dskobj_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskobj_c.html
[dskp02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskp02_c.html
[dsksrf_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dsksrf_c.html
[dskv02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskv02_c.html
[dskz02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskz02_c.html
[furnsh_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/furnsh_c.html
[gcpool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gcpool_c.html
[gdpool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gdpool_c.html
[getfov_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/getfov_c.html
[gipool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gipool_c.html
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
[timout_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/timout_c.html
[unload_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/unload_c.html
*/

/**
A Rust idiomatic CSPICE wrapper built with procedural macros.
*/
pub mod raw;

/**
Improvement on the procedurally generated functions.
*/
pub mod neat;

pub use self::neat::timout;
pub use self::raw::{furnsh, pxform, pxfrm2, spkpos, str2et, unload};

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
