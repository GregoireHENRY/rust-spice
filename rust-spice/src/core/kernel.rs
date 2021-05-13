/*!
# SPICE functions for accessing SPICE kernel data

## Loading and Unloading SPICE Kernels

CSpice | `rust-spice` | Description
-------|--------------|------------
[furnsh_c][furnsh_c link] | [`Kernel::load`] | loads an individual kernel or a collection of kernels
[unload_c][unload_c link] | [`Kernel::unload`] | unloads an individual kernel or a collection of kernels

## Converting between UTC and Ephemeris Time (LSK)

CSpice | `rust-spice` | Description
-------|--------------|------------
[str2et_c][str2et_c link] | [`str2et`] | converts a time string to ET seconds past J2000
[timout_c][timout_c link] | [`timout`] | converts ET seconds past J2000 to a time string

## Converting between Ephemeris Time and Spacecraft Clock (SCLK)

CSpice | `rust-spice` | Description
-------|--------------|------------
[scs2e_c][scs2e_c link] | *TODO*
[sce2s_c][sce2s_c link] | *TODO*
[sct2e_c][sct2e_c link] | *TODO*
[sce2c_c][sce2c_c link] | *TODO*
[scencd_c][scencd_c link] | *TODO*
[scdecd_c][scdecd_c link] | *TODO*

## Constants and Orientation for Natural Bodies (PCK)

CSpice | `rust-spice` | Description
-------|--------------|------------
[bodfnd_c][bodfnd_c link] | *TODO*
[bodvrd_c][bodvrd_c link] | *TODO*
[pxform_c][pxform_c link] | [`pxform`] | returns the 3x3 matrix rotating a position vector one frame to another
[sxform_c][sxform_c link] | *TODO*
[pckfrm_c][pckfrm_c link] | *TODO*
[pckcov_c][pckcov_c link] | *TODO*

## Computing Transformations Between Frames (FK)

CSpice | `rust-spice` | Description
-------|--------------|------------
[pxfrm2_c][pxfrm2_c link] | [`pxfrm2`] | returns the 3x3 matrix rotating a position vector from one frame at a specified epoch to another frame at a different epoch

## Computing Positions of Spacecraft and Natural Bodies (SPK)

CSpice | `rust-spice` | Description
-------|--------------|------------
[spkezr_c][spkezr_c link] | *TODO*
[spkpos_c][spkpos_c link] | [`spkpos`] | returns the position of a target body relative to an observing body
[spkcpo_c][spkcpo_c link] | *TODO*
[spkcpt_c][spkcpt_c link] | *TODO*
[spkcvo_c][spkcvo_c link] | *TODO*
[spkcvt_c][spkcvt_c link] | *TODO*
[spkobj_c][spkobj_c link] | *TODO*
[spkcov_c][spkcov_c link] | *TODO*

## Computing Orientation for Spacecraft and Instruments (CK)

CSpice | `rust-spice` | Description
-------|--------------|------------
[ckobj_c][ckobj_c link] | *TODO*
[ckcov_c][ckcov_c link] | *TODO*
[ckgp_c][ckgp_c link] | *TODO*
[ckgpav_c][ckgpav_c link] | *TODO*

## Retrieving Instrument Parameters (IK)

CSpice | `rust-spice` | Description
-------|--------------|------------
[getfov_c][getfov_c link] | *TODO*
[gdpool_c][gdpool_c link] | *TODO*
[gipool_c][gipool_c link] | *TODO*
[gcpool_c][gcpool_c link] | *TODO*

## Computing surface coordinates using digital shape (DSK)

CSpice | `rust-spice` | Description
-------|--------------|------------
[latsrf_c][latsrf_c link] | *TODO*
[srfnrm_c][srfnrm_c link] | *TODO*
[dskz02_c][dskz02_c link] | *TODO*
[dskp02_c][dskp02_c link] | *TODO*
[dskv02_c][dskv02_c link] | *TODO*
[dskobj_c][dskobj_c link] | *TODO*
[dsksrf_c][dsksrf_c link] | *TODO*

## Mapping Between Object Names and NAIF IDs

CSpice | `rust-spice` | Description
-------|--------------|------------
[bodc2n_c][bodc2n_c link] | *TODO*
[bodn2c_c][bodn2c_c link] | *TODO*

## Mapping between surface names and NAIF IDs

CSpice | `rust-spice` | Description
-------|--------------|------------
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

use crate::cstr;
use nalgebra::{Matrix3, Rotation3};
use std::ffi::{CStr, CString};
use std::path::PathBuf;
use tool::Vector;

/// Instructions to format an ephemeris time to a string.
pub const TIME_FORMAT: &str = "YYYY-MON-DD HR:MN:SC ::RND";

/// Enumeration of the different kinds of errors that can happen when using [`Kernel`].
#[derive(Debug, thiserror::Error)]
pub enum KernelError {
    #[error("cannot reload the kernel")]
    AlreadyLoaded,
    /// The kernel is not loaded or has been unloaded already.
    #[error("the kernel has never been loaded")]
    NotLoaded,
}

/// Load the kernel if it not already loaded.
fn load<S: Into<String>>(name: S) {
    unsafe {
        crate::c::furnsh_c(cstr!(name.into()));
    }
}

/// Unload the kernel if loaded.
fn unload<S: Into<String>>(name: S) {
    unsafe {
        crate::c::unload_c(cstr!(name.into()));
    }
}

/// Status on the state of the loading of the kernel.
#[derive(Debug, Copy, Clone, PartialEq)]
enum KernelStatus {
    Loaded,
    Unloaded,
}

/// Kernel type to automatically load the kernel on the definition and keep a record of the status
/// of the loading.
///
/// # Example
///
/// ```
/// let mut kernel = spice::Kernel::new("rsc/krn/hera_study_PO_EMA_2024.tm")?;
///
/// kernel.unload()?;
/// # Ok::<(), spice::KernelError>(())
/// ```
#[derive(Debug, Clone)]
pub struct Kernel {
    /// The file to the kernel.
    file: PathBuf,
    /// The status of the loading of the kernel.
    status: KernelStatus,
}

impl Kernel {
    /// Constructor from the path of the file. Automatically load the kernel.
    pub fn new<P: Into<PathBuf>>(file: P) -> Result<Self, KernelError> {
        let mut kernel = Self {
            file: file.into(),
            status: KernelStatus::Unloaded,
        };
        kernel.load()?;
        Ok(kernel)
    }

    /// Get the path as a string.
    pub fn name(&self) -> String {
        self.file.to_str().unwrap().to_string()
    }

    /// Load the kernel if not loaded already.
    pub fn load(&mut self) -> Result<(), KernelError> {
        match self.status {
            KernelStatus::Loaded => Err(KernelError::AlreadyLoaded),

            KernelStatus::Unloaded => {
                // Load the kernel.
                load(self.name());
                // Update status
                self.status = KernelStatus::Loaded;

                Ok(())
            }
        }
    }

    /// Unload the kernel if loaded.
    pub fn unload(&mut self) -> Result<(), KernelError> {
        match self.status {
            KernelStatus::Unloaded => Err(KernelError::NotLoaded),

            KernelStatus::Loaded => {
                // Unload the kernel.
                unload(self.name());

                // Update status
                self.status = KernelStatus::Unloaded;

                Ok(())
            }
        }
    }
}

/// Convert a formatted date from string to ephemeris time.
///
/// # Example
///
/// ```
/// # use approx::assert_relative_eq;
/// let mut kernel = spice::Kernel::new("rsc/krn/hera_study_PO_EMA_2024.tm")?;
///
/// let time = spice::str2et("2027-MAR-23 16:00:00");
/// assert_relative_eq!(time, 859089667.1856234, epsilon = f64::EPSILON);
///
/// kernel.unload()?;
/// # Ok::<(), spice::KernelError>(())
/// ```
pub fn str2et<S: Into<String>>(date: S) -> f64 {
    let mut ephemeris_time = 0.0;
    unsafe {
        crate::c::str2et_c(cstr!(date.into()), &mut ephemeris_time);
    }
    ephemeris_time
}

/// Convert an ephemeris time to a formatted string.
///
/// # Example
///
/// ```
/// let mut kernel = spice::Kernel::new("rsc/krn/hera_study_PO_EMA_2024.tm")?;
///
/// let date = spice::timout(859089667.1856234, spice::TIME_FORMAT);
/// assert_eq!(date, "2027-MAR-23 16:00:00");
///
/// kernel.unload()?;
/// # Ok::<(), spice::KernelError>(())
/// ```
pub fn timout(time: f64, time_format: &str) -> String {
    // Define pointer where data will be written.
    let size = time_format.len();
    let date_c = CString::new(String::with_capacity(size))
        .unwrap()
        .into_raw();

    unsafe {
        // Get data.
        let time_format_c = CString::new(time_format.to_string()).unwrap().into_raw();
        crate::c::timout_c(time, time_format_c, size as i32 + 1, date_c);

        // Convert data to Rust type.
        CStr::from_ptr(date_c).to_str().unwrap().to_string()
    }
}

/// Get the position and one way light time of target with respect to observer in the reference
/// frame at time with optional aberration correction.
///
/// # Example
///
/// ```
/// # use approx::assert_relative_eq;
/// # use itertools::multizip;
///
/// let mut kernel = spice::Kernel::new("rsc/krn/hera_study_PO_EMA_2024.tm")?;
/// let time = spice::str2et("2027-MAR-23 16:00:00");
///
/// let (position, light_time) = spice::spkpos("DIMORPHOS", time, "J2000", "NONE", "HERA");
///
/// let expected_position = tool::Vector::new(18.62640405424448, 21.054373008357004, -7.136291402940499);
/// let expected_light_time = 0.00009674257074746383;
///
/// for (component, expected_component) in multizip((position.iter(), expected_position.iter())) {
///     assert_relative_eq!(
///         component,
///         expected_component,
///         epsilon = f64::EPSILON
///     );
/// }
///
/// assert_relative_eq!(
///     light_time,
///     expected_light_time,
///     epsilon = f64::EPSILON
/// );
///
/// kernel.unload()?;
/// # Ok::<(), spice::KernelError>(())
/// ```
pub fn spkpos<S: Into<String>>(
    target: S,
    time: f64,
    frame: S,
    aberration_correction: S,
    observer: S,
) -> (Vector<f64>, f64) {
    // Define pointers where data will be written.
    let mut light_time = 0.0;
    let mut position = [0.0, 0.0, 0.0];

    unsafe {
        // Get data.
        let target_c = CString::new(target.into()).unwrap().into_raw();
        let frame_c = CString::new(frame.into()).unwrap().into_raw();
        let aberration_correction_c = CString::new(aberration_correction.into())
            .unwrap()
            .into_raw();
        let observer_c = CString::new(observer.into()).unwrap().into_raw();
        crate::c::spkpos_c(
            target_c,
            time,
            frame_c,
            aberration_correction_c,
            observer_c,
            &mut position[0],
            &mut light_time,
        );
    }

    // Convert data to Rust type.
    (
        Vector::new(position[0], position[1], position[2]),
        light_time,
    )
}

/// Return the matrix that transforms position vectors from one specified frame to another at a
/// specified epoch.
///
/// # Example
///
/// ```
/// let mut kernel = spice::Kernel::new("rsc/krn/hera_study_PO_EMA_2024.tm")?;
/// let et = spice::str2et("2027-MAR-23 16:00:00");
///
/// let matrix = spice::pxform("J2000", "DIMORPHOS_FIXED", et);
///
/// kernel.unload()?;
/// # Ok::<(), spice::KernelError>(())
/// ```
pub fn pxform<S: Into<String>>(from: S, to: S, time: f64) -> Rotation3<f64> {
    // Define pointers where data will be written.
    let mut rotate = [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];

    unsafe {
        // Get data.
        let from_c = CString::new(from.into()).unwrap().into_raw();
        let to_c = CString::new(to.into()).unwrap().into_raw();
        crate::c::pxform_c(from_c, to_c, time, &mut rotate[0]);
    }

    // Convert data to Rust type.
    Rotation3::from_matrix(&Matrix3::from_rows(&[
        Vector::from_iterator(rotate[0].iter().cloned()).transpose(),
        Vector::from_iterator(rotate[1].iter().cloned()).transpose(),
        Vector::from_iterator(rotate[2].iter().cloned()).transpose(),
    ]))
}

/// Return the 3x3 matrix that transforms position vectors from one specified frame at a specified
/// epoch to another specified frame at another specified epoch.
///
/// # Example
///
/// ```
/// let mut kernel = spice::Kernel::new("rsc/krn/hera_study_PO_EMA_2024.tm")?;
/// let et_from = spice::str2et("2027-MAR-23 16:00:00");
/// let et_to = spice::str2et("2027-MAR-24 16:00:00");
///
/// let matrix = spice::pxfrm2("J2000", "DIMORPHOS_FIXED", et_from, et_to);
///
/// kernel.unload()?;
/// # Ok::<(), spice::KernelError>(())
/// ```
pub fn pxfrm2<S: Into<String>>(from: S, to: S, et_from: f64, et_to: f64) -> Rotation3<f64> {
    // Define pointers where data will be written.
    let mut rotate = [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];

    unsafe {
        // Get data.
        let from_c = CString::new(from.into()).unwrap().into_raw();
        let to_c = CString::new(to.into()).unwrap().into_raw();
        crate::c::pxfrm2_c(from_c, to_c, et_from, et_to, &mut rotate[0]);
    }

    // Convert data to Rust type.
    Rotation3::from_matrix(&Matrix3::from_rows(&[
        Vector::from_iterator(rotate[0].iter().cloned()).transpose(),
        Vector::from_iterator(rotate[1].iter().cloned()).transpose(),
        Vector::from_iterator(rotate[2].iter().cloned()).transpose(),
    ]))
}
