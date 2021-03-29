use na::{Matrix1xX, Matrix3x1, Matrix3xX};
use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::vec::Vec;

pub type System = crate::c::System;

impl System {
    pub fn new<S: AsRef<str>>(
        kernel: S,
        frame: S,
        observer: S,
        target: S,
        start_date: S,
        duration: f64,
        abcorr: S,
    ) -> Self {
        let kernel_ptr = CString::new(kernel.as_ref()).unwrap().into_raw();
        let frame_ptr = CString::new(frame.as_ref()).unwrap().into_raw();
        let observer_ptr = CString::new(observer.as_ref()).unwrap().into_raw();
        let target_ptr = CString::new(target.as_ref()).unwrap().into_raw();
        let start_date_ptr = CString::new(start_date.as_ref()).unwrap().into_raw();
        let abcorr_ptr = CString::new(abcorr.as_ref()).unwrap().into_raw();
        unsafe {
            crate::c::system_new(
                kernel_ptr,
                frame_ptr,
                observer_ptr,
                target_ptr,
                start_date_ptr,
                duration,
                abcorr_ptr,
            )
        }
    }

    pub fn kernel(&mut self) -> &str {
        unsafe { CStr::from_ptr(self.KERNEL).to_str().unwrap() }
    }

    pub fn frame(&mut self) -> &str {
        unsafe { CStr::from_ptr(self.FRAME).to_str().unwrap() }
    }

    pub fn observer(&mut self) -> &str {
        unsafe { CStr::from_ptr(self.OBSERVER).to_str().unwrap() }
    }

    pub fn target(&mut self) -> &str {
        unsafe { CStr::from_ptr(self.TARGET).to_str().unwrap() }
    }

    pub fn start_date(&mut self) -> &str {
        unsafe { CStr::from_ptr(self.START_DATE).to_str().unwrap() }
    }

    pub fn duration(&mut self) -> f64 {
        self.DURATION
    }

    pub fn abcorr(&mut self) -> &str {
        unsafe { CStr::from_ptr(self.ABCORR).to_str().unwrap() }
    }

    pub fn load(&mut self) {
        unsafe {
            crate::c::system_load_kernel(self);
        }
    }

    pub fn unload(&mut self) {
        unsafe {
            crate::c::system_unload_kernel(self);
        }
    }

    pub fn time_start(&mut self) -> f64 {
        unsafe { crate::c::system_get_time_start(self) }
    }

    pub fn time_end(&mut self) -> f64 {
        unsafe { crate::c::system_get_time_end(self) }
    }

    pub fn position_start(&mut self) -> Matrix3x1<f64> {
        let position = unsafe {
            let ptr = crate::c::system_get_position_start(self);
            Vec::from_raw_parts(ptr, 3, 3)
        };
        Matrix3x1::new(position[0], position[1], position[2])
    }

    pub fn position_end(&mut self) -> Matrix3x1<f64> {
        let position = unsafe {
            let ptr = crate::c::system_get_position_end(self);
            Vec::from_raw_parts(ptr, 3, 3)
        };
        Matrix3x1::new(position[0], position[1], position[2])
    }

    pub fn number_points(&mut self, time_step: f64) -> usize {
        unsafe {
            crate::c::system_get_number_points(self, time_step)
                .try_into()
                .unwrap()
        }
    }

    pub fn times(&mut self, time_step: f64) -> Matrix1xX<f64> {
        let size = self.number_points(time_step);
        let times = unsafe {
            let ptr = crate::c::system_get_times(self, time_step);
            Vec::from_raw_parts(ptr, size, size)
        };
        Matrix1xX::from_iterator(size, times.iter().cloned())
    }

    pub fn times_formatted(&mut self, time_step: f64) -> Matrix1xX<&'static str> {
        let size = self.number_points(time_step);
        unsafe {
            let ptr = crate::c::system_get_times_formatted(self, time_step);
            let times = Vec::from_raw_parts(ptr, size, size);
            Matrix1xX::from_iterator(
                size,
                times
                    .iter()
                    .cloned()
                    .map(|time| CStr::from_ptr(time).to_str().unwrap()),
            )
        }
    }

    pub fn positions(&mut self, time_step: f64) -> Matrix3xX<f64> {
        let size = self.number_points(time_step);
        let positions = unsafe {
            let ptr = crate::c::system_get_positions(self, time_step);
            Vec::from_raw_parts(ptr, 3 * size, 3 * size)
        };
        Matrix3xX::from_iterator(size, positions.iter().cloned())
    }
}
