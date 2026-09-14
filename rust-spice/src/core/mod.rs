/*!
An idiomatic interface in Rust to CSPICE.

## Description

Below you will find the index of the CSPICE functions that are wrapped with an idiomatic Rust
interface.

Every routine of the toolkit is there. Should you want to reach past them anyway, the unsafe C
functions are [here][crate::c#functions].

Functions come in two flavours. [`raw`] mirrors CSPICE argument for argument; [`neat`] wraps the
ones whose C signature asks for a buffer size, or for a caller allocated [cell],
so that Rust can size them for you. The index points at the flavour you most likely want, and each
page links to the other.

The headers of CSPICE N0067 declare 649 `*_c` functions. Four of them are private internals whose
names begin with `zz`, and `prefix_c` is declared in `SpiceZpr.h` but is not compiled into the
library NAIF ships, so it cannot be called at all. That leaves 644, and the index below lists all
of them.

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
[azlcpo_c][azlcpo_c link] | [`raw::azlcpo`] | Return the azimuth/elevation coordinates of a specified target relative to an "observer," wh
[azlrec_c][azlrec_c link] | [`raw::azlrec`] | Range, azimuth and elevation to rectangular
[b1900_c][b1900_c link] | [`raw::b1900`] | Toolkit constant
[b1950_c][b1950_c link] | [`raw::b1950`] | Toolkit constant
[badkpv_c][badkpv_c link] | [`raw::badkpv`] | Determine if a kernel pool variable is present and if so that it has the correct size and type
[bltfrm_c][bltfrm_c link] | [`raw::bltfrm`] | Return a SPICE set containing the frame IDs of all built-in frames of a specified class
[bodc2n_c][bodc2n_c link] | [`neat::bodc2n`] | Body ID code to name translation
[bodc2s_c][bodc2s_c link] | [`neat::bodc2s`] | Body ID code to string translation
[boddef_c][boddef_c link] | [`raw::boddef`] | Define a body name/ID code pair for later translation via bodn2c_c or bodc2n_c
[bodfnd_c][bodfnd_c link] | [`raw::bodfnd`] | Find values from the kernel pool
[bodn2c_c][bodn2c_c link] | [`raw::bodn2c`] | Body name to ID code translation
[bods2c_c][bods2c_c link] | [`raw::bods2c`] | Body string to ID code translation
[bodvar_c][bodvar_c link] | [`raw::bodvar`] | Deprecated: This routine has been superseded by bodvcd_c and bodvrd_c
[bodvcd_c][bodvcd_c link] | [`raw::bodvcd`] | Return d.p. values from the kernel pool, by ID code
[bodvrd_c][bodvrd_c link] | [`raw::bodvrd`] | Return d.p. values from the kernel pool
[brcktd_c][brcktd_c link] | [`raw::brcktd`] | Bracket a d.p. value
[brckti_c][brckti_c link] | [`raw::brckti`] | Bracket an integer value
[bschoc_c][bschoc_c link] | [`raw::bschoc`] | Binary search an ordered character array
[bschoi_c][bschoi_c link] | [`raw::bschoi`] | Binary search an ordered integer array
[bsrchc_c][bsrchc_c link] | [`raw::bsrchc`] | Binary search a character array
[bsrchd_c][bsrchd_c link] | [`raw::bsrchd`] | Binary search a d.p. array
[bsrchi_c][bsrchi_c link] | [`raw::bsrchi`] | Binary search an integer array
[card_c][card_c link] | [`raw::card`] | Cardinality of a cell
[ccifrm_c][ccifrm_c link] | [`raw::ccifrm`] | Return the frame name, frame ID, and center associated with a given frame class and class ID
[cgv2el_c][cgv2el_c link] | [`raw::cgv2el`] | Centre and generating vectors to ellipse
[chbder_c][chbder_c link] | [`raw::chbder`] | Return the value of a polynomial and its first `nderiv' derivatives, evaluated at the input
[chbigr_c][chbigr_c link] | [`raw::chbigr`] | Evaluate an indefinite integral of a Chebyshev expansion at a specified point `x' and return
[chbint_c][chbint_c link] | [`raw::chbint`] | Return the value of a polynomial and its derivative, evaluated at the input `x', using the c
[chbval_c][chbval_c link] | [`raw::chbval`] | Return the value of a polynomial evaluated at the input `x' using the coefficients for the C
[chkin_c][chkin_c link] | [`raw::chkin`] | Inform the CSPICE error handling mechanism of entry into a routine
[chkout_c][chkout_c link] | [`raw::chkout`] | Inform the CSPICE error handling mechanism of exit from a routine
[cidfrm_c][cidfrm_c link] | [`raw::cidfrm`] | Retrieve frame ID code and name to associate with a frame center
[ckcls_c][ckcls_c link] | [`raw::ckcls`] | CK, close file
[ckcov_c][ckcov_c link] | [`neat::ckcov`] | CK, coverage
[ckfrot_c][ckfrot_c link] | [`raw::ckfrot`] | Find the position rotation matrix from a C-kernel (CK) frame with the specified frame class
[ckfxfm_c][ckfxfm_c link] | [`raw::ckfxfm`] | Find the state transformation matrix from a C-kernel (CK) frame with the specified frame cla
[ckgp_c][ckgp_c link] | [`raw::ckgp`] | CK, get pointing
[ckgpav_c][ckgpav_c link] | [`raw::ckgpav`] | CK, get pointing and angular velocity
[ckgr02_c][ckgr02_c link] | [`raw::ckgr02`] | Return a specified pointing instance from a CK type 02 segment
[ckgr03_c][ckgr03_c link] | [`raw::ckgr03`] | Return a specified pointing instance from a CK type 03 segment
[cklpf_c][cklpf_c link] | [`raw::cklpf`] | Load a CK pointing file for use by the CK readers
[ckmeta_c][ckmeta_c link] | [`raw::ckmeta`] | Return (depending upon the user's request) the ID code of either the spacecraft or spacecraf
[cknr02_c][cknr02_c link] | [`raw::cknr02`] | Return the number of pointing records in a CK type 02 segment
[cknr03_c][cknr03_c link] | [`raw::cknr03`] | Return the number of pointing instances in a CK type 03 segment
[ckobj_c][ckobj_c link] | [`neat::ckobj`] | CK, objects
[ckopn_c][ckopn_c link] | [`raw::ckopn`] | CK, open new file
[ckupf_c][ckupf_c link] | [`raw::ckupf`] | Unload a CK pointing file so that it will no longer be searched by the readers
[ckw01_c][ckw01_c link] | [`raw::ckw01`] | Add a type 1 segment to a C-kernel
[ckw02_c][ckw02_c link] | [`raw::ckw02`] | Write a type 2 segment to a C-kernel
[ckw03_c][ckw03_c link] | [`raw::ckw03`] | CK, write segment, type 3
[ckw05_c][ckw05_c link] | [`raw::ckw05`] | Write a type 5 segment to a CK file
[clearc_c][clearc_c link] | [`raw::clearc`] | Fill a two-dimensional character array with blank strings
[cleard_c][cleard_c link] | [`raw::cleard`] | Fill a double precision array with zeros
[cleari_c][cleari_c link] | [`raw::cleari`] | Fill an integer array with zeros
[clight_c][clight_c link] | [`raw::clight`] | Toolkit constant
[clpool_c][clpool_c link] | [`raw::clpool`] | Clear the kernel pool
[cmprss_c][cmprss_c link] | [`neat::cmprss`] | Compress a character sequence
[cnmfrm_c][cnmfrm_c link] | [`raw::cnmfrm`] | Retrieve frame ID code and name to associate with an object
[conics_c][conics_c link] | [`raw::conics`] | Determine state from conic elements
[convrt_c][convrt_c link] | [`raw::convrt`] | Take a measurement X, the units associated with X, and units to which X should be converted; ret
[copy_c][copy_c link] | [`raw::copy`] | Copy a cell
[cpos_c][cpos_c link] | [`raw::cpos`] | Find the first occurrence in a string of a character belonging to a collection of characters, st
[cposr_c][cposr_c link] | [`raw::cposr`] | Find the first occurrence in a string of a character belonging to a collection of characters, st
[cvpool_c][cvpool_c link] | [`raw::cvpool`] | Indicate whether or not any watched kernel variables that have a specified agent on their notifi
[cyllat_c][cyllat_c link] | [`raw::cyllat`] | Cylindrical to latitudinal coordinates
[cylrec_c][cylrec_c link] | [`raw::cylrec`] | Cylindrical to rectangular coordinates
[cylsph_c][cylsph_c link] | [`raw::cylsph`] | Cylindrical to spherical coordinates
[dafac_c][dafac_c link] | [`raw::dafac`] | Add comments from a buffer of character strings to the comment area of a binary DAF file, ap
[dafbbs_c][dafbbs_c link] | [`raw::dafbbs`] | Begin a backward search for arrays in a DAF
[dafbfs_c][dafbfs_c link] | [`raw::dafbfs`] | Begin a forward search for arrays in a DAF
[dafcls_c][dafcls_c link] | [`raw::dafcls`] | Close the DAF associated with a given handle
[dafcs_c][dafcs_c link] | [`raw::dafcs`] | Select a DAF that already has a search in progress as the one to continue searching
[dafdc_c][dafdc_c link] | [`raw::dafdc`] | Delete the entire comment area of a specified DAF file
[dafec_c][dafec_c link] | [`neat::dafec`] | Extract comments from the comment area of a binary DAF
[daffna_c][daffna_c link] | [`raw::daffna`] | Find the next (forward) array in the current DAF
[daffpa_c][daffpa_c link] | [`raw::daffpa`] | Find the previous (backward) array in the current DAF
[dafgda_c][dafgda_c link] | [`raw::dafgda`] | Read the double precision data bounded by two addresses within a DAF
[dafgh_c][dafgh_c link] | [`raw::dafgh`] | Return (get) the handle of the DAF currently being searched
[dafgn_c][dafgn_c link] | [`raw::dafgn`] | Return (get) the name for the current array in the current DAF
[dafgs_c][dafgs_c link] | [`raw::dafgs`] | Return (get) the summary for the current array in the current DAF
[dafgsr_c][dafgsr_c link] | [`raw::dafgsr`] | Read a portion of the contents of a summary record in a DAF file
[dafhsf_c][dafhsf_c link] | [`raw::dafhsf`] | Return the summary format associated with a handle
[dafopr_c][dafopr_c link] | [`raw::dafopr`] | Open a DAF for subsequent read requests
[dafopw_c][dafopw_c link] | [`raw::dafopw`] | Open a DAF for subsequent write requests
[dafps_c][dafps_c link] | [`raw::dafps`] | Pack (assemble) an array summary from its double precision and integer components
[dafrda_c][dafrda_c link] | [`raw::dafrda`] | Deprecated: This routine has been superseded by the CSPICE routines dafgda_c and dafgsr_c
[dafrfr_c][dafrfr_c link] | [`raw::dafrfr`] | Read the contents of the file record of a DAF
[dafrs_c][dafrs_c link] | [`raw::dafrs`] | Change the summary for the current array in the current DAF
[dafus_c][dafus_c link] | [`raw::dafus`] | Unpack an array summary into its double precision and integer components
[dasac_c][dasac_c link] | [`raw::dasac`] | Add comments from a buffer of character strings to the comment area of a binary DAS file, ap
[dasadc_c][dasadc_c link] | [`raw::dasadc`] | Add character data to a DAS file
[dasadd_c][dasadd_c link] | [`raw::dasadd`] | Add an array of double precision numbers to a DAS file
[dasadi_c][dasadi_c link] | [`raw::dasadi`] | Add an array of integers to a DAS file
[dascls_c][dascls_c link] | [`raw::dascls`] | DAS, close file
[dasdc_c][dasdc_c link] | [`raw::dasdc`] | Delete the entire comment area of a previously opened binary DAS file
[dasec_c][dasec_c link] | [`neat::dasec`] | Extract comments from the comment area of a binary DAS file
[dashfn_c][dashfn_c link] | [`raw::dashfn`] | Return the name of the DAS file associated with a handle
[dashfs_c][dashfs_c link] | [`raw::dashfs`] | Return a file summary for a specified DAS file
[daslla_c][daslla_c link] | [`raw::daslla`] | Return last DAS logical addresses of character, double precision and integer type that are c
[dasllc_c][dasllc_c link] | [`raw::dasllc`] | Close the DAS file associated with a given handle, without flushing buffered data or segrega
[dasonw_c][dasonw_c link] | [`raw::dasonw`] | Open a new DAS file and set the file type
[dasopr_c][dasopr_c link] | [`raw::dasopr`] | DAS, open for read
[dasops_c][dasops_c link] | [`raw::dasops`] | Open a scratch DAS file for writing
[dasopw_c][dasopw_c link] | [`raw::dasopw`] | Open a DAS file for writing
[dasrdc_c][dasrdc_c link] | [`raw::dasrdc`] | Read character data from a range of DAS logical addresses
[dasrdd_c][dasrdd_c link] | [`raw::dasrdd`] | Read double precision data from a range of DAS logical addresses
[dasrdi_c][dasrdi_c link] | [`raw::dasrdi`] | Read integer data from a range of DAS logical addresses
[dasrfr_c][dasrfr_c link] | [`raw::dasrfr`] | Return the contents of the file record of a specified DAS file
[dasudc_c][dasudc_c link] | [`raw::dasudc`] | Update character data in a specified range of DAS logical addresses with substrings of a cha
[dasudd_c][dasudd_c link] | [`raw::dasudd`] | Update data in a specified range of double precision addresses in a DAS file
[dasudi_c][dasudi_c link] | [`raw::dasudi`] | Update data in a specified range of integer addresses in a DAS file
[daswbr_c][daswbr_c link] | [`raw::daswbr`] | Write out all buffered records of a specified DAS file
[dazldr_c][dazldr_c link] | [`raw::dazldr`] | Derivative of azimuth/elevation w.r.t. rectangular
[dcyldr_c][dcyldr_c link] | [`raw::dcyldr`] | Derivative of cylindrical w.r.t. rectangular
[deltet_c][deltet_c link] | [`raw::deltet`] | Delta ET, ET - UTC
[det_c][det_c link] | [`raw::det`] | Determinant of a double precision 3x3 matrix
[dgeodr_c][dgeodr_c link] | [`raw::dgeodr`] | Derivative of geodetic w.r.t. rectangular
[diags2_c][diags2_c link] | [`raw::diags2`] | Diagonalize a symmetric 2x2 matrix
[diff_c][diff_c link] | [`raw::diff`] | Difference of two sets
[dlabbs_c][dlabbs_c link] | [`raw::dlabbs`] | DLA, begin backward search
[dlabfs_c][dlabfs_c link] | [`raw::dlabfs`] | DLA, begin forward search
[dlabns_c][dlabns_c link] | [`raw::dlabns`] | Begin a new segment in a DLA file
[dlaens_c][dlaens_c link] | [`raw::dlaens`] | End a new segment in a DLA file
[dlafns_c][dlafns_c link] | [`raw::dlafns`] | DLA, find next segment
[dlafps_c][dlafps_c link] | [`raw::dlafps`] | Find the segment preceding a specified segment in a DLA file
[dlaopn_c][dlaopn_c link] | [`raw::dlaopn`] | Open a new DLA file and set the file type
[dlatdr_c][dlatdr_c link] | [`raw::dlatdr`] | Derivative of latitudinal w.r.t. rectangular
[dnearp_c][dnearp_c link] | [`raw::dnearp`] | Compute the state (position and velocity) of an ellipsoid surface point nearest to the posit
[dp2hx_c][dp2hx_c link] | [`raw::dp2hx`] | Convert a double precision number to an equivalent character string using a base 16 "scientific
[dpgrdr_c][dpgrdr_c link] | [`raw::dpgrdr`] | Derivative of planetographic w.r.t. rectangular
[dpmax_c][dpmax_c link] | [`raw::dpmax`] | Return the value of the largest (positive) number representable in a double precision variable
[dpmin_c][dpmin_c link] | [`raw::dpmin`] | Return the value of the smallest (negative) number representable in a double precision variable
[dpr_c][dpr_c link] | [`raw::dpr`] | Degrees per radian
[drdazl_c][drdazl_c link] | [`raw::drdazl`] | Derivative of rectangular w.r.t. azimuth/elevation
[drdcyl_c][drdcyl_c link] | [`raw::drdcyl`] | Derivative of rectangular w.r.t. cylindrical
[drdgeo_c][drdgeo_c link] | [`raw::drdgeo`] | Derivative of rectangular w.r.t. geodetic
[drdlat_c][drdlat_c link] | [`raw::drdlat`] | Derivative of rectangular w.r.t. latitudinal
[drdpgr_c][drdpgr_c link] | [`raw::drdpgr`] | Derivative of rectangular w.r.t. planetographic
[drdsph_c][drdsph_c link] | [`raw::drdsph`] | Derivative of rectangular w.r.t. spherical
[dskb02_c][dskb02_c link] | [`raw::dskb02`] | Return bookkeeping data from a DSK type 2 segment
[dskcls_c][dskcls_c link] | [`raw::dskcls`] | DSK, close file
[dskd02_c][dskd02_c link] | [`raw::dskd02`] | Fetch double precision data from a type 2 DSK segment
[dskgd_c][dskgd_c link] | [`raw::dskgd`] | DSK, return DSK segment descriptor
[dskgtl_c][dskgtl_c link] | [`raw::dskgtl`] | DSK, get tolerance
[dski02_c][dski02_c link] | [`raw::dski02`] | Fetch integer data from a type 2 DSK segment
[dskmi2_c][dskmi2_c link] | [`raw::dskmi2`] | DSK, make spatial index for type 2 segment
[dskn02_c][dskn02_c link] | [`raw::dskn02`] | DSK, type 2, compute normal vector for plate
[dskobj_c][dskobj_c link] | [`neat::dskobj`] | DSK, get object IDs
[dskopn_c][dskopn_c link] | [`raw::dskopn`] | DSK, open new file
[dskp02_c][dskp02_c link] | [`neat::dskp02`] | DSK, fetch type 2 plate data
[dskrb2_c][dskrb2_c link] | [`raw::dskrb2`] | Determine range bounds for a set of triangular plates to be stored in a type 2 DSK segment
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
[ducrss_c][ducrss_c link] | [`raw::ducrss`] | Compute the unit vector parallel to the cross product of two 3-dimensional vectors and the d
[dvcrss_c][dvcrss_c link] | [`raw::dvcrss`] | Compute the cross product of two 3-dimensional vectors and the derivative of this cross prod
[dvdot_c][dvdot_c link] | [`raw::dvdot`] | Compute the derivative of the dot product of two double precision position vectors
[dvhat_c][dvhat_c link] | [`raw::dvhat`] | Find the unit vector corresponding to a state vector and the derivative of the unit vector
[dvnorm_c][dvnorm_c link] | [`raw::dvnorm`] | Calculate the derivative of the norm of a 3-vector
[dvpool_c][dvpool_c link] | [`raw::dvpool`] | Delete a variable from the kernel pool
[dvsep_c][dvsep_c link] | [`raw::dvsep`] | Calculate the time derivative of the separation angle between two input states, S1 and S2
[edlimb_c][edlimb_c link] | [`raw::edlimb`] | Ellipsoid limb
[ednmpt_c][ednmpt_c link] | [`raw::ednmpt`] | Return the unique point on an ellipsoid's surface where the outward normal direction is a gi
[edpnt_c][edpnt_c link] | [`raw::edpnt`] | Scale a point so that it lies on the surface of a specified triaxial ellipsoid that is cente
[edterm_c][edterm_c link] | [`raw::edterm`] | Ellipsoid terminator
[ekacec_c][ekacec_c link] | [`raw::ekacec`] | Add data to a character column in a specified EK record
[ekaced_c][ekaced_c link] | [`raw::ekaced`] | Add data to an double precision column in a specified EK record
[ekacei_c][ekacei_c link] | [`raw::ekacei`] | Add data to an integer column in a specified EK record
[ekaclc_c][ekaclc_c link] | [`raw::ekaclc`] | Add an entire character column to an EK segment
[ekacld_c][ekacld_c link] | [`raw::ekacld`] | Add an entire double precision column to an EK segment
[ekacli_c][ekacli_c link] | [`raw::ekacli`] | Add an entire integer column to an EK segment
[ekappr_c][ekappr_c link] | [`raw::ekappr`] | Append a new, empty record at the end of a specified E-kernel segment
[ekbseg_c][ekbseg_c link] | [`raw::ekbseg`] | Start a new segment in an E-kernel
[ekccnt_c][ekccnt_c link] | [`raw::ekccnt`] | Return the number of distinct columns in a specified, currently loaded table
[ekcii_c][ekcii_c link] | [`raw::ekcii`] | Return attribute information about a column belonging to a loaded EK table, specifying the c
[ekcls_c][ekcls_c link] | [`raw::ekcls`] | Close an E-kernel
[ekdelr_c][ekdelr_c link] | [`raw::ekdelr`] | Delete a specified record from a specified E-kernel segment
[ekffld_c][ekffld_c link] | [`raw::ekffld`] | Complete a fast write operation on a new E-kernel segment
[ekfind_c][ekfind_c link] | [`raw::ekfind`] | Find E-kernel data that satisfy a set of constraints
[ekgc_c][ekgc_c link] | [`raw::ekgc`] | Return an element of an entry in a column of character type in a specified row
[ekgd_c][ekgd_c link] | [`raw::ekgd`] | Return an element of an entry in a column of double precision type in a specified row
[ekgi_c][ekgi_c link] | [`raw::ekgi`] | Return an element of an entry in a column of integer type in a specified row
[ekifld_c][ekifld_c link] | [`raw::ekifld`] | Initialize a new E-kernel segment to allow fast writing
[ekinsr_c][ekinsr_c link] | [`raw::ekinsr`] | Add a new, empty record to a specified E-kernel segment at a specified index
[eklef_c][eklef_c link] | [`raw::eklef`] | Load an EK file, making it accessible to the EK readers
[eknelt_c][eknelt_c link] | [`raw::eknelt`] | Return the number of elements in a specified column entry in the current row
[eknseg_c][eknseg_c link] | [`raw::eknseg`] | Return the number of segments in a specified EK
[ekntab_c][ekntab_c link] | [`raw::ekntab`] | Return the number of loaded EK tables
[ekopn_c][ekopn_c link] | [`raw::ekopn`] | Open a new E-kernel file and prepare the file for writing
[ekopr_c][ekopr_c link] | [`raw::ekopr`] | Open an existing E-kernel file for reading
[ekops_c][ekops_c link] | [`raw::ekops`] | Open a scratch (temporary) E-kernel file and prepare the file for writing
[ekopw_c][ekopw_c link] | [`raw::ekopw`] | Open an existing E-kernel file for writing
[ekpsel_c][ekpsel_c link] | [`raw::ekpsel`] | Parse the SELECT clause of an EK query, returning full particulars concerning each selected
[ekrcec_c][ekrcec_c link] | [`raw::ekrcec`] | Read data from a character column in a specified EK record
[ekrced_c][ekrced_c link] | [`raw::ekrced`] | Read data from a double precision column in a specified EK record
[ekrcei_c][ekrcei_c link] | [`raw::ekrcei`] | Read data from an integer column in a specified EK record
[ekssum_c][ekssum_c link] | [`raw::ekssum`] | Return summary information for a specified segment in a specified EK
[ektnam_c][ektnam_c link] | [`raw::ektnam`] | Return the name of a specified, loaded table
[ekucec_c][ekucec_c link] | [`raw::ekucec`] | Update a character column entry in a specified EK record
[ekuced_c][ekuced_c link] | [`raw::ekuced`] | Update a double precision column entry in a specified EK record
[ekucei_c][ekucei_c link] | [`raw::ekucei`] | Update an integer column entry in a specified EK record
[ekuef_c][ekuef_c link] | [`raw::ekuef`] | Unload an EK file, making its contents inaccessible to the EK reader routines, and clearing
[el2cgv_c][el2cgv_c link] | [`raw::el2cgv`] | Ellipse to centre and generating vectors
[elemc_c][elemc_c link] | [`raw::elemc`] | Element of a character set
[elemd_c][elemd_c link] | [`raw::elemd`] | Element of a d.p. set
[elemi_c][elemi_c link] | [`raw::elemi`] | Element of an integer set
[eqncpv_c][eqncpv_c link] | [`raw::eqncpv`] | Compute the state (position and velocity) of an object whose trajectory is described via equ
[eqstr_c][eqstr_c link] | [`raw::eqstr`] | Are two strings equivalent?
[erract_c][erract_c link] | [`errors::erract`] | Get/Set default error action
[errch_c][errch_c link] | [`raw::errch`] | Substitute a character string for the first occurrence of a marker in the current long error
[errdev_c][errdev_c link] | [`errors::errdev`] | Get/Set error output device name
[errdp_c][errdp_c link] | [`raw::errdp`] | Insert a d.p. number into an error message
[errint_c][errint_c link] | [`raw::errint`] | Insert an integer into an error message
[errprt_c][errprt_c link] | [`errors::errprt`] | Get/Set error output items
[esrchc_c][esrchc_c link] | [`raw::esrchc`] | Equivalence search a character array
[et2lst_c][et2lst_c link] | [`neat::et2lst`] | Compute the local solar time for a given ephemeris epoch `et' for an object on the surface o
[et2utc_c][et2utc_c link] | [`neat::et2utc`] | Ephemeris time to UTC
[etcal_c][etcal_c link] | [`raw::etcal`] | Convert from an ephemeris epoch measured in seconds past the epoch of J2000 to a calendar string
[eul2m_c][eul2m_c link] | [`raw::eul2m`] | Euler angles to matrix
[eul2xf_c][eul2xf_c link] | [`raw::eul2xf`] | Euler angles and derivatives to state transformation
[evsgp4_c][evsgp4_c link] | [`raw::evsgp4`] | Evaluate two-line element set with SGP4
[exists_c][exists_c link] | [`raw::exists`] | Determine whether a file exists
[expool_c][expool_c link] | [`raw::expool`] | Confirm the existence of a pool kernel variable
[failed_c][failed_c link] | [`errors::failed`] | Error status indicator
[filld_c][filld_c link] | [`raw::filld`] | Fill a double precision array with a specified value
[filli_c][filli_c link] | [`raw::filli`] | Fill an integer array with a specified value
[fovray_c][fovray_c link] | [`raw::fovray`] | Determine if a specified ray is within the field-of-view (FOV) of a specified instrument at
[fovtrg_c][fovtrg_c link] | [`raw::fovtrg`] | Determine if a specified ephemeris object is within the field-of-view (FOV) of a specified i
[frame_c][frame_c link] | [`raw::frame`] | Build a right handed orthonormal frame (x,y,z) from a 3-dimensional input vector, where the
[frinfo_c][frinfo_c link] | [`raw::frinfo`] | Retrieve the minimal attributes associated with a frame needed for converting transformation
[frmnam_c][frmnam_c link] | [`neat::frmnam`] | Frame to name translation
[ftncls_c][ftncls_c link] | [`raw::ftncls`] | Close a file designated by a Fortran-style integer logical unit
[furnsh_c][furnsh_c link] | [`raw::furnsh`] | Furnish a program with SPICE kernels
[gcpool_c][gcpool_c link] | [`neat::gcpool`] | Get character values from the kernel pool
[gdpool_c][gdpool_c link] | [`raw::gdpool`] | Get d.p. values from the kernel pool
[georec_c][georec_c link] | [`raw::georec`] | Geodetic to rectangular coordinates
[getcml_c][getcml_c link] | [`raw::getcml`] | Store the contents of argv and argc for later access
[getelm_c][getelm_c link] | [`raw::getelm`] | Parse a two-line element set
[getfat_c][getfat_c link] | [`neat::getfat`] | Determine the file architecture and file type of most SPICE kernel files
[getfov_c][getfov_c link] | [`neat::getfov`] | Get instrument FOV parameters
[getfvn_c][getfvn_c link] | [`neat::getfvn`] | Return the field-of-view (FOV) parameters for a specified instrument
[getmsg_c][getmsg_c link] | [`errors::getmsg`] | Get error message
[gfbail_c][gfbail_c link] | [`raw::gfbail`] | Indicate whether an interrupt signal (SIGINT) has been received
[gfclrh_c][gfclrh_c link] | [`raw::gfclrh`] | Clear the interrupt signal handler status, so that future calls to gfbail_c will indicate no
[gfdist_c][gfdist_c link] | [`neat::gfdist`] | Return the time window over which a specified constraint on observer-target distance is met
[gfevnt_c][gfevnt_c link] | [`raw::gfevnt`] | Determine time intervals when a specified geometric quantity satisfies a specified mathemati
[gffove_c][gffove_c link] | [`raw::gffove`] | Determine time intervals when a specified target body or ray intersects the space bounded by
[gfilum_c][gfilum_c link] | [`neat::gfilum`] | Return the time window over which a specified constraint on the observed phase, solar incide
[gfinth_c][gfinth_c link] | [`raw::gfinth`] | Respond to the interrupt signal SIGINT: save an indication that the signal has been received
[gfocce_c][gfocce_c link] | [`raw::gfocce`] | Determine time intervals when an observer sees one target occulted by another
[gfoclt_c][gfoclt_c link] | [`neat::gfoclt`] | GF, occultation search
[gfpa_c][gfpa_c link] | [`neat::gfpa`] | Determine time intervals for which a specified constraint on the phase angle between an illu
[gfposc_c][gfposc_c link] | [`neat::gfposc`] | Determine time intervals for which a coordinate of an observer-target position vector satisf
[gfrefn_c][gfrefn_c link] | [`raw::gfrefn`] | Estimate, using a bisection method, the next abscissa value at which a state change occurs
[gfrepf_c][gfrepf_c link] | [`raw::gfrepf`] | Finish a GF progress report
[gfrepi_c][gfrepi_c link] | [`raw::gfrepi`] | Initialize a search progress report
[gfrepu_c][gfrepu_c link] | [`raw::gfrepu`] | Tell the progress reporting system how far a search has progressed
[gfrfov_c][gfrfov_c link] | [`neat::gfrfov`] | Determine time intervals when a specified ray intersects the space bounded by the field-of-v
[gfrr_c][gfrr_c link] | [`neat::gfrr`] | Determine time intervals for which a specified constraint on the observer-target range rate
[gfsep_c][gfsep_c link] | [`neat::gfsep`] | Determine time intervals when the angular separation between the position vectors of two tar
[gfsntc_c][gfsntc_c link] | [`neat::gfsntc`] | Determine time intervals for which a coordinate of an surface intercept position vector sati
[gfsstp_c][gfsstp_c link] | [`raw::gfsstp`] | Set the step size to be returned by gfstep_c
[gfstep_c][gfstep_c link] | [`raw::gfstep`] | Return the time step set by the most recent call to gfsstp_c
[gfstol_c][gfstol_c link] | [`raw::gfstol`] | Override the default GF convergence value used in the high level GF routines
[gfsubc_c][gfsubc_c link] | [`neat::gfsubc`] | Determine time intervals for which a coordinate of an subpoint position vector satisfies a n
[gftfov_c][gftfov_c link] | [`neat::gftfov`] | Determine time intervals when a specified ephemeris object intersects the space bounded by t
[gfudb_c][gfudb_c link] | [`neat::gfudb`] | Perform a GF search on a user defined boolean quantity
[gfuds_c][gfuds_c link] | [`neat::gfuds`] | Perform a GF search on a user defined scalar quantity
[gipool_c][gipool_c link] | [`raw::gipool`] | Get integers from the kernel pool
[gnpool_c][gnpool_c link] | [`neat::gnpool`] | Get names of kernel pool variables
[halfpi_c][halfpi_c link] | [`raw::halfpi`] | Toolkit constant
[hrmesp_c][hrmesp_c link] | [`raw::hrmesp`] | Evaluate, at a specified point, a Hermite interpolating polynomial for a specified set of eq
[hrmint_c][hrmint_c link] | [`raw::hrmint`] | Evaluate a Hermite interpolating polynomial at a specified abscissa value
[hx2dp_c][hx2dp_c link] | [`raw::hx2dp`] | Convert a string representing a double precision number in a base 16 "scientific notation" into
[ident_c][ident_c link] | [`raw::ident`] | Return the 3x3 identity matrix
[illum_c][illum_c link] | [`raw::illum`] | Deprecated: This routine has been superseded by the CSPICE routine ilumin_c
[illumf_c][illumf_c link] | [`raw::illumf`] | Illumination angles, general source, return flags
[illumg_c][illumg_c link] | [`raw::illumg`] | Find the illumination angles (phase, incidence, and emission) at a specified surface point o
[ilumin_c][ilumin_c link] | [`raw::ilumin`] | Illumination angles
[inedpl_c][inedpl_c link] | [`raw::inedpl`] | Intersection of an ellipsoid and a plane
[inelpl_c][inelpl_c link] | [`raw::inelpl`] | Intersection of an ellipse and a plane
[inrypl_c][inrypl_c link] | [`raw::inrypl`] | Intersection of a ray and a plane
[insrtc_c][insrtc_c link] | [`raw::insrtc`] | Insert into a character set
[insrtd_c][insrtd_c link] | [`raw::insrtd`] | Insert into a d.p. set
[insrti_c][insrti_c link] | [`raw::insrti`] | Insert into an integer set
[inter_c][inter_c link] | [`raw::inter`] | Intersection of two sets
[intmax_c][intmax_c link] | [`raw::intmax`] | Return the value of the largest (positive) number representable in a SpiceInt variable
[intmin_c][intmin_c link] | [`raw::intmin`] | Return the value of the smallest (negative) number representable in a SpiceInt variable
[invert_c][invert_c link] | [`raw::invert`] | Invert a 3x3 matrix
[invort_c][invort_c link] | [`raw::invort`] | Invert nearly orthogonal matrices
[invstm_c][invstm_c link] | [`raw::invstm`] | Return the inverse of a state transformation matrix
[isordv_c][isordv_c link] | [`raw::isordv`] | Is an array an order vector?
[isrchc_c][isrchc_c link] | [`raw::isrchc`] | Search for a given value within a character string array
[isrchd_c][isrchd_c link] | [`raw::isrchd`] | Search for a given value within a double precision array
[isrchi_c][isrchi_c link] | [`raw::isrchi`] | Search for a given value within an integer array
[isrot_c][isrot_c link] | [`raw::isrot`] | Indicate whether a matrix is a rotation matrix
[iswhsp_c][iswhsp_c link] | [`raw::iswhsp`] | Return a boolean value indicating whether a string contains only white space characters
[j1900_c][j1900_c link] | [`raw::j1900`] | Toolkit constant
[j1950_c][j1950_c link] | [`raw::j1950`] | Toolkit constant
[j2000_c][j2000_c link] | [`raw::j2000`] | Toolkit constant
[j2100_c][j2100_c link] | [`raw::j2100`] | Toolkit constant
[jyear_c][jyear_c link] | [`raw::jyear`] | Toolkit constant
[kclear_c][kclear_c link] | [`raw::kclear`] | Keeper clear
[kdata_c][kdata_c link] | [`neat::kdata`] | Kernel data
[kinfo_c][kinfo_c link] | [`neat::kinfo`] | Kernel information
[kplfrm_c][kplfrm_c link] | [`raw::kplfrm`] | Return a SPICE set containing the frame IDs of all reference frames of a given class having
[ktotal_c][ktotal_c link] | [`raw::ktotal`] | Kernel totals
[kxtrct_c][kxtrct_c link] | [`raw::kxtrct`] | Locate a keyword in a string and extract the substring from the beginning of the first word
[lastnb_c][lastnb_c link] | [`raw::lastnb`] | Return the zero based index of the last non-blank character in a character string
[latcyl_c][latcyl_c link] | [`raw::latcyl`] | Latitudinal to cylindrical coordinates
[latrec_c][latrec_c link] | [`raw::latrec`] | Latitudinal to rectangular coordinates
[latsph_c][latsph_c link] | [`raw::latsph`] | Latitudinal to spherical coordinates
[latsrf_c][latsrf_c link] | [`raw::latsrf`] | Latitudinal grid to surface points
[lcase_c][lcase_c link] | [`neat::lcase`] | Convert to lower case
[ldpool_c][ldpool_c link] | [`raw::ldpool`] | Load variables from a kernel file into the pool
[lgresp_c][lgresp_c link] | [`raw::lgresp`] | Evaluate a Lagrange interpolating polynomial for a specified set of coordinate pairs whose f
[lgrind_c][lgrind_c link] | [`raw::lgrind`] | Evaluate a Lagrange interpolating polynomial, for a specified set of coordinate pairs, at a
[lgrint_c][lgrint_c link] | [`raw::lgrint`] | Evaluate a Lagrange interpolating polynomial for a specified set of coordinate pairs, at a s
[limbpt_c][limbpt_c link] | [`raw::limbpt`] | Limb points on an extended object
[lmpool_c][lmpool_c link] | [`raw::lmpool`] | Load the variables contained in an internal buffer into the kernel pool
[lparse_c][lparse_c link] | [`neat::lparse`] | Parse a list of items on a single delimiter
[lparsm_c][lparsm_c link] | [`neat::lparsm`] | Parse a list of items on multiple delimiters
[lparss_c][lparss_c link] | [`raw::lparss`] | Parse a list of items separated by multiple delimiters, placing the resulting items into a s
[lspcn_c][lspcn_c link] | [`raw::lspcn`] | Longitude of the sun, planetocentric
[lstlec_c][lstlec_c link] | [`raw::lstlec`] | Last character element less than or equal
[lstled_c][lstled_c link] | [`raw::lstled`] | Last d.p. element less than or equal
[lstlei_c][lstlei_c link] | [`raw::lstlei`] | Last integer element less than or equal
[lstltc_c][lstltc_c link] | [`raw::lstltc`] | Last character element less than
[lstltd_c][lstltd_c link] | [`raw::lstltd`] | Last d.p. element less than
[lstlti_c][lstlti_c link] | [`raw::lstlti`] | Last integer element less than
[ltime_c][ltime_c link] | [`raw::ltime`] | Compute the transmission (or reception) time of a signal at a specified target, given the re
[lx4dec_c][lx4dec_c link] | [`raw::lx4dec`] | Scan a string from a specified starting position for the end of a decimal number
[lx4num_c][lx4num_c link] | [`raw::lx4num`] | Scan a string from a specified starting position for the end of a number
[lx4sgn_c][lx4sgn_c link] | [`raw::lx4sgn`] | Scan a string from a specified starting position for the end of a signed integer
[lx4uns_c][lx4uns_c link] | [`raw::lx4uns`] | Scan a string from a specified starting position for the end of an unsigned integer
[lxqstr_c][lxqstr_c link] | [`raw::lxqstr`] | Scan (lex) a quoted string
[m2eul_c][m2eul_c link] | [`raw::m2eul`] | Matrix to Euler angles
[m2q_c][m2q_c link] | [`raw::m2q`] | Matrix to quaternion
[matchi_c][matchi_c link] | [`raw::matchi`] | Match a string against a template, case insensitive
[matchw_c][matchw_c link] | [`raw::matchw`] | Match a string against a template
[maxd_c][maxd_c link] | [`raw::maxd`] | Find the maximum of a set of double precision values
[maxi_c][maxi_c link] | [`raw::maxi`] | Find the maximum of a set of integers
[mequ_c][mequ_c link] | [`raw::mequ`] | Matrix equal to another, 3x3
[mequg_c][mequg_c link] | [`raw::mequg`] | Matrix equal to another, n dimensions
[mind_c][mind_c link] | [`raw::mind`] | Find the minimum of a set of double precision values
[mini_c][mini_c link] | [`raw::mini`] | Find the minimum of a set of integers
[moved_c][moved_c link] | [`raw::moved`] | Move a d.p. array to another
[mtxm_c][mtxm_c link] | [`raw::mtxm`] | Matrix transpose times matrix, 3x3
[mtxmg_c][mtxmg_c link] | [`raw::mtxmg`] | Matrix transpose times matrix, n dimensions
[mtxv_c][mtxv_c link] | [`raw::mtxv`] | Matrix transpose times vector, 3x3
[mtxvg_c][mtxvg_c link] | [`raw::mtxvg`] | Matrix transpose times vector, n dimensions
[mxm_c][mxm_c link] | [`raw::mxm`] | Matrix times matrix, 3x3
[mxmg_c][mxmg_c link] | [`raw::mxmg`] | Matrix times matrix, n dimensions
[mxmt_c][mxmt_c link] | [`raw::mxmt`] | Matrix times matrix transpose, 3x3
[mxmtg_c][mxmtg_c link] | [`raw::mxmtg`] | Matrix times matrix transpose, n dimensions
[mxv_c][mxv_c link] | [`raw::mxv`] | Matrix times vector, 3x3
[mxvg_c][mxvg_c link] | [`raw::mxvg`] | Matrix times vector, n dimensions
[namfrm_c][namfrm_c link] | [`raw::namfrm`] | Name to frame translation
[ncpos_c][ncpos_c link] | [`raw::ncpos`] | Find the first occurrence in a string of a character NOT belonging to a collection of charac
[ncposr_c][ncposr_c link] | [`raw::ncposr`] | Find the first occurrence in a string of a character NOT belonging to a collection of charac
[nearpt_c][nearpt_c link] | [`raw::nearpt`] | Nearest point on an ellipsoid
[nextwd_c][nextwd_c link] | [`neat::nextwd`] | Next word in a string
[npedln_c][npedln_c link] | [`raw::npedln`] | Nearest point on an ellipsoid to a line
[npelpt_c][npelpt_c link] | [`raw::npelpt`] | Nearest point on an ellipse to a point
[nplnpt_c][nplnpt_c link] | [`raw::nplnpt`] | Nearest point on a line to a point
[nthwd_c][nthwd_c link] | [`raw::nthwd`] | Return the nth word in a character string, and its location in the string
[nvc2pl_c][nvc2pl_c link] | [`raw::nvc2pl`] | Normal vector and constant to plane
[nvp2pl_c][nvp2pl_c link] | [`raw::nvp2pl`] | Normal vector and point to plane
[occult_c][occult_c link] | [`raw::occult`] | Find occultation type at time
[ordc_c][ordc_c link] | [`raw::ordc`] | Return the ordinal position of a given item in a set
[ordd_c][ordd_c link] | [`raw::ordd`] | Return the ordinal position of a given item in a set
[orderc_c][orderc_c link] | [`raw::orderc`] | Order of a character array
[orderd_c][orderd_c link] | [`raw::orderd`] | Order of a d.p. array
[orderi_c][orderi_c link] | [`raw::orderi`] | Order of an integer array
[ordi_c][ordi_c link] | [`raw::ordi`] | Return the ordinal position of a given item in a set
[oscelt_c][oscelt_c link] | [`raw::oscelt`] | Determine conic elements from state
[oscltx_c][oscltx_c link] | [`raw::oscltx`] | Extended osculating elements from state
[pckcls_c][pckcls_c link] | [`raw::pckcls`] | Close an open PCK file
[pckcov_c][pckcov_c link] | [`neat::pckcov`] | PCK, coverage
[pckfrm_c][pckfrm_c link] | [`neat::pckfrm`] | PCK, reference frame class ID set
[pcklof_c][pcklof_c link] | [`raw::pcklof`] | Load a binary PCK file for use by the readers
[pckopn_c][pckopn_c link] | [`raw::pckopn`] | Create a new PCK file, returning the handle of the opened file
[pckuof_c][pckuof_c link] | [`raw::pckuof`] | Unload a binary PCK file so that it will no longer be searched by the readers
[pckw02_c][pckw02_c link] | [`raw::pckw02`] | Write a type 2 segment to a PCK binary file given the file handle, frame class ID, base fram
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
[pltar_c][pltar_c link] | [`raw::pltar`] | Compute the total area of a collection of triangular plates
[pltexp_c][pltexp_c link] | [`raw::pltexp`] | Expand a triangular plate by a specified amount
[pltnp_c][pltnp_c link] | [`raw::pltnp`] | Find the nearest point on a triangular plate to a given point
[pltnrm_c][pltnrm_c link] | [`raw::pltnrm`] | Compute an outward normal vector of a triangular plate
[pltvol_c][pltvol_c link] | [`raw::pltvol`] | Compute the volume of a three-dimensional region bounded by a collection of triangular plate
[polyds_c][polyds_c link] | [`raw::polyds`] | Compute the value of a polynomial and its first `nderiv' derivatives at the value `t'
[pos_c][pos_c link] | [`raw::pos`] | Find the first occurrence in a string of a substring, starting at a specified location, searchin
[posr_c][posr_c link] | [`raw::posr`] | Find the first occurrence in a string of a substring, starting at a specified location, searchin
[prompt_c][prompt_c link] | [`raw::prompt`] | Prompt a user for keyboard input
[prop2b_c][prop2b_c link] | [`raw::prop2b`] | Propagate a two-body solution
[prsdp_c][prsdp_c link] | [`raw::prsdp`] | Parse a string as a double precision number, encapsulating error handling
[prsint_c][prsint_c link] | [`raw::prsint`] | Parse a string as an integer, encapsulating error handling
[psv2pl_c][psv2pl_c link] | [`raw::psv2pl`] | Point and spanning vectors to plane
[putcml_c][putcml_c link] | [`raw::putcml`] | Store the contents of argv and argc for later access
[pxform_c][pxform_c link] | [`raw::pxform`] | Position transformation matrix
[pxfrm2_c][pxfrm2_c link] | [`raw::pxfrm2`] | Position transform matrix, different epochs
[q2m_c][q2m_c link] | [`raw::q2m`] | Quaternion to matrix
[qcktrc_c][qcktrc_c link] | [`errors::qcktrc`] | Get quick traceback
[qderiv_c][qderiv_c link] | [`raw::qderiv`] | Estimate the derivative of a function by finding the derivative of a quadratic approximating
[qdq2av_c][qdq2av_c link] | [`raw::qdq2av`] | Derive angular velocity from a unit quaternion and its derivative with respect to time
[qxq_c][qxq_c link] | [`raw::qxq`] | Quaternion times quaternion
[radrec_c][radrec_c link] | [`raw::radrec`] | RA and DEC to rectangular coordinates
[rav2xf_c][rav2xf_c link] | [`raw::rav2xf`] | Rotation and angular velocity to transform
[raxisa_c][raxisa_c link] | [`raw::raxisa`] | Rotation axis of a matrix
[rdtext_c][rdtext_c link] | [`raw::rdtext`] | Read the next line of text from a text file
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
[reordc_c][reordc_c link] | [`raw::reordc`] | Reorder a character array
[reordd_c][reordd_c link] | [`raw::reordd`] | Reorder a d.p. array
[reordi_c][reordi_c link] | [`raw::reordi`] | Reorder an integer array
[reordl_c][reordl_c link] | [`raw::reordl`] | Reorder a logical array
[repmc_c][repmc_c link] | [`neat::repmc`] | Replace a marker with a string
[repmct_c][repmct_c link] | [`neat::repmct`] | Replace a marker with the text representation of a cardinal number
[repmd_c][repmd_c link] | [`neat::repmd`] | Replace a marker with a d.p. number
[repmf_c][repmf_c link] | [`raw::repmf`] | Replace a marker with a formatted d.p. number
[repmi_c][repmi_c link] | [`neat::repmi`] | Replace a marker with an integer
[repml_c][repml_c link] | [`raw::repml`] | Replace a marker with a boolean
[repmot_c][repmot_c link] | [`raw::repmot`] | Replace a marker with an ordinal
[reset_c][reset_c link] | [`errors::reset`] | Reset error status
[return_c][return_c link] | [`raw::return_c`] | Immediate return indicator
[rotate_c][rotate_c link] | [`raw::rotate`] | Generate a rotation matrix
[rotmat_c][rotmat_c link] | [`raw::rotmat`] | Rotate a matrix
[rotvec_c][rotvec_c link] | [`raw::rotvec`] | Transform a vector to a new coordinate system rotated by `angle' radians about axis `iaxis'
[rpd_c][rpd_c link] | [`raw::rpd`] | Radians per degree
[rquad_c][rquad_c link] | [`raw::rquad`] | Find the roots of a quadratic equation
[saelgv_c][saelgv_c link] | [`raw::saelgv`] | Semi-axes of an ellipse from generating vectors
[scard_c][scard_c link] | [`raw::scard`] | Set the cardinality of a cell
[scdecd_c][scdecd_c link] | [`neat::scdecd`] | Decode spacecraft clock
[sce2c_c][sce2c_c link] | [`raw::sce2c`] | ET to continuous SCLK ticks
[sce2s_c][sce2s_c link] | [`neat::sce2s`] | ET to SCLK string
[sce2t_c][sce2t_c link] | [`raw::sce2t`] | Convert ephemeris seconds past J2000 (ET) to integral encoded spacecraft clock (`ticks')
[scencd_c][scencd_c link] | [`raw::scencd`] | Encode spacecraft clock
[scfmt_c][scfmt_c link] | [`raw::scfmt`] | Convert encoded spacecraft clock ticks to character clock format
[scpart_c][scpart_c link] | [`raw::scpart`] | Get spacecraft clock partition information from a spacecraft clock kernel file
[scs2e_c][scs2e_c link] | [`raw::scs2e`] | SCLK string to ET
[sct2e_c][sct2e_c link] | [`raw::sct2e`] | SCLK ticks to ET
[sctiks_c][sctiks_c link] | [`raw::sctiks`] | Convert a spacecraft clock format string to number of "ticks"
[sdiff_c][sdiff_c link] | [`raw::sdiff`] | Take the symmetric difference of two sets of any data type to form a third set
[set_c][set_c link] | [`raw::set`] | Compare two sets of any data type, as indicated by a relational operator
[setmsg_c][setmsg_c link] | [`raw::setmsg`] | Set the value of the current long error message
[shellc_c][shellc_c link] | [`raw::shellc`] | Shell sort a character array
[shelld_c][shelld_c link] | [`raw::shelld`] | Shell sort a d.p. array
[shelli_c][shelli_c link] | [`raw::shelli`] | Shell sort an integer array
[sigerr_c][sigerr_c link] | [`raw::sigerr`] | Signal an error condition
[sincpt_c][sincpt_c link] | [`raw::sincpt`] | Surface intercept
[size_c][size_c link] | [`raw::size`] | Size of a cell
[spd_c][spd_c link] | [`raw::spd`] | Seconds per day
[sphcyl_c][sphcyl_c link] | [`raw::sphcyl`] | Spherical to cylindrical coordinates
[sphlat_c][sphlat_c link] | [`raw::sphlat`] | Spherical to latitudinal coordinates
[sphrec_c][sphrec_c link] | [`raw::sphrec`] | Spherical to rectangular coordinates
[spk14a_c][spk14a_c link] | [`raw::spk14a`] | Add data to a type 14 SPK segment associated with `handle'
[spk14b_c][spk14b_c link] | [`raw::spk14b`] | Begin a type 14 SPK segment in the SPK file associated with `handle'
[spk14e_c][spk14e_c link] | [`raw::spk14e`] | End the type 14 SPK segment currently being written to the SPK file associated with `handle'
[spkacs_c][spkacs_c link] | [`raw::spkacs`] | Return the state (position and velocity) of a target body relative to an observer, optionall
[spkapo_c][spkapo_c link] | [`raw::spkapo`] | Return the position of a target body relative to an observer, optionally corrected for light
[spkapp_c][spkapp_c link] | [`raw::spkapp`] | Deprecated: This routine has been superseded by the CSPICE routine spkaps_c
[spkaps_c][spkaps_c link] | [`raw::spkaps`] | Return the state (position and velocity) of a target body relative to an observer specified
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
[spkgps_c][spkgps_c link] | [`raw::spkgps`] | Compute the geometric position of a target body relative to an observing body
[spklef_c][spklef_c link] | [`raw::spklef`] | Load an ephemeris file for use by the readers
[spkltc_c][spkltc_c link] | [`raw::spkltc`] | Return the state (position and velocity) of a target body relative to an observer, optionall
[spkobj_c][spkobj_c link] | [`neat::spkobj`] | SPK, objects
[spkopa_c][spkopa_c link] | [`raw::spkopa`] | SPK, open for addition
[spkopn_c][spkopn_c link] | [`raw::spkopn`] | SPK, open new file
[spkpds_c][spkpds_c link] | [`raw::spkpds`] | Perform routine error checks and if all check pass, pack the descriptor for an SPK segment
[spkpos_c][spkpos_c link] | [`raw::spkpos`] | S/P Kernel, position
[spkpvn_c][spkpvn_c link] | [`raw::spkpvn`] | Return, for a specified SPK segment and time, the state (position and velocity) of the segme
[spksfs_c][spksfs_c link] | [`raw::spksfs`] | Search through loaded SPK files to find the highest-priority segment applicable to the body
[spkssb_c][spkssb_c link] | [`raw::spkssb`] | Return the state (position and velocity) of a target body relative to the solar system baryc
[spksub_c][spksub_c link] | [`raw::spksub`] | Extract a subset of the data in an SPK segment into a separate segment
[spkuds_c][spkuds_c link] | [`raw::spkuds`] | Unpack the contents of an SPK segment descriptor
[spkuef_c][spkuef_c link] | [`raw::spkuef`] | Unload an ephemeris file so that it will no longer be searched by the readers
[spkw02_c][spkw02_c link] | [`raw::spkw02`] | Write a type 2 segment to an SPK file
[spkw03_c][spkw03_c link] | [`raw::spkw03`] | Write a type 3 segment to an SPK file
[spkw05_c][spkw05_c link] | [`raw::spkw05`] | Write an SPK segment of type 5 given a time-ordered set of discrete states and epochs, and t
[spkw08_c][spkw08_c link] | [`raw::spkw08`] | Write a type 8 segment to an SPK file
[spkw09_c][spkw09_c link] | [`raw::spkw09`] | Write SPK segment, type 9
[spkw10_c][spkw10_c link] | [`raw::spkw10`] | Write an SPK type 10 segment to the file specified by the input `handle'
[spkw12_c][spkw12_c link] | [`raw::spkw12`] | Write a type 12 segment to an SPK file
[spkw13_c][spkw13_c link] | [`raw::spkw13`] | Write a type 13 segment to an SPK file
[spkw15_c][spkw15_c link] | [`raw::spkw15`] | Write an SPK segment of type 15 given a type 15 data record
[spkw17_c][spkw17_c link] | [`raw::spkw17`] | Write an SPK segment of type 17 given a type 17 data record
[spkw18_c][spkw18_c link] | [`raw::spkw18`] | Write a type 18 segment to an SPK file
[spkw20_c][spkw20_c link] | [`raw::spkw20`] | Write a type 20 segment to an SPK file
[srfc2s_c][srfc2s_c link] | [`neat::srfc2s`] | Surface and body ID codes to surface string
[srfcss_c][srfcss_c link] | [`neat::srfcss`] | Surface ID and body string to surface string
[srfnrm_c][srfnrm_c link] | [`raw::srfnrm`] | Map surface points to outward normal vectors
[srfrec_c][srfrec_c link] | [`raw::srfrec`] | Surface to rectangular coordinates
[srfs2c_c][srfs2c_c link] | [`raw::srfs2c`] | Surface and body strings to surface ID code
[srfscc_c][srfscc_c link] | [`raw::srfscc`] | Surface string and body ID code to surface ID code
[srfxpt_c][srfxpt_c link] | [`raw::srfxpt`] | Deprecated: This routine has been superseded by the CSPICE routine sincpt_c
[ssize_c][ssize_c link] | [`raw::ssize`] | Set the size of a cell
[stelab_c][stelab_c link] | [`raw::stelab`] | Correct the apparent position of an object for stellar aberration
[stlabx_c][stlabx_c link] | [`raw::stlabx`] | Correct the position of a target for the stellar aberration effect on radiation transmitted
[stpool_c][stpool_c link] | [`neat::stpool`] | Retrieve the nth string from a kernel pool variable, where the string may be continued acros
[str2et_c][str2et_c link] | [`raw::str2et`] | String to ET
[subpnt_c][subpnt_c link] | [`raw::subpnt`] | Sub-observer point
[subpt_c][subpt_c link] | [`raw::subpt`] | Deprecated: This routine has been superseded by the CSPICE routine subpnt_c
[subslr_c][subslr_c link] | [`raw::subslr`] | Sub-solar point
[subsol_c][subsol_c link] | [`raw::subsol`] | Deprecated: This routine has been superseded by the CSPICE routine subslr_c
[sumad_c][sumad_c link] | [`raw::sumad`] | Sum of a d.p. array
[sumai_c][sumai_c link] | [`raw::sumai`] | Sum of an integer array
[surfnm_c][surfnm_c link] | [`raw::surfnm`] | Surface normal of an ellipsoid
[surfpt_c][surfpt_c link] | [`raw::surfpt`] | Surface point on an ellipsoid
[surfpv_c][surfpv_c link] | [`raw::surfpv`] | Surface point and velocity on an ellipsoid
[swpool_c][swpool_c link] | [`raw::swpool`] | Set a watch on a set of kernel pool variables
[sxform_c][sxform_c link] | [`raw::sxform`] | State transformation matrix
[szpool_c][szpool_c link] | [`raw::szpool`] | Return the kernel pool size limitations
[tangpt_c][tangpt_c link] | [`raw::tangpt`] | Compute, for a given observer, ray emanating from the observer, and target, the "tangent poi
[termpt_c][termpt_c link] | [`raw::termpt`] | Terminator points on an extended object
[timdef_c][timdef_c link] | [`neat::timdef`] | Set and retrieve the defaults associated with calendar input strings
[timout_c][timout_c link] | [`neat::timout`] | Time output
[tipbod_c][tipbod_c link] | [`raw::tipbod`] | Return a 3x3 matrix that transforms positions in inertial coordinates to positions in body-e
[tisbod_c][tisbod_c link] | [`raw::tisbod`] | Return a 6x6 matrix that transforms states in inertial coordinates to states in body-equator
[tkfram_c][tkfram_c link] | [`raw::tkfram`] | Find the position rotation matrix from a Text Kernel (TK) frame with the specified frame cla
[tkvrsn_c][tkvrsn_c link] | [`raw::tkvrsn`] | Toolkit version strings
[tparch_c][tparch_c link] | [`raw::tparch`] | Restrict the set of strings that are recognized by SPICE time parsing routines to those that hav
[tparse_c][tparse_c link] | [`neat::tparse`] | Parse a UTC time string
[tpictr_c][tpictr_c link] | [`neat::tpictr`] | Create a time format picture suitable for use by the routine timout_c from a given sample ti
[trace_c][trace_c link] | [`raw::trace`] | Trace of a 3x3 matrix
[trcdep_c][trcdep_c link] | [`raw::trcdep`] | Return the number of modules in the traceback representation
[trcnam_c][trcnam_c link] | [`neat::trcnam`] | Return the name of the module having the specified position in the trace representation
[trcoff_c][trcoff_c link] | [`raw::trcoff`] | Disable tracing
[trgsep_c][trgsep_c link] | [`raw::trgsep`] | Compute the angular separation in radians between two spherical or point objects
[tsetyr_c][tsetyr_c link] | [`raw::tsetyr`] | Set the lower bound on the 100 year range
[twopi_c][twopi_c link] | [`raw::twopi`] | Toolkit constant
[twovec_c][twovec_c link] | [`raw::twovec`] | Two vectors defining an orthonormal frame
[twovxf_c][twovxf_c link] | [`raw::twovxf`] | Find the state transformation from a base frame to the right-handed frame defined by two sta
[tyear_c][tyear_c link] | [`raw::tyear`] | Toolkit constant
[ucase_c][ucase_c link] | [`neat::ucase`] | Convert to upper case
[ucrss_c][ucrss_c link] | [`raw::ucrss`] | Compute the normalized cross product of two 3-vectors
[uddc_c][uddc_c link] | [`raw::uddc`] | Return SPICETRUE if the derivative of the callback function `udfunc' at a given abscissa val
[uddf_c][uddf_c link] | [`raw::uddf`] | Calculate the first derivative of a caller-specified scalar function using a three-point est
[udf_c][udf_c link] | [`raw::udf`] | Serve as a dummy function for GF routines expecting an `udfuns' argument
[union_c][union_c link] | [`raw::union`] | Union of two sets
[unitim_c][unitim_c link] | [`raw::unitim`] | Uniform time scale transformation
[unload_c][unload_c link] | [`raw::unload`] | Unload a kernel
[unorm_c][unorm_c link] | [`raw::unorm`] | Unit vector and norm, 3 dimensions
[unormg_c][unormg_c link] | [`raw::unormg`] | Unit vector and norm, n dimensions
[utc2et_c][utc2et_c link] | [`raw::utc2et`] | UTC to ephemeris time
[vadd_c][vadd_c link] | [`raw::vadd`] | Vector addition, 3 dimensions
[vaddg_c][vaddg_c link] | [`raw::vaddg`] | Vector addition, n dimensions
[valid_c][valid_c link] | [`raw::valid`] | Validate a set
[vcrss_c][vcrss_c link] | [`raw::vcrss`] | Vector cross product, 3 dimensions
[vdist_c][vdist_c link] | [`raw::vdist`] | Vector distance
[vdistg_c][vdistg_c link] | [`raw::vdistg`] | Vector distance, n dimensions
[vdot_c][vdot_c link] | [`raw::vdot`] | Vector dot product, 3 dimensions
[vdotg_c][vdotg_c link] | [`raw::vdotg`] | Vector dot product, n dimensions
[vequ_c][vequ_c link] | [`raw::vequ`] | Vector equality, 3 dimensions
[vequg_c][vequg_c link] | [`raw::vequg`] | Vector equality, n dimensions
[vhat_c][vhat_c link] | [`raw::vhat`] | Unit vector along a vector, 3 dimensions
[vhatg_c][vhatg_c link] | [`raw::vhatg`] | Unit vector along a vector, n dimensions
[vlcom3_c][vlcom3_c link] | [`raw::vlcom3`] | Linear combination of three vectors
[vlcom_c][vlcom_c link] | [`raw::vlcom`] | Linear combination of two vectors
[vlcomg_c][vlcomg_c link] | [`raw::vlcomg`] | Linear combination of two vectors, n dimensions
[vminug_c][vminug_c link] | [`raw::vminug`] | Negate a vector, n dimensions
[vminus_c][vminus_c link] | [`raw::vminus`] | Negate a vector, 3 dimensions
[vnorm_c][vnorm_c link] | [`raw::vnorm`] | Vector norm, 3 dimensions
[vnormg_c][vnormg_c link] | [`raw::vnormg`] | Vector norm, n dimensions
[vpack_c][vpack_c link] | [`raw::vpack`] | Pack three scalars into a vector
[vperp_c][vperp_c link] | [`raw::vperp`] | Perpendicular component of a vector
[vprjp_c][vprjp_c link] | [`raw::vprjp`] | Project a vector onto a plane
[vprjpi_c][vprjpi_c link] | [`raw::vprjpi`] | Invert an orthogonal projection
[vproj_c][vproj_c link] | [`raw::vproj`] | Projection of a vector onto another
[vprojg_c][vprojg_c link] | [`raw::vprojg`] | Projection of a vector, n dimensions
[vrel_c][vrel_c link] | [`raw::vrel`] | Vector relative difference, 3 dimensions
[vrelg_c][vrelg_c link] | [`raw::vrelg`] | Vector relative difference, n dimensions
[vrotv_c][vrotv_c link] | [`raw::vrotv`] | Rotate a vector about an axis
[vscl_c][vscl_c link] | [`raw::vscl`] | Vector scaling, 3 dimensions
[vsclg_c][vsclg_c link] | [`raw::vsclg`] | Vector scaling, n dimensions
[vsep_c][vsep_c link] | [`raw::vsep`] | Angular separation of vectors, 3 dimensions
[vsepg_c][vsepg_c link] | [`raw::vsepg`] | Angular separation of vectors, n dimensions
[vsub_c][vsub_c link] | [`raw::vsub`] | Vector subtraction, 3 dimensions
[vsubg_c][vsubg_c link] | [`raw::vsubg`] | Vector subtraction, n dimensions
[vtmv_c][vtmv_c link] | [`raw::vtmv`] | Vector transpose times matrix times vector
[vtmvg_c][vtmvg_c link] | [`raw::vtmvg`] | Vector transpose times matrix times vector, n dimensions
[vupack_c][vupack_c link] | [`raw::vupack`] | Unpack a vector into three scalars
[vzero_c][vzero_c link] | [`raw::vzero`] | Is a vector the zero vector?
[vzerog_c][vzerog_c link] | [`raw::vzerog`] | Is a vector the zero vector, n dimensions
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
[xposeg_c][xposeg_c link] | [`raw::xposeg`] | Transpose a matrix, general

[appndc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/appndc_c.html
[appndd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/appndd_c.html
[appndi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/appndi_c.html
[axisar_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/axisar_c.html
[azlcpo_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/azlcpo_c.html
[azlrec_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/azlrec_c.html
[b1900_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/b1900_c.html
[b1950_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/b1950_c.html
[badkpv_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/badkpv_c.html
[bltfrm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bltfrm_c.html
[bodc2n_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bodc2n_c.html
[bodc2s_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bodc2s_c.html
[boddef_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/boddef_c.html
[bodfnd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bodfnd_c.html
[bodn2c_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bodn2c_c.html
[bods2c_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bods2c_c.html
[bodvar_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bodvar_c.html
[bodvcd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bodvcd_c.html
[bodvrd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bodvrd_c.html
[brcktd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/brcktd_c.html
[brckti_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/brckti_c.html
[bschoc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bschoc_c.html
[bschoi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bschoi_c.html
[bsrchc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bsrchc_c.html
[bsrchd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bsrchd_c.html
[bsrchi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/bsrchi_c.html
[card_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/card_c.html
[ccifrm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ccifrm_c.html
[cgv2el_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/cgv2el_c.html
[chbder_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/chbder_c.html
[chbigr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/chbigr_c.html
[chbint_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/chbint_c.html
[chbval_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/chbval_c.html
[chkin_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/chkin_c.html
[chkout_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/chkout_c.html
[cidfrm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/cidfrm_c.html
[ckcls_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckcls_c.html
[ckcov_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckcov_c.html
[ckfrot_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckfrot_c.html
[ckfxfm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckfxfm_c.html
[ckgp_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckgp_c.html
[ckgpav_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckgpav_c.html
[ckgr02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckgr02_c.html
[ckgr03_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckgr03_c.html
[cklpf_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/cklpf_c.html
[ckmeta_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckmeta_c.html
[cknr02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/cknr02_c.html
[cknr03_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/cknr03_c.html
[ckobj_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckobj_c.html
[ckopn_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckopn_c.html
[ckupf_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckupf_c.html
[ckw01_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckw01_c.html
[ckw02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckw02_c.html
[ckw03_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckw03_c.html
[ckw05_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ckw05_c.html
[clearc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/clearc_c.html
[cleard_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/cleard_c.html
[cleari_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/cleari_c.html
[clight_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/clight_c.html
[clpool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/clpool_c.html
[cmprss_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/cmprss_c.html
[cnmfrm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/cnmfrm_c.html
[conics_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/conics_c.html
[convrt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/convrt_c.html
[copy_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/copy_c.html
[cpos_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/cpos_c.html
[cposr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/cposr_c.html
[cvpool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/cvpool_c.html
[cyllat_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/cyllat_c.html
[cylrec_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/cylrec_c.html
[cylsph_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/cylsph_c.html
[dafac_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dafac_c.html
[dafbbs_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dafbbs_c.html
[dafbfs_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dafbfs_c.html
[dafcls_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dafcls_c.html
[dafcs_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dafcs_c.html
[dafdc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dafdc_c.html
[dafec_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dafec_c.html
[daffna_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/daffna_c.html
[daffpa_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/daffpa_c.html
[dafgda_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dafgda_c.html
[dafgh_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dafgh_c.html
[dafgn_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dafgn_c.html
[dafgs_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dafgs_c.html
[dafgsr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dafgsr_c.html
[dafhsf_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dafhsf_c.html
[dafopr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dafopr_c.html
[dafopw_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dafopw_c.html
[dafps_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dafps_c.html
[dafrda_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dafrda_c.html
[dafrfr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dafrfr_c.html
[dafrs_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dafrs_c.html
[dafus_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dafus_c.html
[dasac_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dasac_c.html
[dasadc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dasadc_c.html
[dasadd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dasadd_c.html
[dasadi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dasadi_c.html
[dascls_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dascls_c.html
[dasdc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dasdc_c.html
[dasec_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dasec_c.html
[dashfn_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dashfn_c.html
[dashfs_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dashfs_c.html
[daslla_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/daslla_c.html
[dasllc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dasllc_c.html
[dasonw_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dasonw_c.html
[dasopr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dasopr_c.html
[dasops_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dasops_c.html
[dasopw_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dasopw_c.html
[dasrdc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dasrdc_c.html
[dasrdd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dasrdd_c.html
[dasrdi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dasrdi_c.html
[dasrfr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dasrfr_c.html
[dasudc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dasudc_c.html
[dasudd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dasudd_c.html
[dasudi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dasudi_c.html
[daswbr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/daswbr_c.html
[dazldr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dazldr_c.html
[dcyldr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dcyldr_c.html
[deltet_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/deltet_c.html
[det_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/det_c.html
[dgeodr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dgeodr_c.html
[diags2_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/diags2_c.html
[diff_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/diff_c.html
[dlabbs_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dlabbs_c.html
[dlabfs_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dlabfs_c.html
[dlabns_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dlabns_c.html
[dlaens_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dlaens_c.html
[dlafns_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dlafns_c.html
[dlafps_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dlafps_c.html
[dlaopn_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dlaopn_c.html
[dlatdr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dlatdr_c.html
[dnearp_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dnearp_c.html
[dp2hx_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dp2hx_c.html
[dpgrdr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dpgrdr_c.html
[dpmax_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dpmax_c.html
[dpmin_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dpmin_c.html
[dpr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dpr_c.html
[drdazl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/drdazl_c.html
[drdcyl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/drdcyl_c.html
[drdgeo_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/drdgeo_c.html
[drdlat_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/drdlat_c.html
[drdpgr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/drdpgr_c.html
[drdsph_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/drdsph_c.html
[dskb02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskb02_c.html
[dskcls_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskcls_c.html
[dskd02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskd02_c.html
[dskgd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskgd_c.html
[dskgtl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskgtl_c.html
[dski02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dski02_c.html
[dskmi2_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskmi2_c.html
[dskn02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskn02_c.html
[dskobj_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskobj_c.html
[dskopn_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskopn_c.html
[dskp02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskp02_c.html
[dskrb2_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dskrb2_c.html
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
[ducrss_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ducrss_c.html
[dvcrss_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dvcrss_c.html
[dvdot_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dvdot_c.html
[dvhat_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dvhat_c.html
[dvnorm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dvnorm_c.html
[dvpool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dvpool_c.html
[dvsep_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/dvsep_c.html
[edlimb_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/edlimb_c.html
[ednmpt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ednmpt_c.html
[edpnt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/edpnt_c.html
[edterm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/edterm_c.html
[ekacec_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekacec_c.html
[ekaced_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekaced_c.html
[ekacei_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekacei_c.html
[ekaclc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekaclc_c.html
[ekacld_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekacld_c.html
[ekacli_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekacli_c.html
[ekappr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekappr_c.html
[ekbseg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekbseg_c.html
[ekccnt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekccnt_c.html
[ekcii_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekcii_c.html
[ekcls_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekcls_c.html
[ekdelr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekdelr_c.html
[ekffld_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekffld_c.html
[ekfind_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekfind_c.html
[ekgc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekgc_c.html
[ekgd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekgd_c.html
[ekgi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekgi_c.html
[ekifld_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekifld_c.html
[ekinsr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekinsr_c.html
[eklef_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/eklef_c.html
[eknelt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/eknelt_c.html
[eknseg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/eknseg_c.html
[ekntab_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekntab_c.html
[ekopn_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekopn_c.html
[ekopr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekopr_c.html
[ekops_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekops_c.html
[ekopw_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekopw_c.html
[ekpsel_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekpsel_c.html
[ekrcec_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekrcec_c.html
[ekrced_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekrced_c.html
[ekrcei_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekrcei_c.html
[ekssum_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekssum_c.html
[ektnam_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ektnam_c.html
[ekucec_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekucec_c.html
[ekuced_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekuced_c.html
[ekucei_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekucei_c.html
[ekuef_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ekuef_c.html
[el2cgv_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/el2cgv_c.html
[elemc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/elemc_c.html
[elemd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/elemd_c.html
[elemi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/elemi_c.html
[eqncpv_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/eqncpv_c.html
[eqstr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/eqstr_c.html
[erract_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/erract_c.html
[errch_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/errch_c.html
[errdev_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/errdev_c.html
[errdp_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/errdp_c.html
[errint_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/errint_c.html
[errprt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/errprt_c.html
[esrchc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/esrchc_c.html
[et2lst_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/et2lst_c.html
[et2utc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/et2utc_c.html
[etcal_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/etcal_c.html
[eul2m_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/eul2m_c.html
[eul2xf_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/eul2xf_c.html
[evsgp4_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/evsgp4_c.html
[exists_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/exists_c.html
[expool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/expool_c.html
[failed_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/failed_c.html
[filld_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/filld_c.html
[filli_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/filli_c.html
[fovray_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/fovray_c.html
[fovtrg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/fovtrg_c.html
[frame_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/frame_c.html
[frinfo_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/frinfo_c.html
[frmnam_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/frmnam_c.html
[ftncls_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ftncls_c.html
[furnsh_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/furnsh_c.html
[gcpool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gcpool_c.html
[gdpool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gdpool_c.html
[georec_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/georec_c.html
[getcml_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/getcml_c.html
[getelm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/getelm_c.html
[getfat_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/getfat_c.html
[getfov_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/getfov_c.html
[getfvn_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/getfvn_c.html
[getmsg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/getmsg_c.html
[gfbail_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gfbail_c.html
[gfclrh_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gfclrh_c.html
[gfdist_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gfdist_c.html
[gfevnt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gfevnt_c.html
[gffove_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gffove_c.html
[gfilum_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gfilum_c.html
[gfinth_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gfinth_c.html
[gfocce_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gfocce_c.html
[gfoclt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gfoclt_c.html
[gfpa_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gfpa_c.html
[gfposc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gfposc_c.html
[gfrefn_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gfrefn_c.html
[gfrepf_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gfrepf_c.html
[gfrepi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gfrepi_c.html
[gfrepu_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gfrepu_c.html
[gfrfov_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gfrfov_c.html
[gfrr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gfrr_c.html
[gfsep_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gfsep_c.html
[gfsntc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gfsntc_c.html
[gfsstp_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gfsstp_c.html
[gfstep_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gfstep_c.html
[gfstol_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gfstol_c.html
[gfsubc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gfsubc_c.html
[gftfov_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gftfov_c.html
[gfudb_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gfudb_c.html
[gfuds_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gfuds_c.html
[gipool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gipool_c.html
[gnpool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gnpool_c.html
[halfpi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/halfpi_c.html
[hrmesp_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/hrmesp_c.html
[hrmint_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/hrmint_c.html
[hx2dp_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/hx2dp_c.html
[ident_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ident_c.html
[illum_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/illum_c.html
[illumf_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/illumf_c.html
[illumg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/illumg_c.html
[ilumin_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ilumin_c.html
[inedpl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/inedpl_c.html
[inelpl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/inelpl_c.html
[inrypl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/inrypl_c.html
[insrtc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/insrtc_c.html
[insrtd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/insrtd_c.html
[insrti_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/insrti_c.html
[inter_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/inter_c.html
[intmax_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/intmax_c.html
[intmin_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/intmin_c.html
[invert_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/invert_c.html
[invort_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/invort_c.html
[invstm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/invstm_c.html
[isordv_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/isordv_c.html
[isrchc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/isrchc_c.html
[isrchd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/isrchd_c.html
[isrchi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/isrchi_c.html
[isrot_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/isrot_c.html
[iswhsp_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/iswhsp_c.html
[j1900_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/j1900_c.html
[j1950_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/j1950_c.html
[j2000_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/j2000_c.html
[j2100_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/j2100_c.html
[jyear_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/jyear_c.html
[kclear_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/kclear_c.html
[kdata_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/kdata_c.html
[kinfo_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/kinfo_c.html
[kplfrm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/kplfrm_c.html
[ktotal_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ktotal_c.html
[kxtrct_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/kxtrct_c.html
[lastnb_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/lastnb_c.html
[latcyl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/latcyl_c.html
[latrec_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/latrec_c.html
[latsph_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/latsph_c.html
[latsrf_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/latsrf_c.html
[lcase_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/lcase_c.html
[ldpool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ldpool_c.html
[lgresp_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/lgresp_c.html
[lgrind_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/lgrind_c.html
[lgrint_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/lgrint_c.html
[limbpt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/limbpt_c.html
[lmpool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/lmpool_c.html
[lparse_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/lparse_c.html
[lparsm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/lparsm_c.html
[lparss_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/lparss_c.html
[lspcn_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/lspcn_c.html
[lstlec_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/lstlec_c.html
[lstled_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/lstled_c.html
[lstlei_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/lstlei_c.html
[lstltc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/lstltc_c.html
[lstltd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/lstltd_c.html
[lstlti_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/lstlti_c.html
[ltime_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ltime_c.html
[lx4dec_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/lx4dec_c.html
[lx4num_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/lx4num_c.html
[lx4sgn_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/lx4sgn_c.html
[lx4uns_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/lx4uns_c.html
[lxqstr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/lxqstr_c.html
[m2eul_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/m2eul_c.html
[m2q_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/m2q_c.html
[matchi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/matchi_c.html
[matchw_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/matchw_c.html
[maxd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/maxd_c.html
[maxi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/maxi_c.html
[mequ_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/mequ_c.html
[mequg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/mequg_c.html
[mind_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/mind_c.html
[mini_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/mini_c.html
[moved_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/moved_c.html
[mtxm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/mtxm_c.html
[mtxmg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/mtxmg_c.html
[mtxv_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/mtxv_c.html
[mtxvg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/mtxvg_c.html
[mxm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/mxm_c.html
[mxmg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/mxmg_c.html
[mxmt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/mxmt_c.html
[mxmtg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/mxmtg_c.html
[mxv_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/mxv_c.html
[mxvg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/mxvg_c.html
[namfrm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/namfrm_c.html
[ncpos_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ncpos_c.html
[ncposr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ncposr_c.html
[nearpt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/nearpt_c.html
[nextwd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/nextwd_c.html
[npedln_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/npedln_c.html
[npelpt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/npelpt_c.html
[nplnpt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/nplnpt_c.html
[nthwd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/nthwd_c.html
[nvc2pl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/nvc2pl_c.html
[nvp2pl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/nvp2pl_c.html
[occult_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/occult_c.html
[ordc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ordc_c.html
[ordd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ordd_c.html
[orderc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/orderc_c.html
[orderd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/orderd_c.html
[orderi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/orderi_c.html
[ordi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ordi_c.html
[oscelt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/oscelt_c.html
[oscltx_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/oscltx_c.html
[pckcls_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pckcls_c.html
[pckcov_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pckcov_c.html
[pckfrm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pckfrm_c.html
[pcklof_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pcklof_c.html
[pckopn_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pckopn_c.html
[pckuof_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pckuof_c.html
[pckw02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pckw02_c.html
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
[pltar_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pltar_c.html
[pltexp_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pltexp_c.html
[pltnp_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pltnp_c.html
[pltnrm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pltnrm_c.html
[pltvol_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pltvol_c.html
[polyds_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/polyds_c.html
[pos_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pos_c.html
[posr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/posr_c.html
[prompt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/prompt_c.html
[prop2b_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/prop2b_c.html
[prsdp_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/prsdp_c.html
[prsint_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/prsint_c.html
[psv2pl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/psv2pl_c.html
[putcml_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/putcml_c.html
[pxform_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pxform_c.html
[pxfrm2_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/pxfrm2_c.html
[q2m_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/q2m_c.html
[qcktrc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/qcktrc_c.html
[qderiv_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/qderiv_c.html
[qdq2av_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/qdq2av_c.html
[qxq_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/qxq_c.html
[radrec_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/radrec_c.html
[rav2xf_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/rav2xf_c.html
[raxisa_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/raxisa_c.html
[rdtext_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/rdtext_c.html
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
[reordc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/reordc_c.html
[reordd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/reordd_c.html
[reordi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/reordi_c.html
[reordl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/reordl_c.html
[repmc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/repmc_c.html
[repmct_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/repmct_c.html
[repmd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/repmd_c.html
[repmf_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/repmf_c.html
[repmi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/repmi_c.html
[repml_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/repml_c.html
[repmot_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/repmot_c.html
[reset_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/reset_c.html
[return_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/return_c.html
[rotate_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/rotate_c.html
[rotmat_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/rotmat_c.html
[rotvec_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/rotvec_c.html
[rpd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/rpd_c.html
[rquad_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/rquad_c.html
[saelgv_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/saelgv_c.html
[scard_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/scard_c.html
[scdecd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/scdecd_c.html
[sce2c_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sce2c_c.html
[sce2s_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sce2s_c.html
[sce2t_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sce2t_c.html
[scencd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/scencd_c.html
[scfmt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/scfmt_c.html
[scpart_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/scpart_c.html
[scs2e_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/scs2e_c.html
[sct2e_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sct2e_c.html
[sctiks_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sctiks_c.html
[sdiff_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sdiff_c.html
[set_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/set_c.html
[setmsg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/setmsg_c.html
[shellc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/shellc_c.html
[shelld_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/shelld_c.html
[shelli_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/shelli_c.html
[sigerr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sigerr_c.html
[sincpt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sincpt_c.html
[size_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/size_c.html
[spd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spd_c.html
[sphcyl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sphcyl_c.html
[sphlat_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sphlat_c.html
[sphrec_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sphrec_c.html
[spk14a_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spk14a_c.html
[spk14b_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spk14b_c.html
[spk14e_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spk14e_c.html
[spkacs_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkacs_c.html
[spkapo_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkapo_c.html
[spkapp_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkapp_c.html
[spkaps_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkaps_c.html
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
[spkgps_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkgps_c.html
[spklef_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spklef_c.html
[spkltc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkltc_c.html
[spkobj_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkobj_c.html
[spkopa_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkopa_c.html
[spkopn_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkopn_c.html
[spkpds_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkpds_c.html
[spkpos_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkpos_c.html
[spkpvn_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkpvn_c.html
[spksfs_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spksfs_c.html
[spkssb_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkssb_c.html
[spksub_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spksub_c.html
[spkuds_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkuds_c.html
[spkuef_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkuef_c.html
[spkw02_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkw02_c.html
[spkw03_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkw03_c.html
[spkw05_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkw05_c.html
[spkw08_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkw08_c.html
[spkw09_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkw09_c.html
[spkw10_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkw10_c.html
[spkw12_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkw12_c.html
[spkw13_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkw13_c.html
[spkw15_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkw15_c.html
[spkw17_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkw17_c.html
[spkw18_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkw18_c.html
[spkw20_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/spkw20_c.html
[srfc2s_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/srfc2s_c.html
[srfcss_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/srfcss_c.html
[srfnrm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/srfnrm_c.html
[srfrec_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/srfrec_c.html
[srfs2c_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/srfs2c_c.html
[srfscc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/srfscc_c.html
[srfxpt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/srfxpt_c.html
[ssize_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ssize_c.html
[stelab_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/stelab_c.html
[stlabx_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/stlabx_c.html
[stpool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/stpool_c.html
[str2et_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/str2et_c.html
[subpnt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/subpnt_c.html
[subpt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/subpt_c.html
[subslr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/subslr_c.html
[subsol_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/subsol_c.html
[sumad_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sumad_c.html
[sumai_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sumai_c.html
[surfnm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/surfnm_c.html
[surfpt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/surfpt_c.html
[surfpv_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/surfpv_c.html
[swpool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/swpool_c.html
[sxform_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sxform_c.html
[szpool_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/szpool_c.html
[tangpt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/tangpt_c.html
[termpt_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/termpt_c.html
[timdef_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/timdef_c.html
[timout_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/timout_c.html
[tipbod_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/tipbod_c.html
[tisbod_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/tisbod_c.html
[tkfram_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/tkfram_c.html
[tkvrsn_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/tkvrsn_c.html
[tparch_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/tparch_c.html
[tparse_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/tparse_c.html
[tpictr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/tpictr_c.html
[trace_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/trace_c.html
[trcdep_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/trcdep_c.html
[trcnam_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/trcnam_c.html
[trcoff_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/trcoff_c.html
[trgsep_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/trgsep_c.html
[tsetyr_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/tsetyr_c.html
[twopi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/twopi_c.html
[twovec_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/twovec_c.html
[twovxf_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/twovxf_c.html
[tyear_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/tyear_c.html
[ucase_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ucase_c.html
[ucrss_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/ucrss_c.html
[uddc_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/uddc_c.html
[uddf_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/uddf_c.html
[udf_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/udf_c.html
[union_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/union_c.html
[unitim_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/unitim_c.html
[unload_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/unload_c.html
[unorm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/unorm_c.html
[unormg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/unormg_c.html
[utc2et_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/utc2et_c.html
[vadd_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vadd_c.html
[vaddg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vaddg_c.html
[valid_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/valid_c.html
[vcrss_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vcrss_c.html
[vdist_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vdist_c.html
[vdistg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vdistg_c.html
[vdot_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vdot_c.html
[vdotg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vdotg_c.html
[vequ_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vequ_c.html
[vequg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vequg_c.html
[vhat_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vhat_c.html
[vhatg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vhatg_c.html
[vlcom_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vlcom_c.html
[vlcom3_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vlcom3_c.html
[vlcomg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vlcomg_c.html
[vminug_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vminug_c.html
[vminus_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vminus_c.html
[vnorm_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vnorm_c.html
[vnormg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vnormg_c.html
[vpack_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vpack_c.html
[vperp_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vperp_c.html
[vprjp_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vprjp_c.html
[vprjpi_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vprjpi_c.html
[vproj_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vproj_c.html
[vprojg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vprojg_c.html
[vrel_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vrel_c.html
[vrelg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vrelg_c.html
[vrotv_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vrotv_c.html
[vscl_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vscl_c.html
[vsclg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vsclg_c.html
[vsep_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vsep_c.html
[vsepg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vsepg_c.html
[vsub_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vsub_c.html
[vsubg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vsubg_c.html
[vtmv_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vtmv_c.html
[vtmvg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vtmvg_c.html
[vupack_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vupack_c.html
[vzero_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vzero_c.html
[vzerog_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/vzerog_c.html
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
[xposeg_c link]: https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/xposeg_c.html
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
    bodc2n, bodc2s, ckcov, ckobj, cmprss, dafec, dasec, dskobj, dskp02, dsksrf, dskv02, et2lst,
    et2utc, frmnam, gcpool, getfat, getfov, getfvn, gfdist, gfilum, gfoclt, gfpa, gfposc, gfrfov,
    gfrr, gfsep, gfsntc, gfsubc, gftfov, gfudb, gfuds, gnpool, kdata, kinfo, lcase, lparse, lparsm,
    nextwd, pckcov, pckfrm, repmc, repmct, repmd, repmi, scdecd, sce2s, spkcov, spkobj, spksfs,
    srfc2s, srfcss, stpool, timdef, timout, tparse, tpictr, trcnam, ucase,
};
#[allow(unused_imports)]
pub use self::raw::{
    appndc, appndd, appndi, axisar, azlcpo, azlrec, b1900, b1950, badkpv, bltfrm, boddef, bodfnd,
    bodn2c, bods2c, bodvar, bodvcd, bodvrd, brcktd, brckti, bschoc, bschoi, bsrchc, bsrchd, bsrchi,
    card, ccifrm, cgv2el, chbder, chbigr, chbint, chbval, chkin, chkout, cidfrm, ckcls, ckfrot,
    ckfxfm, ckgp, ckgpav, ckgr02, ckgr03, cklpf, ckmeta, cknr02, cknr03, ckopn, ckupf, ckw01,
    ckw02, ckw03, ckw05, clearc, cleard, cleari, clight, clpool, cnmfrm, conics, convrt, copy,
    cpos, cposr, cvpool, cyllat, cylrec, cylsph, dafac, dafbbs, dafbfs, dafcls, dafcs, dafdc,
    daffna, daffpa, dafgda, dafgh, dafgn, dafgs, dafgsr, dafhsf, dafopr, dafopw, dafps, dafrda,
    dafrfr, dafrs, dafus, dasac, dasadc, dasadd, dasadi, dascls, dasdc, dashfn, dashfs, daslla,
    dasllc, dasonw, dasopr, dasops, dasopw, dasrdc, dasrdd, dasrdi, dasrfr, dasudc, dasudd, dasudi,
    daswbr, dazldr, dcyldr, deltet, det, dgeodr, diags2, diff, dlabbs, dlabfs, dlabns, dlaens,
    dlafns, dlafps, dlaopn, dlatdr, dnearp, dp2hx, dpgrdr, dpmax, dpmin, dpr, drdazl, drdcyl,
    drdgeo, drdlat, drdpgr, drdsph, dskb02, dskcls, dskd02, dskgd, dskgtl, dski02, dskmi2, dskn02,
    dskopn, dskrb2, dskstl, dskw02, dskx02, dskxsi, dskxv, dskz02, dsphdr, dtpool, ducrss, dvcrss,
    dvdot, dvhat, dvnorm, dvpool, dvsep, edlimb, ednmpt, edpnt, edterm, ekacec, ekaced, ekacei,
    ekaclc, ekacld, ekacli, ekappr, ekbseg, ekccnt, ekcii, ekcls, ekdelr, ekffld, ekfind, ekgc,
    ekgd, ekgi, ekifld, ekinsr, eklef, eknelt, eknseg, ekntab, ekopn, ekopr, ekops, ekopw, ekpsel,
    ekrcec, ekrced, ekrcei, ekssum, ektnam, ekucec, ekuced, ekucei, ekuef, el2cgv, elemc, elemd,
    elemi, eqncpv, eqstr, errch, errdp, errint, esrchc, etcal, eul2m, eul2xf, evsgp4, exists,
    expool, filld, filli, fovray, fovtrg, frame, frinfo, ftncls, furnsh, gdpool, georec, getcml,
    getelm, gfbail, gfclrh, gfevnt, gffove, gfinth, gfocce, gfrefn, gfrepf, gfrepi, gfrepu, gfsstp,
    gfstep, gfstol, gipool, halfpi, hrmesp, hrmint, hx2dp, ident, illum, illumf, illumg, ilumin,
    inedpl, inelpl, inrypl, insrtc, insrtd, insrti, inter, intmax, intmin, invert, invort, invstm,
    isordv, isrchc, isrchd, isrchi, isrot, iswhsp, j1900, j1950, j2000, j2100, jyear, kclear,
    kplfrm, ktotal, kxtrct, lastnb, latcyl, latrec, latsph, latsrf, ldpool, lgresp, lgrind, lgrint,
    limbpt, lmpool, lparss, lspcn, lstlec, lstled, lstlei, lstltc, lstltd, lstlti, ltime, lx4dec,
    lx4num, lx4sgn, lx4uns, lxqstr, m2eul, m2q, matchi, matchw, maxd, maxi, mequ, mequg, mind,
    mini, moved, mtxm, mtxmg, mtxv, mtxvg, mxm, mxmg, mxmt, mxmtg, mxv, mxvg, namfrm, ncpos,
    ncposr, nearpt, npedln, npelpt, nplnpt, nthwd, nvc2pl, nvp2pl, occult, ordc, ordd, orderc,
    orderd, orderi, ordi, oscelt, oscltx, pckcls, pcklof, pckopn, pckuof, pckw02, pcpool, pdpool,
    pgrrec, phaseq, pi, pipool, pjelpl, pl2nvc, pl2nvp, pl2psv, pltar, pltexp, pltnp, pltnrm,
    pltvol, polyds, pos, posr, prompt, prop2b, prsdp, prsint, psv2pl, putcml, pxform, pxfrm2, q2m,
    qderiv, qdq2av, qxq, radrec, rav2xf, raxisa, rdtext, recazl, reccyl, recgeo, reclat, recpgr,
    recrad, recsph, removc, removd, removi, reordc, reordd, reordi, reordl, repmf, repml, repmot,
    return_c, rotate, rotmat, rotvec, rpd, rquad, saelgv, scard, sce2c, sce2t, scencd, scfmt,
    scpart, scs2e, sct2e, sctiks, sdiff, set, setmsg, shellc, shelld, shelli, sigerr, sincpt, size,
    spd, sphcyl, sphlat, sphrec, spk14a, spk14b, spk14e, spkacs, spkapo, spkapp, spkaps, spkcls,
    spkcpo, spkcpt, spkcvo, spkcvt, spkez, spkezp, spkezr, spkgeo, spkgps, spklef, spkltc, spkopa,
    spkopn, spkpds, spkpos, spkpvn, spkssb, spksub, spkuds, spkuef, spkw02, spkw03, spkw05, spkw08,
    spkw09, spkw10, spkw12, spkw13, spkw15, spkw17, spkw18, spkw20, srfnrm, srfrec, srfs2c, srfscc,
    srfxpt, ssize, stelab, stlabx, str2et, subpnt, subpt, subslr, subsol, sumad, sumai, surfnm,
    surfpt, surfpv, swpool, sxform, szpool, tangpt, termpt, tipbod, tisbod, tkfram, tkvrsn, tparch,
    trace, trcdep, trcoff, trgsep, tsetyr, twopi, twovec, twovxf, tyear, ucrss, uddc, uddf, udf,
    union, unitim, unload, unorm, unormg, utc2et, vadd, vaddg, valid, vcrss, vdist, vdistg, vdot,
    vdotg, vequ, vequg, vhat, vhatg, vlcom, vlcom3, vlcomg, vminug, vminus, vnorm, vnormg, vpack,
    vperp, vprjp, vprjpi, vproj, vprojg, vrel, vrelg, vrotv, vscl, vsclg, vsep, vsepg, vsub, vsubg,
    vtmv, vtmvg, vupack, vzero, vzerog, wncard, wncomd, wncond, wndifd, wnelmd, wnexpd, wnextd,
    wnfetd, wnfild, wnfltd, wnincd, wninsd, wnintd, wnreld, wnsumd, wnunid, wnvald, xf2eul, xf2rav,
    xfmsta, xpose, xpose6, xposeg, CELL, DAF_MAXSUM, DLADSC, DSK02_SPADSZ, DSKDSC, DSKXSI_DCSIZE,
    DSKXSI_ICSIZE, DSK_KEYAMG, DSK_KEYLAL, DSK_KEYPTM, DSK_KEYSGR, DSK_KEYSPM, DSK_KEYXFR,
    DSK_NSYPAR, EKATTDSC, EKSEGSUM, EK_CHR, EK_CSTRLN, EK_DP, EK_EXP_COL, EK_EXP_EXPR, EK_EXP_FUNC,
    EK_INT, EK_MAXQSEL, EK_MXCLSG, EK_TIME, EK_TSTRLN, EK_VARSIZ, ELLIPSE, PLANE, SPK_DSCSIZ,
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
