use na::Matrix3x1;
use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::vec::Vec;

impl crate::System {
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
            crate::system_new(
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
            crate::system_load_kernel(self);
        }
    }

    pub fn unload(&mut self) {
        unsafe {
            crate::system_unload_kernel(self);
        }
    }

    pub fn time_start(&mut self) -> f64 {
        unsafe { crate::system_get_time_start(self) }
    }

    pub fn time_end(&mut self) -> f64 {
        unsafe { crate::system_get_time_end(self) }
    }

    pub fn position_start(&mut self) -> Matrix3x1<f64> {
        let position = unsafe {
            let ptr = crate::system_get_position_start(self);
            Vec::from_raw_parts(ptr, 3, 3)
        };
        Matrix3x1::new(position[0], position[1], position[2])
    }

    pub fn position_end(&mut self) -> Matrix3x1<f64> {
        let position = unsafe {
            let ptr = crate::system_get_position_end(self);
            Vec::from_raw_parts(ptr, 3, 3)
        };
        Matrix3x1::new(position[0], position[1], position[2])
    }

    pub fn number_points(&mut self, time_step: f64) -> usize {
        unsafe {
            crate::system_get_number_points(self, time_step)
                .try_into()
                .unwrap()
        }
    }
}
