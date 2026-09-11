/*!
An idiomatic interface in Rust to CSPICE.

## Description

Below you will find the index of the CSPICE functions that are wrapped with an idiomatic Rust
interface.

It takes a long time to correctly wrap all functions of the API. Raise an issue to ask a specific
function to be implemented and we will do it immediately. Pull requests are warmly welcomed to help
speed up this process (do not forget to include a proper documentation and a test).

In the meantime, if you are in a rush and need quickly to use a function not implemented with the
Rust interface, use the unsafe C functions [here][crate::c#functions]. You can find some inspiration
in the source of this lib to deal with the FFI types and unsafe code.

Functions come in two flavours. [`raw`] mirrors CSPICE argument for argument; [`neat`] wraps the
ones whose C signature asks for a buffer size, or for a caller allocated [cell],
so that Rust can size them for you. The index points at the flavour you most likely want, and each
page links to the other.

CSPICE reports failures through its own error state, and by default it prints to the screen then
**terminates the process**. [`errors`] wraps the routines that control it, and adds three helpers of
its own, which is why they are absent from the index below: [`errors::quiet`] and [`errors::loud`]
switch between returning and aborting, and [`errors::check`] turns the error state into a Rust
[`Result`].

## Bindings

CSPICE | **rust-spice** | Description
-------|--------------|------------
[appndc_c][appndc_c link] | [`raw::appndc`] | Append to a character cell
[appndd_c][appndd_c link] | [`raw::appndd`] | Append to a d.p. cell
[appndi_c][appndi_c link] | [`raw::appndi`] | Append to an integer cell
[axisar_c][axisar_c link] | [`raw::axisar`] | Axis and angle to rotation matrix
[azlrec_c][azlrec_c link] | [`raw::azlrec`] | Range, azimuth and elevation to rectangular
[b1900_c][b1900_c link] | [`raw::b1900`] | Toolkit constant
[b1950_c][b1950_c link] | [`raw::b1950`] | Toolkit constant
[bodc2n_c][bodc2n_c link] | [`neat::bodc2n`] | Body ID code to name translation
[bodc2s_c][bodc2s_c link] | [`neat::bodc2s`] | Body ID code to string translation
[bodfnd_c][bodfnd_c link] | [`raw::bodfnd`] | Find values from the kernel pool
[bodn2c_c][bodn2c_c link] | [`raw::bodn2c`] | Body name to ID code translation
[bods2c_c][bods2c_c link] | [`raw::bods2c`] | Body string to ID code translation
[bodvcd_c][bodvcd_c link] | [`raw::bodvcd`] | Return d.p. values from the kernel pool, by ID code
[bodvrd_c][bodvrd_c link] | [`raw::bodvrd`] | Return d.p. values from the kernel pool
[card_c][card_c link] | [`raw::card`] | Cardinality of a cell
[cgv2el_c][cgv2el_c link] | [`raw::cgv2el`] | Centre and generating vectors to ellipse
[ckcls_c][ckcls_c link] | [`raw::ckcls`] | CK, close file
[ckcov_c][ckcov_c link] | [`neat::ckcov`] | CK, coverage
[ckgp_c][ckgp_c link] | [`raw::ckgp`] | CK, get pointing
[ckgpav_c][ckgpav_c link] | [`raw::ckgpav`] | CK, get pointing and angular velocity
[ckobj_c][ckobj_c link] | [`neat::ckobj`] | CK, objects
[ckopn_c][ckopn_c link] | [`raw::ckopn`] | CK, open new file
[ckw03_c][ckw03_c link] | [`raw::ckw03`] | CK, write segment, type 3
[clight_c][clight_c link] | [`raw::clight`] | Toolkit constant
[clpool_c][clpool_c link] | [`raw::clpool`] | Clear the kernel pool
[conics_c][conics_c link] | [`raw::conics`] | Determine state from conic elements
[copy_c][copy_c link] | [`raw::copy`] | Copy a cell
[cyllat_c][cyllat_c link] | [`raw::cyllat`] | Cylindrical to latitudinal coordinates
[cylrec_c][cylrec_c link] | [`raw::cylrec`] | Cylindrical to rectangular coordinates
[cylsph_c][cylsph_c link] | [`raw::cylsph`] | Cylindrical to spherical coordinates
[dascls_c][dascls_c link] | [`raw::dascls`] | DAS, close file
[dasopr_c][dasopr_c link] | [`raw::dasopr`] | DAS, open for read
[dazldr_c][dazldr_c link] | [`raw::dazldr`] | Derivative of azimuth/elevation w.r.t. rectangular
[dcyldr_c][dcyldr_c link] | [`raw::dcyldr`] | Derivative of cylindrical w.r.t. rectangular
[deltet_c][deltet_c link] | [`raw::deltet`] | Delta ET, ET - UTC
[det_c][det_c link] | [`raw::det`] | Determinant of a double precision 3x3 matrix
[dgeodr_c][dgeodr_c link] | [`raw::dgeodr`] | Derivative of geodetic w.r.t. rectangular
[diff_c][diff_c link] | [`raw::diff`] | Difference of two sets
[dlabbs_c][dlabbs_c link] | [`raw::dlabbs`] | DLA, begin backward search
[dlabfs_c][dlabfs_c link] | [`raw::dlabfs`] | DLA, begin forward search
[dlafns_c][dlafns_c link] | [`raw::dlafns`] | DLA, find next segment
[dlatdr_c][dlatdr_c link] | [`raw::dlatdr`] | Derivative of latitudinal w.r.t. rectangular
[dpgrdr_c][dpgrdr_c link] | [`raw::dpgrdr`] | Derivative of planetographic w.r.t. rectangular
[dpr_c][dpr_c link] | [`raw::dpr`] | Degrees per radian
[drdazl_c][drdazl_c link] | [`raw::drdazl`] | Derivative of rectangular w.r.t. azimuth/elevation
[drdcyl_c][drdcyl_c link] | [`raw::drdcyl`] | Derivative of rectangular w.r.t. cylindrical
[drdgeo_c][drdgeo_c link] | [`raw::drdgeo`] | Derivative of rectangular w.r.t. geodetic
[drdlat_c][drdlat_c link] | [`raw::drdlat`] | Derivative of rectangular w.r.t. latitudinal
[drdpgr_c][drdpgr_c link] | [`raw::drdpgr`] | Derivative of rectangular w.r.t. planetographic
[drdsph_c][drdsph_c link] | [`raw::drdsph`] | Derivative of rectangular w.r.t. spherical
[dskcls_c][dskcls_c link] | [`raw::dskcls`] | DSK, close file
[dskgd_c][dskgd_c link] | [`raw::dskgd`] | DSK, return DSK segment descriptor
[dskgtl_c][dskgtl_c link] | [`raw::dskgtl`] | DSK, get tolerance
[dskmi2_c][dskmi2_c link] | [`raw::dskmi2`] | DSK, make spatial index for type 2 segment
[dskn02_c][dskn02_c link] | [`raw::dskn02`] | DSK, type 2, compute normal vector for plate
[dskobj_c][dskobj_c link] | [`neat::dskobj`] | DSK, get object IDs
[dskopn_c][dskopn_c link] | [`raw::dskopn`] | DSK, open new file
[dskp02_c][dskp02_c link] | [`neat::dskp02`] | DSK, fetch type 2 plate data
[dsksrf_c][dsksrf_c link] | [`neat::dsksrf`] | DSK, get surface IDs for body
[dskstl_c][dskstl_c link] | [`raw::dskstl`] | DSK, set tolerance
[dskv02_c][dskv02_c link] | [`neat::dskv02`] | DSK, fetch type 2 vertex data
[dskw02_c][dskw02_c link] | [`raw::dskw02`] | DSK, write type 2 segment
[dskx02_c][dskx02_c link] | [`raw::dskx02`] | DSK, ray-surface intercept, type 2
[dskxsi_c][dskxsi_c link] | [`raw::dskxsi`] | DSK, ray-surface intercept with source information
[dskxv_c][dskxv_c link] | [`raw::dskxv`] | DSK, ray-surface intercepts, vectorized
[dskz02_c][dskz02_c link] | [`raw::dskz02`] | DSK, fetch type 2 model size parameters
[dsphdr_c][dsphdr_c link] | [`raw::dsphdr`] | Derivative of spherical w.r.t. rectangular
[dtpool_c][dtpool_c link] | [`raw::dtpool`] | Data for a kernel pool variable
[edlimb_c][edlimb_c link] | [`raw::edlimb`] | Ellipsoid limb
[edterm_c][edterm_c link] | [`raw::edterm`] | Ellipsoid terminator
[el2cgv_c][el2cgv_c link] | [`raw::el2cgv`] | Ellipse to centre and generating vectors
[elemc_c][elemc_c link] | [`raw::elemc`] | Element of a character set
[elemd_c][elemd_c link] | [`raw::elemd`] | Element of a d.p. set
[elemi_c][elemi_c link] | [`raw::elemi`] | Element of an integer set
[erract_c][erract_c link] | [`errors::erract`] | Get/Set default error action
[errdev_c][errdev_c link] | [`errors::errdev`] | Get/Set error output device name
[errprt_c][errprt_c link] | [`errors::errprt`] | Get/Set error output items
[et2utc_c][et2utc_c link] | [`neat::et2utc`] | Ephemeris time to UTC
[eul2m_c][eul2m_c link] | [`raw::eul2m`] | Euler angles to matrix
[eul2xf_c][eul2xf_c link] | [`raw::eul2xf`] | Euler angles and derivatives to state transformation
[evsgp4_c][evsgp4_c link] | [`raw::evsgp4`] | Evaluate two-line element set with SGP4
[expool_c][expool_c link] | [`raw::expool`] | Confirm the existence of a pool kernel variable
[failed_c][failed_c link] | [`errors::failed`] | Error status indicator
[frmnam_c][frmnam_c link] | [`neat::frmnam`] | Frame to name translation
[furnsh_c][furnsh_c link] | [`raw::furnsh`] | Furnish a program with SPICE kernels
[gcpool_c][gcpool_c link] | [`neat::gcpool`] | Get character values from the kernel pool
[gdpool_c][gdpool_c link] | [`raw::gdpool`] | Get d.p. values from the kernel pool
[georec_c][georec_c link] | [`raw::georec`] | Geodetic to rectangular coordinates
[getelm_c][getelm_c link] | [`raw::getelm`] | Parse a two-line element set
[getfov_c][getfov_c link] | [`neat::getfov`] | Get instrument FOV parameters
[getmsg_c][getmsg_c link] | [`errors::getmsg`] | Get error message
[gfoclt_c][gfoclt_c link] | [`neat::gfoclt`] | GF, occultation search
[gipool_c][gipool_c link] | [`raw::gipool`] | Get integers from the kernel pool
[halfpi_c][halfpi_c link] | [`raw::halfpi`] | Toolkit constant
[ident_c][ident_c link] | [`raw::ident`] | Return the 3x3 identity matrix
[illumf_c][illumf_c link] | [`raw::illumf`] | Illumination angles, general source, return flags
[ilumin_c][ilumin_c link] | [`raw::ilumin`] | Illumination angles
[inedpl_c][inedpl_c link] | [`raw::inedpl`] | Intersection of an ellipsoid and a plane
[inelpl_c][inelpl_c link] | [`raw::inelpl`] | Intersection of an ellipse and a plane
[inrypl_c][inrypl_c link] | [`raw::inrypl`] | Intersection of a ray and a plane
[insrtc_c][insrtc_c link] | [`raw::insrtc`] | Insert into a character set
[insrtd_c][insrtd_c link] | [`raw::insrtd`] | Insert into a d.p. set
[insrti_c][insrti_c link] | [`raw::insrti`] | Insert into an integer set
[inter_c][inter_c link] | [`raw::inter`] | Intersection of two sets
[invert_c][invert_c link] | [`raw::invert`] | Invert a 3x3 matrix
[invort_c][invort_c link] | [`raw::invort`] | Invert nearly orthogonal matrices
[isrot_c][isrot_c link] | [`raw::isrot`] | Indicate whether a matrix is a rotation matrix
[j1900_c][j1900_c link] | [`raw::j1900`] | Toolkit constant
[j1950_c][j1950_c link] | [`raw::j1950`] | Toolkit constant
[j2000_c][j2000_c link] | [`raw::j2000`] | Toolkit constant
[j2100_c][j2100_c link] | [`raw::j2100`] | Toolkit constant
[jyear_c][jyear_c link] | [`raw::jyear`] | Toolkit constant
[kclear_c][kclear_c link] | [`raw::kclear`] | Keeper clear
[kdata_c][kdata_c link] | [`neat::kdata`] | Kernel data
[kinfo_c][kinfo_c link] | [`neat::kinfo`] | Kernel information
[ktotal_c][ktotal_c link] | [`raw::ktotal`] | Kernel totals
[latcyl_c][latcyl_c link] | [`raw::latcyl`] | Latitudinal to cylindrical coordinates
[latrec_c][latrec_c link] | [`raw::latrec`] | Latitudinal to rectangular coordinates
[latsph_c][latsph_c link] | [`raw::latsph`] | Latitudinal to spherical coordinates
[latsrf_c][latsrf_c link] | [`raw::latsrf`] | Latitudinal grid to surface points
[ldpool_c][ldpool_c link] | [`raw::ldpool`] | Load variables from a kernel file into the pool
[limbpt_c][limbpt_c link] | [`raw::limbpt`] | Limb points on an extended object
[lspcn_c][lspcn_c link] | [`raw::lspcn`] | Longitude of the sun, planetocentric
[m2eul_c][m2eul_c link] | [`raw::m2eul`] | Matrix to Euler angles
[m2q_c][m2q_c link] | [`raw::m2q`] | Matrix to quaternion
[mequ_c][mequ_c link] | [`raw::mequ`] | Matrix equal to another, 3x3
[mtxm_c][mtxm_c link] | [`raw::mtxm`] | Matrix transpose times matrix, 3x3
[mtxv_c][mtxv_c link] | [`raw::mtxv`] | Matrix transpose times vector, 3x3
[mxm_c][mxm_c link] | [`raw::mxm`] | Matrix times matrix, 3x3
[mxmt_c][mxmt_c link] | [`raw::mxmt`] | Matrix times matrix transpose, 3x3
[mxv_c][mxv_c link] | [`raw::mxv`] | Matrix times vector, 3x3
[namfrm_c][namfrm_c link] | [`raw::namfrm`] | Name to frame translation
[nearpt_c][nearpt_c link] | [`raw::nearpt`] | Nearest point on an ellipsoid
[npedln_c][npedln_c link] | [`raw::npedln`] | Nearest point on an ellipsoid to a line
[npelpt_c][npelpt_c link] | [`raw::npelpt`] | Nearest point on an ellipse to a point
[nplnpt_c][nplnpt_c link] | [`raw::nplnpt`] | Nearest point on a line to a point
[nvc2pl_c][nvc2pl_c link] | [`raw::nvc2pl`] | Normal vector and constant to plane
[nvp2pl_c][nvp2pl_c link] | [`raw::nvp2pl`] | Normal vector and point to plane
[occult_c][occult_c link] | [`raw::occult`] | Find occultation type at time
[oscelt_c][oscelt_c link] | [`raw::oscelt`] | Determine conic elements from state
[oscltx_c][oscltx_c link] | [`raw::oscltx`] | Extended osculating elements from state
[pckcov_c][pckcov_c link] | [`neat::pckcov`] | PCK, coverage
[pckfrm_c][pckfrm_c link] | [`neat::pckfrm`] | PCK, reference frame class ID set
[pcpool_c][pcpool_c link] | [`raw::pcpool`] | Put character strings into the kernel pool
[pdpool_c][pdpool_c link] | [`raw::pdpool`] | Put d.p.s into the kernel pool
[pgrrec_c][pgrrec_c link] | [`raw::pgrrec`] | Planetographic to rectangular
[phaseq_c][phaseq_c link] | [`raw::phaseq`] | Phase angle quantity between bodies
[pi_c][pi_c link] | [`raw::pi`] | Toolkit constant
[pipool_c][pipool_c link] | [`raw::pipool`] | Put integers into the kernel pool
[pjelpl_c][pjelpl_c link] | [`raw::pjelpl`] | Project an ellipse onto a plane
[pl2nvc_c][pl2nvc_c link] | [`raw::pl2nvc`] | Plane to normal vector and constant
[pl2nvp_c][pl2nvp_c link] | [`raw::pl2nvp`] | Plane to normal vector and point
[pl2psv_c][pl2psv_c link] | [`raw::pl2psv`] | Plane to point and spanning vectors
[prop2b_c][prop2b_c link] | [`raw::prop2b`] | Propagate a two-body solution
[psv2pl_c][psv2pl_c link] | [`raw::psv2pl`] | Point and spanning vectors to plane
[pxform_c][pxform_c link] | [`raw::pxform`] | Position transformation matrix
[pxfrm2_c][pxfrm2_c link] | [`raw::pxfrm2`] | Position transform matrix, different epochs
[q2m_c][q2m_c link] | [`raw::q2m`] | Quaternion to matrix
[qcktrc_c][qcktrc_c link] | [`errors::qcktrc`] | Get quick traceback
[qxq_c][qxq_c link] | [`raw::qxq`] | Quaternion times quaternion
[radrec_c][radrec_c link] | [`raw::radrec`] | RA and DEC to rectangular coordinates
[rav2xf_c][rav2xf_c link] | [`raw::rav2xf`] | Rotation and angular velocity to transform
[raxisa_c][raxisa_c link] | [`raw::raxisa`] | Rotation axis of a matrix
[recazl_c][recazl_c link] | [`raw::recazl`] | Rectangular to range, azimuth and elevation
[reccyl_c][reccyl_c link] | [`raw::reccyl`] | Rectangular to cylindrical coordinates
[recgeo_c][recgeo_c link] | [`raw::recgeo`] | Rectangular to geodetic coordinates
[reclat_c][reclat_c link] | [`raw::reclat`] | Rectangular to latitudinal coordinates
[recpgr_c][recpgr_c link] | [`raw::recpgr`] | Rectangular to planetographic
[recrad_c][recrad_c link] | [`raw::recrad`] | Rectangular coordinates to RA and DEC
[recsph_c][recsph_c link] | [`raw::recsph`] | Rectangular to spherical coordinates
[removc_c][removc_c link] | [`raw::removc`] | Remove from a character set
[removd_c][removd_c link] | [`raw::removd`] | Remove from a d.p. set
[removi_c][removi_c link] | [`raw::removi`] | Remove from an integer set
[reset_c][reset_c link] | [`errors::reset`] | Reset error status
[rotate_c][rotate_c link] | [`raw::rotate`] | Generate a rotation matrix
[rotmat_c][rotmat_c link] | [`raw::rotmat`] | Rotate a matrix
[rpd_c][rpd_c link] | [`raw::rpd`] | Radians per degree
[saelgv_c][saelgv_c link] | [`raw::saelgv`] | Semi-axes of an ellipse from generating vectors
[scard_c][scard_c link] | [`raw::scard`] | Set the cardinality of a cell
[scdecd_c][scdecd_c link] | [`neat::scdecd`] | Decode spacecraft clock
[sce2c_c][sce2c_c link] | [`raw::sce2c`] | ET to continuous SCLK ticks
[sce2s_c][sce2s_c link] | [`neat::sce2s`] | ET to SCLK string
[scencd_c][scencd_c link] | [`raw::scencd`] | Encode spacecraft clock
[scs2e_c][scs2e_c link] | [`raw::scs2e`] | SCLK string to ET
[sct2e_c][sct2e_c link] | [`raw::sct2e`] | SCLK ticks to ET
[sincpt_c][sincpt_c link] | [`raw::sincpt`] | Surface intercept
[size_c][size_c link] | [`raw::size`] | Size of a cell
[spd_c][spd_c link] | [`raw::spd`] | Seconds per day
[sphcyl_c][sphcyl_c link] | [`raw::sphcyl`] | Spherical to cylindrical coordinates
[sphlat_c][sphlat_c link] | [`raw::sphlat`] | Spherical to latitudinal coordinates
[sphrec_c][sphrec_c link] | [`raw::sphrec`] | Spherical to rectangular coordinates
[spkcls_c][spkcls_c link] | [`raw::spkcls`] | SPK, close file
[spkcov_c][spkcov_c link] | [`neat::spkcov`] | SPK, coverage
[spkcpo_c][spkcpo_c link] | [`raw::spkcpo`] | SPK, constant position observer state
[spkcpt_c][spkcpt_c link] | [`raw::spkcpt`] | SPK, constant position target state
[spkcvo_c][spkcvo_c link] | [`raw::spkcvo`] | SPK, constant velocity observer state
[spkcvt_c][spkcvt_c link] | [`raw::spkcvt`] | SPK, constant velocity target state
[spkez_c][spkez_c link] | [`raw::spkez`] | S/P Kernel, easy reader
[spkezp_c][spkezp_c link] | [`raw::spkezp`] | S/P Kernel, easy position
[spkezr_c][spkezr_c link] | [`raw::spkezr`] | S/P Kernel, easier reader
[spkgeo_c][spkgeo_c link] | [`raw::spkgeo`] | S/P Kernel, geometric state
[spkobj_c][spkobj_c link] | [`neat::spkobj`] | SPK, objects
[spkopa_c][spkopa_c link] | [`raw::spkopa`] | SPK, open for addition
[spkopn_c][spkopn_c link] | [`raw::spkopn`] | SPK, open new file
[spkpos_c][spkpos_c link] | [`raw::spkpos`] | S/P Kernel, position
[spkw09_c][spkw09_c link] | [`raw::spkw09`] | Write SPK segment, type 9
[srfc2s_c][srfc2s_c link] | [`neat::srfc2s`] | Surface and body ID codes to surface string
[srfcss_c][srfcss_c link] | [`neat::srfcss`] | Surface ID and body string to surface string
[srfnrm_c][srfnrm_c link] | [`raw::srfnrm`] | Map surface points to outward normal vectors
[srfrec_c][srfrec_c link] | [`raw::srfrec`] | Surface to rectangular coordinates
[srfs2c_c][srfs2c_c link] | [`raw::srfs2c`] | Surface and body strings to surface ID code
[srfscc_c][srfscc_c link] | [`raw::srfscc`] | Surface string and body ID code to surface ID code
[ssize_c][ssize_c link] | [`raw::ssize`] | Set the size of a cell
[str2et_c][str2et_c link] | [`raw::str2et`] | String to ET
[subpnt_c][subpnt_c link] | [`raw::subpnt`] | Sub-observer point
[subslr_c][subslr_c link] | [`raw::subslr`] | Sub-solar point
[surfnm_c][surfnm_c link] | [`raw::surfnm`] | Surface normal of an ellipsoid
[surfpt_c][surfpt_c link] | [`raw::surfpt`] | Surface point on an ellipsoid
[surfpv_c][surfpv_c link] | [`raw::surfpv`] | Surface point and velocity on an ellipsoid
[sxform_c][sxform_c link] | [`raw::sxform`] | State transformation matrix
[termpt_c][termpt_c link] | [`raw::termpt`] | Terminator points on an extended object
[timout_c][timout_c link] | [`neat::timout`] | Time output
[tkvrsn_c][tkvrsn_c link] | [`raw::tkvrsn`] | Toolkit version strings
[tparse_c][tparse_c link] | [`neat::tparse`] | Parse a UTC time string
[trace_c][trace_c link] | [`raw::trace`] | Trace of a 3x3 matrix
[twopi_c][twopi_c link] | [`raw::twopi`] | Toolkit constant
[twovec_c][twovec_c link] | [`raw::twovec`] | Two vectors defining an orthonormal frame
[tyear_c][tyear_c link] | [`raw::tyear`] | Toolkit constant
[union_c][union_c link] | [`raw::union`] | Union of two sets
[unitim_c][unitim_c link] | [`raw::unitim`] | Uniform time scale transformation
[unload_c][unload_c link] | [`raw::unload`] | Unload a kernel
[unorm_c][unorm_c link] | [`raw::unorm`] | Unit vector and norm, 3 dimensions
[utc2et_c][utc2et_c link] | [`raw::utc2et`] | UTC to ephemeris time
[vadd_c][vadd_c link] | [`raw::vadd`] | Vector addition, 3 dimensions
[valid_c][valid_c link] | [`raw::valid`] | Validate a set
[vcrss_c][vcrss_c link] | [`raw::vcrss`] | Vector cross product, 3 dimensions
[vdist_c][vdist_c link] | [`raw::vdist`] | Vector distance
[vdot_c][vdot_c link] | [`raw::vdot`] | Vector dot product, 3 dimensions
[vequ_c][vequ_c link] | [`raw::vequ`] | Vector equality, 3 dimensions
[vhat_c][vhat_c link] | [`raw::vhat`] | Unit vector along a vector, 3 dimensions
[vlcom3_c][vlcom3_c link] | [`raw::vlcom3`] | Linear combination of three vectors
[vlcom_c][vlcom_c link] | [`raw::vlcom`] | Linear combination of two vectors
[vminus_c][vminus_c link] | [`raw::vminus`] | Negate a vector, 3 dimensions
[vnorm_c][vnorm_c link] | [`raw::vnorm`] | Vector norm, 3 dimensions
[vpack_c][vpack_c link] | [`raw::vpack`] | Pack three scalars into a vector
[vperp_c][vperp_c link] | [`raw::vperp`] | Perpendicular component of a vector
[vprjp_c][vprjp_c link] | [`raw::vprjp`] | Project a vector onto a plane
[vprjpi_c][vprjpi_c link] | [`raw::vprjpi`] | Invert an orthogonal projection
[vproj_c][vproj_c link] | [`raw::vproj`] | Projection of a vector onto another
[vrel_c][vrel_c link] | [`raw::vrel`] | Vector relative difference, 3 dimensions
[vrotv_c][vrotv_c link] | [`raw::vrotv`] | Rotate a vector about an axis
[vscl_c][vscl_c link] | [`raw::vscl`] | Vector scaling, 3 dimensions
[vsep_c][vsep_c link] | [`raw::vsep`] | Angular separation of vectors, 3 dimensions
[vsub_c][vsub_c link] | [`raw::vsub`] | Vector subtraction, 3 dimensions
[vtmv_c][vtmv_c link] | [`raw::vtmv`] | Vector transpose times matrix times vector
[vupack_c][vupack_c link] | [`raw::vupack`] | Unpack a vector into three scalars
[vzero_c][vzero_c link] | [`raw::vzero`] | Is a vector the zero vector?
[wncard_c][wncard_c link] | [`raw::wncard`] | Cardinality of a double precision window
[wncomd_c][wncomd_c link] | [`raw::wncomd`] | Complement a double precision window
[wncond_c][wncond_c link] | [`raw::wncond`] | Contract the intervals of a window
[wndifd_c][wndifd_c link] | [`raw::wndifd`] | Difference two windows
[wnelmd_c][wnelmd_c link] | [`raw::wnelmd`] | Element of a double precision window
[wnexpd_c][wnexpd_c link] | [`raw::wnexpd`] | Expand the intervals of a window
[wnextd_c][wnextd_c link] | [`raw::wnextd`] | Extract the endpoints of a window
[wnfetd_c][wnfetd_c link] | [`raw::wnfetd`] | Fetch an interval from a window
[wnfild_c][wnfild_c link] | [`raw::wnfild`] | Fill small gaps in a window
[wnfltd_c][wnfltd_c link] | [`raw::wnfltd`] | Filter small intervals out of a window
[wnincd_c][wnincd_c link] | [`raw::wnincd`] | Included in a double precision window
[wninsd_c][wninsd_c link] | [`raw::wninsd`] | Insert an interval into a window
[wnintd_c][wnintd_c link] | [`raw::wnintd`] | Intersect two windows
[wnreld_c][wnreld_c link] | [`raw::wnreld`] | Compare two windows
[wnsumd_c][wnsumd_c link] | [`raw::wnsumd`] | Summary of a double precision window
[wnunid_c][wnunid_c link] | [`raw::wnunid`] | Union two windows
[wnvald_c][wnvald_c link] | [`raw::wnvald`] | Validate a double precision window
[xf2eul_c][xf2eul_c link] | [`raw::xf2eul`] | State transformation to Euler angles
[xf2rav_c][xf2rav_c link] | [`raw::xf2rav`] | Transform to rotation and angular velocity
[xfmsta_c][xfmsta_c link] | [`raw::xfmsta`] | Transform state between coordinate systems
[xpose6_c][xpose6_c link] | [`raw::xpose6`] | Transpose a matrix, 6x6
[xpose_c][xpose_c link] | [`raw::xpose`] | Transpose a matrix, 3x3

[appndc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/appndc_c.html
[appndd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/appndd_c.html
[appndi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/appndi_c.html
[axisar_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/axisar_c.html
[azlrec_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/azlrec_c.html
[b1900_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/b1900_c.html
[b1950_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/b1950_c.html
[bodc2n_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bodc2n_c.html
[bodc2s_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bodc2s_c.html
[bodfnd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bodfnd_c.html
[bodn2c_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bodn2c_c.html
[bods2c_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bods2c_c.html
[bodvcd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bodvcd_c.html
[bodvrd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bodvrd_c.html
[card_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/card_c.html
[cgv2el_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/cgv2el_c.html
[ckcls_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckcls_c.html
[ckcov_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckcov_c.html
[ckgp_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckgp_c.html
[ckgpav_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckgpav_c.html
[ckobj_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckobj_c.html
[ckopn_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckopn_c.html
[ckw03_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckw03_c.html
[clight_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/clight_c.html
[clpool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/clpool_c.html
[conics_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/conics_c.html
[copy_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/copy_c.html
[cyllat_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/cyllat_c.html
[cylrec_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/cylrec_c.html
[cylsph_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/cylsph_c.html
[dascls_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dascls_c.html
[dasopr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dasopr_c.html
[dazldr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dazldr_c.html
[dcyldr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dcyldr_c.html
[deltet_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/deltet_c.html
[det_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/det_c.html
[dgeodr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dgeodr_c.html
[diff_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/diff_c.html
[dlabbs_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dlabbs_c.html
[dlabfs_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dlabfs_c.html
[dlafns_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dlafns_c.html
[dlatdr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dlatdr_c.html
[dpgrdr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dpgrdr_c.html
[dpr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dpr_c.html
[drdazl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/drdazl_c.html
[drdcyl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/drdcyl_c.html
[drdgeo_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/drdgeo_c.html
[drdlat_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/drdlat_c.html
[drdpgr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/drdpgr_c.html
[drdsph_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/drdsph_c.html
[dskcls_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskcls_c.html
[dskgd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskgd_c.html
[dskgtl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskgtl_c.html
[dskmi2_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskmi2_c.html
[dskn02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskn02_c.html
[dskobj_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskobj_c.html
[dskopn_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskopn_c.html
[dskp02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskp02_c.html
[dsksrf_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dsksrf_c.html
[dskstl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskstl_c.html
[dskv02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskv02_c.html
[dskw02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskw02_c.html
[dskx02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskx02_c.html
[dskxsi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskxsi_c.html
[dskxv_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskxv_c.html
[dskz02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskz02_c.html
[dsphdr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dsphdr_c.html
[dtpool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dtpool_c.html
[edlimb_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/edlimb_c.html
[edterm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/edterm_c.html
[el2cgv_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/el2cgv_c.html
[elemc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/elemc_c.html
[elemd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/elemd_c.html
[elemi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/elemi_c.html
[erract_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/erract_c.html
[errdev_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/errdev_c.html
[errprt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/errprt_c.html
[et2utc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/et2utc_c.html
[eul2m_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/eul2m_c.html
[eul2xf_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/eul2xf_c.html
[evsgp4_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/evsgp4_c.html
[expool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/expool_c.html
[failed_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/failed_c.html
[frmnam_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/frmnam_c.html
[furnsh_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/furnsh_c.html
[gcpool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gcpool_c.html
[gdpool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gdpool_c.html
[georec_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/georec_c.html
[getelm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/getelm_c.html
[getfov_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/getfov_c.html
[getmsg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/getmsg_c.html
[gfoclt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gfoclt_c.html
[gipool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gipool_c.html
[halfpi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/halfpi_c.html
[ident_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ident_c.html
[illumf_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/illumf_c.html
[ilumin_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ilumin_c.html
[inedpl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/inedpl_c.html
[inelpl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/inelpl_c.html
[inrypl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/inrypl_c.html
[insrtc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/insrtc_c.html
[insrtd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/insrtd_c.html
[insrti_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/insrti_c.html
[inter_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/inter_c.html
[invert_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/invert_c.html
[invort_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/invort_c.html
[isrot_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/isrot_c.html
[j1900_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/j1900_c.html
[j1950_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/j1950_c.html
[j2000_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/j2000_c.html
[j2100_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/j2100_c.html
[jyear_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/jyear_c.html
[kclear_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/kclear_c.html
[kdata_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/kdata_c.html
[kinfo_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/kinfo_c.html
[ktotal_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ktotal_c.html
[latcyl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/latcyl_c.html
[latrec_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/latrec_c.html
[latsph_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/latsph_c.html
[latsrf_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/latsrf_c.html
[ldpool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ldpool_c.html
[limbpt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/limbpt_c.html
[lspcn_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/lspcn_c.html
[m2eul_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/m2eul_c.html
[m2q_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/m2q_c.html
[mequ_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/mequ_c.html
[mtxm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/mtxm_c.html
[mtxv_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/mtxv_c.html
[mxm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/mxm_c.html
[mxmt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/mxmt_c.html
[mxv_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/mxv_c.html
[namfrm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/namfrm_c.html
[nearpt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/nearpt_c.html
[npedln_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/npedln_c.html
[npelpt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/npelpt_c.html
[nplnpt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/nplnpt_c.html
[nvc2pl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/nvc2pl_c.html
[nvp2pl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/nvp2pl_c.html
[occult_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/occult_c.html
[oscelt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/oscelt_c.html
[oscltx_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/oscltx_c.html
[pckcov_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pckcov_c.html
[pckfrm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pckfrm_c.html
[pcpool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pcpool_c.html
[pdpool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pdpool_c.html
[pgrrec_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pgrrec_c.html
[phaseq_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/phaseq_c.html
[pi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pi_c.html
[pipool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pipool_c.html
[pjelpl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pjelpl_c.html
[pl2nvc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pl2nvc_c.html
[pl2nvp_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pl2nvp_c.html
[pl2psv_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pl2psv_c.html
[prop2b_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/prop2b_c.html
[psv2pl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/psv2pl_c.html
[pxform_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pxform_c.html
[pxfrm2_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pxfrm2_c.html
[q2m_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/q2m_c.html
[qcktrc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/qcktrc_c.html
[qxq_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/qxq_c.html
[radrec_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/radrec_c.html
[rav2xf_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/rav2xf_c.html
[raxisa_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/raxisa_c.html
[recazl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/recazl_c.html
[reccyl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/reccyl_c.html
[recgeo_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/recgeo_c.html
[reclat_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/reclat_c.html
[recpgr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/recpgr_c.html
[recrad_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/recrad_c.html
[recsph_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/recsph_c.html
[removc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/removc_c.html
[removd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/removd_c.html
[removi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/removi_c.html
[reset_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/reset_c.html
[rotate_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/rotate_c.html
[rotmat_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/rotmat_c.html
[rpd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/rpd_c.html
[saelgv_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/saelgv_c.html
[scard_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/scard_c.html
[scdecd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/scdecd_c.html
[sce2c_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sce2c_c.html
[sce2s_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sce2s_c.html
[scencd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/scencd_c.html
[scs2e_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/scs2e_c.html
[sct2e_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sct2e_c.html
[sincpt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sincpt_c.html
[size_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/size_c.html
[spd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spd_c.html
[sphcyl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sphcyl_c.html
[sphlat_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sphlat_c.html
[sphrec_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sphrec_c.html
[spkcls_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkcls_c.html
[spkcov_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkcov_c.html
[spkcpo_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkcpo_c.html
[spkcpt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkcpt_c.html
[spkcvo_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkcvo_c.html
[spkcvt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkcvt_c.html
[spkez_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkez_c.html
[spkezp_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkezp_c.html
[spkezr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkezr_c.html
[spkgeo_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkgeo_c.html
[spkobj_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkobj_c.html
[spkopa_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkopa_c.html
[spkopn_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkopn_c.html
[spkpos_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkpos_c.html
[spkw09_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkw09_c.html
[srfc2s_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/srfc2s_c.html
[srfcss_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/srfcss_c.html
[srfnrm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/srfnrm_c.html
[srfrec_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/srfrec_c.html
[srfs2c_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/srfs2c_c.html
[srfscc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/srfscc_c.html
[ssize_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ssize_c.html
[str2et_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/str2et_c.html
[subpnt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/subpnt_c.html
[subslr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/subslr_c.html
[surfnm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/surfnm_c.html
[surfpt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/surfpt_c.html
[surfpv_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/surfpv_c.html
[sxform_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sxform_c.html
[termpt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/termpt_c.html
[timout_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/timout_c.html
[tkvrsn_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/tkvrsn_c.html
[tparse_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/tparse_c.html
[trace_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/trace_c.html
[twopi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/twopi_c.html
[twovec_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/twovec_c.html
[tyear_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/tyear_c.html
[union_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/union_c.html
[unitim_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/unitim_c.html
[unload_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/unload_c.html
[unorm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/unorm_c.html
[utc2et_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/utc2et_c.html
[vadd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vadd_c.html
[valid_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/valid_c.html
[vcrss_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vcrss_c.html
[vdist_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vdist_c.html
[vdot_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vdot_c.html
[vequ_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vequ_c.html
[vhat_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vhat_c.html
[vlcom_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vlcom_c.html
[vlcom3_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vlcom3_c.html
[vminus_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vminus_c.html
[vnorm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vnorm_c.html
[vpack_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vpack_c.html
[vperp_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vperp_c.html
[vprjp_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vprjp_c.html
[vprjpi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vprjpi_c.html
[vproj_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vproj_c.html
[vrel_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vrel_c.html
[vrotv_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vrotv_c.html
[vscl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vscl_c.html
[vsep_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vsep_c.html
[vsub_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vsub_c.html
[vtmv_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vtmv_c.html
[vupack_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vupack_c.html
[vzero_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vzero_c.html
[wncard_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/wncard_c.html
[wncomd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/wncomd_c.html
[wncond_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/wncond_c.html
[wndifd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/wndifd_c.html
[wnelmd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/wnelmd_c.html
[wnexpd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/wnexpd_c.html
[wnextd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/wnextd_c.html
[wnfetd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/wnfetd_c.html
[wnfild_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/wnfild_c.html
[wnfltd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/wnfltd_c.html
[wnincd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/wnincd_c.html
[wninsd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/wninsd_c.html
[wnintd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/wnintd_c.html
[wnreld_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/wnreld_c.html
[wnsumd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/wnsumd_c.html
[wnunid_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/wnunid_c.html
[wnvald_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/wnvald_c.html
[xf2eul_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/xf2eul_c.html
[xf2rav_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/xf2rav_c.html
[xfmsta_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/xfmsta_c.html
[xpose_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/xpose_c.html
[xpose6_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/xpose6_c.html
*/

