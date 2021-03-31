use na::Matrix3x1;
use std::ffi::{CStr, CString};
use std::fmt;
use std::path::PathBuf;

/// Instructions to format an ephemeris time to a string.
pub const TIME_FORMAT: &str = "YYYY-MON-DD HR:MN:SC ::RND";

/// Convert a formatted date from string to ephemeris time.
pub fn ephemeris_from_date<S: Into<String>>(date: S) -> f64 {
    let mut ephemeris_time = 0.0;
    unsafe {
        let date_c = CString::new(date.into()).unwrap().into_raw();
        crate::c::str2et_c(date_c, &mut ephemeris_time);
    }
    ephemeris_time
}

/// Convert an ephemeris time to a formatted string.
pub fn date_from_ephemeris(time: f64) -> String {
    // Define pointer where data will be written.
    let size = TIME_FORMAT.len();
    let date_c = CString::new(String::with_capacity(size))
        .unwrap()
        .into_raw();

    unsafe {
        // Get data.
        let time_format_c = CString::new(TIME_FORMAT.to_string()).unwrap().into_raw();
        crate::c::timout_c(time, time_format_c, size as i32 + 1, date_c);

        // Convert data to Rust type.
        CStr::from_ptr(date_c).to_str().unwrap().to_string()
    }
}

/// Get position of target with respect to observer in the reference frame at time with optional
/// aberration correction.
pub fn position<S: Into<String>>(
    target: S,
    time: f64,
    frame: S,
    aberration_correction: S,
    observer: S,
) -> (Matrix3x1<f64>, f64) {
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
        Matrix3x1::new(position[0], position[1], position[2]),
        light_time,
    )
}

/// List of loaded kernels to avoid loading an already loaded.
pub static mut LoadedKernels: Vec<String> = vec![];

/// Custom error type when trying to load an already load kernel.
#[derive(Debug)]
pub struct KernelAlreadyLoadedError {
    name: String,
}

impl KernelAlreadyLoadedError {
    /// Build KernelAlreadyLoadedError with the name of the kernel.
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl fmt::Display for KernelAlreadyLoadedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The kernel {} is already loaded.", self.name)
    }
}

/// Custom error type when trying to unload an kernel that was not loaded.
#[derive(Debug)]
pub struct KernelNotLoadedError {
    name: String,
}

impl KernelNotLoadedError {
    /// Build KernelNotLoadedError with the name of the kernel.
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl fmt::Display for KernelNotLoadedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The kernel {} was not loaded.", self.name)
    }
}

/// Load the kernel if it not already loaded.
pub fn load<S: AsRef<str>>(name: S) -> Result<(), KernelAlreadyLoadedError> {
    let _name = name.as_ref();
    unsafe {
        // Check if the name of the kernel is in the static list of already loaded kernels.
        match LoadedKernels.contains(&_name.to_string()) {
            true => Err(KernelAlreadyLoadedError::new(_name.to_string())),
            false => {
                // Add the name of the kernel to this list.
                LoadedKernels.push(_name.to_string());

                // Load it.
                let kernel = CString::new(_name).unwrap().into_raw();
                crate::c::furnsh_c(kernel);

                Ok(())
            }
        }
    }
}

/// Unload the kernel if loaded.
pub fn unload<S: AsRef<str>>(name: S) -> Result<(), KernelNotLoadedError> {
    let _name = name.as_ref();
    unsafe {
        // Check if the name of the kernel is not in the static list of already loaded kernels.
        match !LoadedKernels.contains(&_name.to_string()) {
            true => Err(KernelNotLoadedError::new(_name.to_string())),
            false => {
                // Remove the name of the kernel from this list.
                LoadedKernels.retain(|n| n != &_name.to_string());

                // Unload it.
                let kernel = CString::new(_name).unwrap().into_raw();
                crate::c::unload_c(kernel);

                Ok(())
            }
        }
    }
}

/// Status on the state of the loading of the kernel.
#[derive(Debug, Copy, Clone)]
enum KernelStatus {
    Loaded,
    Unloaded,
}

/// Kernel type to automatically load the kernel on the definition and keep a record of the status
/// of the loading.
#[derive(Debug, Clone)]
pub struct Kernel {
    /// The file to the kernel.
    file: PathBuf,
    /// The status of the loading of the kernel.
    status: KernelStatus,
}

impl Kernel {
    /// Constructor from the path of the file. Automatically load the kernel.
    pub fn new<P: Into<PathBuf>>(file: P) -> Result<Self, KernelAlreadyLoadedError> {
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
    pub fn load(&mut self) -> Result<(), KernelAlreadyLoadedError> {
        match self.status {
            KernelStatus::Loaded => Err(KernelAlreadyLoadedError::new(self.name())),
            KernelStatus::Unloaded => {
                // Load if not loaded by someone else.
                load(self.name())?;

                // Update status
                self.status = KernelStatus::Loaded;

                Ok(())
            }
        }
    }

    /// Unload the kernel if loaded.
    pub fn unload(&mut self) -> Result<(), KernelNotLoadedError> {
        match self.status {
            KernelStatus::Unloaded => Err(KernelNotLoadedError::new(self.name())),
            KernelStatus::Loaded => {
                // Unload if loaded.
                unload(self.name())?;

                // Update status
                self.status = KernelStatus::Unloaded;

                Ok(())
            }
        }
    }
}
