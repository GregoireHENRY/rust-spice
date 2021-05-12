use crate::{Kernel, KernelError};
use itertools::multizip;
use std::{path::PathBuf, vec::Vec};
use tool::{List, Vector, Vectors};

/// Enumeration of the different kinds of errors that can happen during the build of [`System`].
#[derive(Debug, thiserror::Error)]
pub enum SystemBuilderError {
    #[error("the parameter `{0}` is missing")]
    MissingParameter(String),
}

/// Enumeration of the different kinds of errors that can happen when using [`System`].
#[derive(Debug, thiserror::Error)]
pub enum SystemError {
    #[error("error during the build: {0}")]
    Build(#[from] SystemBuilderError),
    #[error("error related to kernel: {0}")]
    Kernel(#[from] KernelError),
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
    kernel: Kernel,
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
    pub fn kernel(&self) -> &Kernel {
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
    pub fn load(&mut self) -> Result<(), KernelError> {
        self.kernel.load()
    }

    /// Unload the kernel of the system.
    pub fn unload(&mut self) -> Result<(), KernelError> {
        self.kernel.unload()
    }

    /// Get the time at the start.
    pub fn time_start(&self) -> f64 {
        crate::str2et(self.start_date())
    }

    /// Get the time at the end.
    pub fn time_end(&self) -> f64 {
        crate::str2et(self.start_date()) + self.duration
    }

    /// Get the position at the start.
    pub fn position_start(&self) -> Vector<f64> {
        let time = self.time_start();
        let (position, _) = crate::spkpos(
            self.target(),
            time,
            self.frame(),
            self.aberration_correction(),
            self.observer(),
        );
        position
    }

    /// Get the position at the end.
    pub fn position_end(&self) -> Vector<f64> {
        let time = self.time_end();
        let (position, _) = crate::spkpos(
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
    pub fn times(&self, time_step: f64) -> List<f64> {
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
            .map(|&time| crate::timout(time, crate::TIME_FORMAT))
            .collect()
    }

    /// Get the positions from start to end with time step.
    pub fn positions(&self, time_step: f64) -> Vectors<f64> {
        // Get times.
        let times = self.times(time_step);

        // Allocate positions matrix.
        let mut positions = Vectors::zeros(times.len());

        // Get position at each time.
        for (time, mut position) in multizip((times.iter(), positions.column_iter_mut())) {
            position.copy_from(
                &crate::spkpos(
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
    kernel: Option<Kernel>,
    frame: Option<String>,
    observer: Option<String>,
    target: Option<String>,
    start_date: Option<String>,
    duration: Option<f64>,
    aberration_correction: Option<String>,
}

impl SystemBuilder {
    /// Set kernel from its name.
    pub fn kernel<P: Into<PathBuf>>(&mut self, file: P) -> Result<&mut Self, KernelError> {
        self.kernel = Some(Kernel::new(file)?);
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
        // Check for missing mandatory fields.
        let first_missing = tool::all_some!(
            self.kernel,
            self.frame,
            self.observer,
            self.target,
            self.start_date,
            self.duration,
            self.aberration_correction
        );
        if let Some(missing) = first_missing {
            return Err(SystemBuilderError::MissingParameter(missing));
        }

        Ok(System {
            kernel: self.kernel.clone().unwrap(),
            frame: self.frame.clone().unwrap(),
            observer: self.observer.clone().unwrap(),
            target: self.target.clone().unwrap(),
            start_date: self.start_date.clone().unwrap(),
            duration: self.duration.unwrap(),
            aberration_correction: self.aberration_correction.clone().unwrap(),
        })
    }
}
