/*!
# The Rust layer to ease the use of the wrapper.

Below you will find the index of the C spice functions that are wrapped with a nice Rust interface.

It takes a long time to correctly wrap all functions of the API. Raise an issue to ask a specific
function to be implemented and we will do it immediately. Pull requests are warmly welcomed to help
speed us this process (do not forget to include a proper documentation and a test).

In the meantime, if you are in a rush and need quickly to use a function not implemented with the
Rust interface, use the C binded function [here][c#functions]. You can find some inspiration in the source of
the already implemented functions with the Rust interface to deal with the FFI types and unsafe.

## Most common API

### Loading and Unloading SPICE Kernels

CSpice | `rust-spice`
-------|-------------
[furnsh_c][furnsh_c link] | [`Kernel::load`]
[unload_c][unload_c link] | [`Kernel::unload`]

### Converting between UTC and Ephemeris Time (LSK)

CSpice | `rust-spice`
-------|-------------
[str2et_c][str2et_c link] | [`ephemeris_from_date`]
[timout_c][timout_c link] | [`date_from_ephemeris`]

### Converting between Ephemeris Time and Spacecraft Clock (SCLK)

CSpice | `rust-spice`
-------|-------------
[scs2e_c][scs2e_c link] | *TODO*
[sce2s_c][sce2s_c link] | *TODO*
[sct2e_c][sct2e_c link] | *TODO*
[sce2c_c][sce2c_c link] | *TODO*
[scencd_c][scencd_c link] | *TODO*
[scdecd_c][scdecd_c link] | *TODO*

### Constants and Orientation for Natural Bodies (PCK)

CSpice | `rust-spice`
-------|-------------
[bodfnd_c][bodfnd_c link] | *TODO*
[bodvrd_c][bodvrd_c link] | *TODO*
[pxform_c][pxform_c link] | *TODO*
[sxform_c][sxform_c link] | *TODO*
[pckfrm_c][pckfrm_c link] | *TODO*
[pckcov_c][pckcov_c link] | *TODO*

### Computing Transformations Between Frames (FK)

CSpice | `rust-spice`
-------|-------------
[pxfrm2_c][pxfrm2_c link] | *TODO*

### Computing Positions of Spacecraft and Natural Bodies (SPK)

CSpice | `rust-spice`
-------|-------------
[spkezr_c][spkezr_c link] | *TODO*
[spkpos_c][spkpos_c link] | [`position`]
[spkcpo_c][spkcpo_c link] | *TODO*
[spkcpt_c][spkcpt_c link] | *TODO*
[spkcvo_c][spkcvo_c link] | *TODO*
[spkcvt_c][spkcvt_c link] | *TODO*
[spkobj_c][spkobj_c link] | *TODO*
[spkcov_c][spkcov_c link] | *TODO*

### Computing Orientation for Spacecraft and Instruments (CK)

CSpice | `rust-spice`
-------|-------------
[ckobj_c][ckobj_c link] | *TODO*
[ckcov_c][ckcov_c link] | *TODO*
[ckgp_c][ckgp_c link] | *TODO*
[ckgpav_c][ckgpav_c link] | *TODO*

### Retrieving Instrument Parameters (IK)

CSpice | `rust-spice`
-------|-------------
[getfov_c][getfov_c link] | *TODO*
[gdpool_c][gdpool_c link] | *TODO*
[gipool_c][gipool_c link] | *TODO*
[gcpool_c][gcpool_c link] | *TODO*

### Computing surface coordinates using digital shape (DSK)

CSpice | `rust-spice`
-------|-------------
[latsrf_c][latsrf_c link] | *TODO*
[srfnrm_c][srfnrm_c link] | *TODO*
[dskz02_c][dskz02_c link] | *TODO*
[dskp02_c][dskp02_c link] | *TODO*
[dskv02_c][dskv02_c link] | *TODO*
[dskobj_c][dskobj_c link] | *TODO*
[dsksrf_c][dsksrf_c link] | *TODO*

### Mapping Between Object Names and NAIF IDs

CSpice | `rust-spice`
-------|-------------
[bodc2n_c][bodc2n_c link] | *TODO*
[bodn2c_c][bodn2c_c link] | *TODO*

### Mapping between surface names and NAIF IDs

CSpice | `rust-spice`
-------|-------------
[srfcss_c][srfcss_c link] | *TODO*
[srfs2c_c][srfs2c_c link] | *TODO*
[srfc2s_c][srfc2s_c link] | *TODO*
[srfscc_c][srfscc_c link] | *TODO*

[furnsh_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/furnsh_c.html
[unload_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/unload_c.html
[str2et_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/str2et_c.html
[timout_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/timout_c.html
[scs2e_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/scs2e_c.html
[sce2s_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sce2s_c.html
[sct2e_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sct2e_c.html
[sce2c_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sce2c_c.html
[scencd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/scencd_c.html
[scdecd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/scdecd_c.html
[bodfnd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bodfnd_c.html
[bodvrd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bodvrd_c.html
[pxform_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pxform_c.html
[sxform_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sxform_c.html
[pckfrm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pckfrm_c.html
[pckcov_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pckcov_c.html
[pxfrm2_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pxfrm2_c.html
[spkezr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkezr_c.html
[spkpos_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkpos_c.html
[spkcpo_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkcpo_c.html
[spkcpt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkcpt_c.html
[spkcvo_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkcvo_c.html
[spkcvt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkcvt_c.html
[spkobj_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkobj_c.html
[spkcov_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkcov_c.html
[ckobj_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckobj_c.html
[ckcov_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckcov_c.html
[ckgp_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckgp_c.html
[ckgpav_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckgpav_c.html
[getfov_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/getfov_c.html
[gdpool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gdpool_c.html
[gipool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gipool_c.html
[gcpool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gcpool_c.html
[latsrf_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/latsrf_c.html
[srfnrm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/srfnrm_c.html
[dskz02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskz02_c.html
[dskp02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskp02_c.html
[dskv02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskv02_c.html
[dskobj_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskobj_c.html
[dsksrf_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dsksrf_c.html
[bodc2n_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bodc2n_c.html
[bodn2c_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bodn2c_c.html
[srfcss_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/srfcss_c.html
[srfs2c_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/srfs2c_c.html
[srfc2s_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/srfc2s_c.html
[srfscc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/srfscc_c.html
*/

/// Spice API for accessing kernel data.
mod kernel;

pub use self::kernel::*;