pub mod cell;
pub mod errors;
pub mod ffi;
pub mod neat;
pub mod raw;

#[cfg(any(feature = "lock", doc))]
#[cfg_attr(docsrs, doc(cfg(feature = "lock")))]
pub mod lock;

#[allow(unused_imports)]
pub use self::cell::{Cell, CellItem};
#[allow(unused_imports)]
pub use self::neat::{
    bodc2n, bodc2s, ckcov, ckobj, dskobj, dskp02, dsksrf, dskv02, et2utc, frmnam, gcpool, getfov,
    gfoclt, kdata, kinfo, pckcov, pckfrm, scdecd, sce2s, spkcov, spkobj, srfc2s, srfcss, timout,
    tparse,
};
#[allow(unused_imports)]
pub use self::raw::{
    appndc, appndd, appndi, axisar, azlrec, b1900, b1950, bodfnd, bodn2c, bods2c, bodvcd, bodvrd,
    card, cgv2el, ckcls, ckgp, ckgpav, ckopn, ckw03, clight, clpool, conics, copy, cyllat, cylrec,
    cylsph, dascls, dasopr, dazldr, dcyldr, deltet, det, dgeodr, diff, dlabbs, dlabfs, dlafns,
    dlatdr, dpgrdr, dpr, drdazl, drdcyl, drdgeo, drdlat, drdpgr, drdsph, dskcls, dskgd, dskgtl,
    dskmi2, dskn02, dskopn, dskstl, dskw02, dskx02, dskxsi, dskxv, dskz02, dsphdr, dtpool, edlimb,
    edterm, el2cgv, elemc, elemd, elemi, eul2m, eul2xf, evsgp4, expool, furnsh, gdpool, georec,
    getelm, gipool, halfpi, ident, illumf, ilumin, inedpl, inelpl, inrypl, insrtc, insrtd, insrti,
    inter, invert, invort, isrot, j1900, j1950, j2000, j2100, jyear, kclear, ktotal, latcyl,
    latrec, latsph, latsrf, ldpool, limbpt, lspcn, m2eul, m2q, mequ, mtxm, mtxv, mxm, mxmt, mxv,
    namfrm, nearpt, npedln, npelpt, nplnpt, nvc2pl, nvp2pl, occult, oscelt, oscltx, pcpool, pdpool,
    pgrrec, phaseq, pi, pipool, pjelpl, pl2nvc, pl2nvp, pl2psv, prop2b, psv2pl, pxform, pxfrm2,
    q2m, qxq, radrec, rav2xf, raxisa, recazl, reccyl, recgeo, reclat, recpgr, recrad, recsph,
    removc, removd, removi, rotate, rotmat, rpd, saelgv, scard, sce2c, scencd, scs2e, sct2e,
    sincpt, size, spd, sphcyl, sphlat, sphrec, spkcls, spkcpo, spkcpt, spkcvo, spkcvt, spkez,
    spkezp, spkezr, spkgeo, spkopa, spkopn, spkpos, spkw09, srfnrm, srfrec, srfs2c, srfscc, ssize,
    str2et, subpnt, subslr, surfnm, surfpt, surfpv, sxform, termpt, tkvrsn, trace, twopi, twovec,
    tyear, union, unitim, unload, unorm, utc2et, vadd, valid, vcrss, vdist, vdot, vequ, vhat,
    vlcom, vlcom3, vminus, vnorm, vpack, vperp, vprjp, vprjpi, vproj, vrel, vrotv, vscl, vsep,
    vsub, vtmv, vupack, vzero, wncard, wncomd, wncond, wndifd, wnelmd, wnexpd, wnextd, wnfetd,
    wnfild, wnfltd, wnincd, wninsd, wnintd, wnreld, wnsumd, wnunid, wnvald, xf2eul, xf2rav, xfmsta,
    xpose, xpose6, CELL, DLADSC, DSK02_SPADSZ, DSKDSC, DSKXSI_DCSIZE, DSKXSI_ICSIZE, DSK_KEYAMG,
    DSK_KEYLAL, DSK_KEYPTM, DSK_KEYSGR, DSK_KEYSPM, DSK_KEYXFR, DSK_NSYPAR, ELLIPSE, PLANE,
    TLE_NELTS, TLE_NGEOPHS,
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
Maximum size of string outputs.
*/
pub const MAX_LEN_OUT: usize = 256;

/**
Allocate zeroed memory for a given type and number of elements.

Kept for use with the [unsafe C API][crate::c]; the safe wrappers no longer need it. The memory is
never freed, so prefer a [`Vec`] when you can.
*/
#[macro_export]
macro_rules! malloc {
    ($a:ty, $n:expr) => {{
        let count = $n as usize;
        unsafe { libc::calloc(count, std::mem::size_of::<$a>()) as *mut $a }
    }};
}

/**
Allocate a zeroed [`*mut i8`][`std::os::raw::c_char`] to be sent as a pointer to a string.

Kept for use with the [unsafe C API][crate::c]; the memory is never freed.
*/
#[macro_export]
macro_rules! mallocstr {
    ($s:expr) => {
        $crate::malloc!(i8, $s as usize + 1)
    };
}

/**
Convert [`String`] to [`*mut i8`][`std::os::raw::c_char`].

Kept for use with the [unsafe C API][crate::c]. The string is leaked: it has to outlive the C call,
and there is no way for the macro to know when that is. Prefer
[`ffi::to_cstring`][crate::core::ffi::to_cstring], which hands back an owned [`std::ffi::CString`].

# Panics

Panics if the string contains an interior null byte.
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

# Safety

The pointer must be non-null and point to a null terminated string.
*/
#[macro_export]
macro_rules! fcstr {
    ($s:expr) => {{
        let pointer = $s;
        unsafe {
            std::ffi::CStr::from_ptr(pointer)
                .to_string_lossy()
                .into_owned()
        }
    }};
}

/**
Pointer to expression.
*/
#[macro_export]
macro_rules! mptr {
    ($e:expr) => {
        $e.as_mut_ptr()
    };
}

/**
Allocate a scalar to be sent as a pointer.
*/
#[macro_export]
macro_rules! init_scalar {
    () => {
        std::mem::MaybeUninit::uninit()
    };
}

/**
Retrieve a value from a pointer to a scalar created using [`init_scalar`].

# Safety

The scalar must have been initialised, typically by the C call it was handed to.
*/
#[macro_export]
macro_rules! get_scalar {
    ($e:expr) => {
        $e.assume_init()
    };
}

/**
Get a vector of array from the pointer to the array.

# Safety

The pointer must be valid for `$n` reads.
*/
#[macro_export]
macro_rules! get_varr {
    ($e:expr, $n:expr) => {
        std::slice::from_raw_parts($e, $n as usize).to_vec()
    };
}
