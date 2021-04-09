use nalgebra::{Matrix3, Rotation3};
use std::ffi::{CStr, CString};
use std::fmt;
use std::path::PathBuf;
use tool::Vector;

/// Instructions to format an ephemeris time to a string.
pub const TIME_FORMAT: &str = "YYYY-MON-DD HR:MN:SC ::RND";

/// An error which can be returned when using a kernel.
///
/// This error can happen when loading/unloading a kernel.
///
/// Read [`KernelErrorKind`] for the different kinds of errors related to kernels.
#[derive(Clone, PartialEq)]
pub struct KernelError {
    pub kind: KernelErrorKind,
}

/// Enumeration of the different kinds of errors about kernels.
#[derive(Debug, Clone, PartialEq)]
pub enum KernelErrorKind {
    /// The kernel is already loaded and cannot be loaded twice.
    AlreadyLoaded,
    /// The kernel is not loaded or has been unloaded already.
    NotLoaded,
}

impl KernelError {
    /// Error messages for kernel failing.
    fn description(&self) -> &str {
        match self.kind {
            KernelErrorKind::AlreadyLoaded => "the kernel is already loaded",
            KernelErrorKind::NotLoaded => "the kernel is not loaded, it cannot be unloaded",
        }
    }
}

impl fmt::Display for KernelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}
impl fmt::Debug for KernelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

/// Load the kernel if it not already loaded.
fn load<S: AsRef<str>>(name: S) {
    let _name = name.as_ref();
    unsafe {
        let kernel = CString::new(_name).unwrap().into_raw();
        crate::c::furnsh_c(kernel);
    }
}

/// Unload the kernel if loaded.
fn unload<S: AsRef<str>>(name: S) {
    let _name = name.as_ref();
    unsafe {
        let kernel = CString::new(_name).unwrap().into_raw();
        crate::c::unload_c(kernel);
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
/// let mut kernel = spice::Kernel::new("rsc/data/hera_PO_EMA_2024.tm")?;
/// kernel.unload()?;
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
            KernelStatus::Loaded => Err(KernelError {
                kind: KernelErrorKind::AlreadyLoaded,
            }),

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
            KernelStatus::Unloaded => Err(KernelError {
                kind: KernelErrorKind::NotLoaded,
            }),

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
/// let mut kernel = spice::Kernel::new("rsc/data/hera_PO_EMA_2024.tm")?;
/// let time = spice::ephemeris_from_date("2027-MAR-23 16:00:00");
///
/// // time: 859089667.1856234
///
/// kernel.unload()?;
/// ```
pub fn str2et<S: Into<String>>(date: S) -> f64 {
    let mut ephemeris_time = 0.0;
    unsafe {
        let date_c = CString::new(date.into()).unwrap().into_raw();
        crate::c::str2et_c(date_c, &mut ephemeris_time);
    }
    ephemeris_time
}

/// Convert an ephemeris time to a formatted string.
///
/// # Example
///
/// ```
/// let mut kernel = spice::Kernel::new("rsc/data/hera_PO_EMA_2024.tm")?;
/// let date = spice::date_from_ephemeris(859089667.1856234, spice::TIME_FORMAT);
///
/// // date: "2027-MAR-23 16:00:00"
///
/// kernel.unload()?;
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
/// let mut kernel = spice::Kernel::new("rsc/data/hera_PO_EMA_2024.tm")?;
/// let time = spice::ephemeris_from_date("2027-MAR-23 16:00:00");
/// let (position, light_time) = spice::position("DIMORPHOS", time, "J2000", "NONE", "HERA");
///
/// // position: [18.62639796759623  21.05444863563425 -7.136416860555217]
/// // light_time: 0.00009674284381011395
///
/// kernel.unload()?;
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
/// let mut kernel = spice::Kernel::new("rsc/data/hera_PO_EMA_2024.tm")?;
/// let et = spice::ephemeris_from_date("2027-MAR-23 16:00:00");
///
/// let matrix = spice::pxform("J2000", "DIMORPHOS", time);
///
/// kernel.unload()?;
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
/// let mut kernel = spice::Kernel::new("rsc/data/hera_PO_EMA_2024.tm")?;
/// let et_from = spice::ephemeris_from_date("2027-MAR-23 16:00:00");
/// let et_to = spice::ephemeris_from_date("2027-MAR-24 16:00:00");
///
/// let matrix = spice::pxfrm2("J2000", "DIMORPHOS", et_from, et_to);
///
/// kernel.unload()?;
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

#[macro_use]
#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn already_loaded_description() -> Result<(), KernelError> {
        let mut kernel = Kernel::new("rsc/data/hera_PO_EMA_2024.tm")?;

        assert_eq!(
            kernel.load().err().unwrap().description(),
            "the kernel is already loaded"
        );

        kernel.unload()?;
        Ok(())
    }

    #[test]
    #[serial]
    fn not_loaded_description() -> Result<(), KernelError> {
        let mut kernel = Kernel::new("rsc/data/hera_PO_EMA_2024.tm")?;
        kernel.unload()?;

        match kernel.unload() {
            Ok(_) => (),
            Err(e) => assert_eq!(
                e.description(),
                "the kernel is not loaded, it cannot be unloaded"
            ),
        };

        Ok(())
    }
}
