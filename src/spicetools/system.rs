use itertools::multizip;
use na::{Matrix1xX, Matrix3x1, Matrix3xX};
use std::path::PathBuf;
use std::vec::Vec;

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
    pub fn load(&mut self) -> Result<(), crate::KernelAlreadyLoadedError> {
        Ok(self.kernel.load()?)
    }

    /// Unload the kernel of the system.
    pub fn unload(&mut self) -> Result<(), crate::KernelNotLoadedError> {
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
        crate::size_range_with_step(time_start, time_end, time_step)
    }

    /// Get the times.
    pub fn times(&self, time_step: f64) -> Matrix1xX<f64> {
        let time_start = self.time_start();
        let time_end = self.time_end();
        crate::linspace(time_start, time_end, time_step)
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
    pub fn kernel<P: Into<PathBuf>>(
        &mut self,
        file: P,
    ) -> Result<&mut Self, crate::KernelAlreadyLoadedError> {
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
    pub fn build(&self) -> Result<System, String> {
        Ok(System {
            kernel: Clone::clone(self.kernel.as_ref().ok_or("kernel must be initialized.")?),
            frame: Clone::clone(self.frame.as_ref().ok_or("frame must be initialized.")?),
            observer: Clone::clone(
                self.observer
                    .as_ref()
                    .ok_or("observer must be initialized.")?,
            ),
            target: Clone::clone(self.target.as_ref().ok_or("target must be initialized.")?),
            start_date: Clone::clone(
                self.start_date
                    .as_ref()
                    .ok_or("start_date must be initialized.")?,
            ),
            duration: Clone::clone(
                self.duration
                    .as_ref()
                    .ok_or("duration must be initialized.")?,
            ),
            aberration_correction: Clone::clone(
                self.aberration_correction
                    .as_ref()
                    .ok_or("aberration_correction must be initialized.")?,
            ),
        })
    }
}
