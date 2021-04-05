use itertools::multizip;
use na::{Matrix1xX, Matrix3x1, Matrix3xX};
use std::fmt;
use std::path::PathBuf;
use std::vec::Vec;

/// An error which can be returned during the build of the type System.
///
/// This error happens if any of the fields of the type System is missing when building.
///
/// Read [`SystemBuilderErrorKind`] for the different kinds of errors related to the build of the
/// type System.
#[derive(Clone)]
pub struct SystemBuilderError {
    pub kind: SystemBuilderErrorKind,
}

/// Enumeration of the different kinds of errors about the build of the type System.
#[derive(Clone)]
pub enum SystemBuilderErrorKind {
    /// Kernel is missing.
    Kernel,
    /// Frame is missing.
    Frame,
    /// Observer is missing.
    Observer,
    /// Target is missing.
    Target,
    /// Start date is missing.
    StartDate,
    /// Duration is missing.
    Duration,
    /// Aberration correction is missing.
    AberrationCorrection,
}

impl SystemBuilderError {
    /// Error messages for system builder failing.
    fn description(&self) -> &str {
        match self.kind {
            SystemBuilderErrorKind::Kernel => "the kernel must be initialized",
            SystemBuilderErrorKind::Frame => "the frame must be initialized",
            SystemBuilderErrorKind::Observer => "the observer must be initialized",
            SystemBuilderErrorKind::Target => "the target must be initialized",
            SystemBuilderErrorKind::StartDate => "the start date must be initialized",
            SystemBuilderErrorKind::Duration => "the duration must be initialized",
            SystemBuilderErrorKind::AberrationCorrection => {
                "the aberration correction must be initialized"
            }
        }
    }
}

impl fmt::Display for SystemBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

impl fmt::Debug for SystemBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

/// An error which can be returned when using the type System.
///
/// This error can happen when loading/unload a kernel or building the system.
///
/// Read [`SystemErrorKind`] for the different kinds of errors related to the type System.
#[derive(Clone)]
pub struct SystemError {
    pub kind: SystemErrorKind,
}

/// Enumeration of the different kinds of errors about the type System.
#[derive(Clone)]
pub enum SystemErrorKind {
    /// Errors in loading/unloading a kernel, see [`KernelError`].
    Kernel(crate::KernelError),
    /// Error during the build of the type System, see [`SystemBuilderError`].
    Build(SystemBuilderError),
}

impl SystemError {
    /// Error messages for System failing.
    fn description(&self) -> String {
        match self.kind {
            SystemErrorKind::Kernel(ref e) => format!("error with the kernel: {}", e),
            SystemErrorKind::Build(ref e) => format!("error during the build: {}", e),
        }
    }
}

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

impl fmt::Debug for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

/// Implement the conversion from `KernelError` to `SystemError`. This will be automatically called
/// by `?` if a `KernelError` needs to be converted into a `SystemError`.
impl From<crate::KernelError> for SystemError {
    fn from(err: crate::KernelError) -> SystemError {
        SystemError {
            kind: SystemErrorKind::Kernel(err),
        }
    }
}

/// Implement the conversion from `SystemBuilderError` to `SystemError`. This will be automatically called
/// by `?` if a `SystemBuilderError` needs to be converted into a `SystemError`.
impl From<SystemBuilderError> for SystemError {
    fn from(err: SystemBuilderError) -> SystemError {
        SystemError {
            kind: SystemErrorKind::Build(err),
        }
    }
}

/// System type to quickly provide sets of basic functions.
///
/// # Example
///
/// The type [`System`] is created from the builder [`SystemBuilder`].
///
/// ```.ignore
/// use spice;
///
/// let mut system = spice::SystemBuilder::default()
///     .kernel("/path/to/metakernels.mk")?
///     .frame("J2000")
///     .observer("HERA")
///     .target("DIMORPHOS")
///     .start_date("2027-MAR-23 16:00:00")
///     .duration(129.0 * spice::DAY)
///     .aberration_correction("NONE")
///     .build()?;
///
/// system.unload()?;
/// ```
#[derive(Debug, Clone)]
pub struct System {
    /// The kernel containing the information.
    kernel: crate::Kernel,
    /// The reference frame.
    frame: String,
    /// The origin of the reference frame.
    observer: String,
    /// The target.
    target: String,
    /// The formatted start date.
    start_date: String,
    /// The duration of the window to compute the end date.
    duration: f64,
    /// The aberration correction.
    aberration_correction: String,
}

impl System {
    /// Get a reference to the kernel.
    pub fn kernel(&self) -> &crate::Kernel {
        &self.kernel
    }

    /// Get the frame.
    pub fn frame(&self) -> String {
        self.frame.clone()
    }

    /// Get the observer.
    pub fn observer(&self) -> String {
        self.observer.clone()
    }

    /// Get the target.
    pub fn target(&self) -> String {
        self.target.clone()
    }

    /// Get the start date.
    pub fn start_date(&self) -> String {
        self.start_date.clone()
    }

    /// Get the duration.
    pub fn duration(&self) -> f64 {
        self.duration
    }

    /// Get the aberration correction.
    pub fn aberration_correction(&self) -> String {
        self.aberration_correction.clone()
    }

    /// Load the kernel of the system.
    pub fn load(&mut self) -> Result<(), crate::KernelError> {
        Ok(self.kernel.load()?)
    }

    /// Unload the kernel of the system.
    pub fn unload(&mut self) -> Result<(), crate::KernelError> {
        Ok(self.kernel.unload()?)
    }

    /// Get the time at the start.
    pub fn time_start(&self) -> f64 {
        crate::ephemeris_from_date(self.start_date())
    }

    /// Get the time at the end.
    pub fn time_end(&self) -> f64 {
        crate::ephemeris_from_date(self.start_date()) + self.duration
    }

    /// Get the position at the start.
    pub fn position_start(&self) -> Matrix3x1<f64> {
        let time = self.time_start();
        let (position, _) = crate::position(
            self.target(),
            time,
            self.frame(),
            self.aberration_correction(),
            self.observer(),
        );
        position
    }

    /// Get the position at the end.
    pub fn position_end(&self) -> Matrix3x1<f64> {
        let time = self.time_end();
        let (position, _) = crate::position(
            self.target(),
            time,
            self.frame(),
            self.aberration_correction(),
            self.observer(),
        );
        position
    }

    /// Get the number of points from start to end date with time step.
    pub fn number_points(&self, time_step: f64) -> usize {
        let time_start = self.time_start();
        let time_end = self.time_end();
        tool::size_range_with_step(time_start, time_end, time_step)
    }

    /// Get the times.
    pub fn times(&self, time_step: f64) -> Matrix1xX<f64> {
        let time_start = self.time_start();
        let time_end = self.time_end();
        tool::linspace(time_start, time_end, time_step)
    }

    /// Get the times formatted.
    pub fn times_formatted(&self, time_step: f64) -> Vec<String> {
        // Get times as ephemerides.
        let times = self.times(time_step);

        // Convert them into formatted string.
        times
            .iter()
            .map(|&time| crate::date_from_ephemeris(time, crate::TIME_FORMAT))
            .collect()
    }

    /// Get the positions from start to end with time step.
    pub fn positions(&self, time_step: f64) -> Matrix3xX<f64> {
        // Get times.
        let times = self.times(time_step);

        // Allocate positions matrix.
        let mut positions = Matrix3xX::zeros(times.len());

        // Get position at each time.
        for (time, mut position) in multizip((times.iter(), positions.column_iter_mut())) {
            position.copy_from(
                &crate::position(
                    self.target(),
                    *time,
                    self.frame(),
                    self.aberration_correction(),
                    self.observer(),
                )
                .0,
            );
        }

        positions
    }
}

/// Builder of the system type.
///
/// You might be interested in [`System`].
#[derive(Debug, Clone, Default)]
pub struct SystemBuilder {
    kernel: Option<crate::Kernel>,
    frame: Option<String>,
    observer: Option<String>,
    target: Option<String>,
    start_date: Option<String>,
    duration: Option<f64>,
    aberration_correction: Option<String>,
}

impl SystemBuilder {
    /// Set kernel from its name.
    pub fn kernel<P: Into<PathBuf>>(&mut self, file: P) -> Result<&mut Self, crate::KernelError> {
        self.kernel = Some(crate::Kernel::new(file)?);
        Ok(self)
    }

    /// Set reference frame.
    pub fn frame<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.frame = Some(name.into());
        self
    }

    /// Set observer.
    pub fn observer<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.observer = Some(name.into());
        self
    }

    /// Set target.
    pub fn target<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.target = Some(name.into());
        self
    }

    /// Set start date.
    pub fn start_date<S: Into<String>>(&mut self, date: S) -> &mut Self {
        self.start_date = Some(date.into());
        self
    }

    /// Set duration.
    pub fn duration(&mut self, time: f64) -> &mut Self {
        self.duration = Some(time);
        self
    }

    /// Set aberration correction.
    pub fn aberration_correction<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.aberration_correction = Some(name.into());
        self
    }

    /// Build to give System.
    pub fn build(&self) -> Result<System, SystemBuilderError> {
        match (
            self.kernel.as_ref(),
            self.frame.as_ref(),
            self.observer.as_ref(),
            self.target.as_ref(),
            self.start_date.as_ref(),
            self.duration,
            self.aberration_correction.as_ref(),
        ) {
            (
                Some(kernel),
                Some(frame),
                Some(observer),
                Some(target),
                Some(start),
                Some(duration),
                Some(aberration),
            ) => Ok(System {
                kernel: kernel.clone(),
                frame: frame.clone(),
                observer: observer.clone(),
                target: target.clone(),
                start_date: start.clone(),
                duration,
                aberration_correction: aberration.clone(),
            }),
            (None, _, _, _, _, _, _) => Err(SystemBuilderError {
                kind: SystemBuilderErrorKind::Kernel,
            }),
            (_, None, _, _, _, _, _) => Err(SystemBuilderError {
                kind: SystemBuilderErrorKind::Frame,
            }),
            (_, _, None, _, _, _, _) => Err(SystemBuilderError {
                kind: SystemBuilderErrorKind::Observer,
            }),
            (_, _, _, None, _, _, _) => Err(SystemBuilderError {
                kind: SystemBuilderErrorKind::Target,
            }),
            (_, _, _, _, None, _, _) => Err(SystemBuilderError {
                kind: SystemBuilderErrorKind::StartDate,
            }),
            (_, _, _, _, _, None, _) => Err(SystemBuilderError {
                kind: SystemBuilderErrorKind::Duration,
            }),
            (_, _, _, _, _, _, None) => Err(SystemBuilderError {
                kind: SystemBuilderErrorKind::AberrationCorrection,
            }),
        }
    }
}

#[macro_use]
#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn builder_description_kernel() -> Result<(), SystemError> {
        let system_res = SystemBuilder::default()
            .frame("J2000")
            .observer("HERA")
            .target("DIMORPHOS")
            .start_date("2027-MAR-23 16:00:00")
            .duration(129.0 * crate::DAY)
            .aberration_correction("NONE")
            .build();

        // Not need direct assert as the message might change in the future.
        assert_eq!(
            system_res.err().unwrap().description(),
            "the kernel must be initialized"
        );
        Ok(())
    }

    #[test]
    #[serial]
    fn builder_description_frame() -> Result<(), SystemError> {
        let system_res = SystemBuilder::default()
            .kernel("rsc/data/hera_PO_EMA_2024.tm")?
            .observer("HERA")
            .target("DIMORPHOS")
            .start_date("2027-MAR-23 16:00:00")
            .duration(129.0 * crate::DAY)
            .aberration_correction("NONE")
            .build();

        // Not need direct assert as the message might change in the future.
        assert_eq!(
            system_res.err().unwrap().description(),
            "the frame must be initialized"
        );
        Ok(())
    }

    #[test]
    #[serial]
    fn builder_description_observer() -> Result<(), SystemError> {
        let system_res = SystemBuilder::default()
            .kernel("rsc/data/hera_PO_EMA_2024.tm")?
            .frame("J2000")
            .target("DIMORPHOS")
            .start_date("2027-MAR-23 16:00:00")
            .duration(129.0 * crate::DAY)
            .aberration_correction("NONE")
            .build();

        // Not need direct assert as the message might change in the future.
        assert_eq!(
            system_res.err().unwrap().description(),
            "the observer must be initialized"
        );
        Ok(())
    }

    #[test]
    #[serial]
    fn builder_description_target() -> Result<(), SystemError> {
        let system_res = SystemBuilder::default()
            .kernel("rsc/data/hera_PO_EMA_2024.tm")?
            .frame("J2000")
            .observer("HERA")
            .start_date("2027-MAR-23 16:00:00")
            .duration(129.0 * crate::DAY)
            .aberration_correction("NONE")
            .build();

        // Not need direct assert as the message might change in the future.
        assert_eq!(
            system_res.err().unwrap().description(),
            "the target must be initialized"
        );
        Ok(())
    }

    #[test]
    #[serial]
    fn builder_description_start() -> Result<(), SystemError> {
        let system_res = SystemBuilder::default()
            .kernel("rsc/data/hera_PO_EMA_2024.tm")?
            .frame("J2000")
            .observer("HERA")
            .target("DIMORPHOS")
            .duration(129.0 * crate::DAY)
            .aberration_correction("NONE")
            .build();

        // Not need direct assert as the message might change in the future.
        assert_eq!(
            system_res.err().unwrap().description(),
            "the start date must be initialized"
        );
        Ok(())
    }

    #[test]
    #[serial]
    fn builder_description_duration() -> Result<(), SystemError> {
        let system_res = SystemBuilder::default()
            .kernel("rsc/data/hera_PO_EMA_2024.tm")?
            .frame("J2000")
            .observer("HERA")
            .target("DIMORPHOS")
            .start_date("2027-MAR-23 16:00:00")
            .aberration_correction("NONE")
            .build();

        // Not need direct assert as the message might change in the future.
        assert_eq!(
            system_res.err().unwrap().description(),
            "the duration must be initialized"
        );
        Ok(())
    }

    #[test]
    #[serial]
    fn builder_description_aberration() -> Result<(), SystemError> {
        let system_res = SystemBuilder::default()
            .kernel("rsc/data/hera_PO_EMA_2024.tm")?
            .frame("J2000")
            .observer("HERA")
            .target("DIMORPHOS")
            .start_date("2027-MAR-23 16:00:00")
            .duration(129.0 * crate::DAY)
            .build();

        // Not need direct assert as the message might change in the future.
        assert_eq!(
            system_res.err().unwrap().description(),
            "the aberration correction must be initialized"
        );
        Ok(())
    }
}
