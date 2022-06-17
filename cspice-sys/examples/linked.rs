#![allow(unused_unsafe)]
use cspice_sys::*;

// TODO actual const, macro unchecked? think
// Using strings needs to be less painful
/*#[repr(transparent)]
pub struct SString(pub *mut i8);

impl SString {
    pub fn new<T: Into<Vec<u8>>>(t: T) -> Result<Self, std::ffi::NulError> {
        Ok(Self(std::ffi::CString::new::<T>(t)?.into_raw()))
    }
}

impl Drop for SString {
    fn drop(&mut self) {
        unsafe { std::ffi::CString::from_raw(self.0); }
    }
}*/

pub struct SString(Vec<i8>);

impl SString {
    pub fn new<T: Into<Vec<u8>>>(t: T) -> Self {
        let mut v = t.into();
        if let Some(n) = v.iter().position(|&x| x == b'\0') {
            v.truncate(n + 1);
        } else {
            v.push(b'\0');
        }

        let mut v = std::mem::ManuallyDrop::new(v);

        Self(unsafe { Vec::from_raw_parts(v.as_mut_ptr() as *mut i8, v.len(), v.capacity()) })
    }

    pub fn with_size(s: usize) -> Self {
        Self(vec![0; s])
    }

    pub fn as_cs(&mut self) -> *mut i8 {
        self.0.as_mut_ptr()
    }

    pub fn len(&self) -> i32 {
        self.0.len() as i32
    }
}

macro_rules! cspice_err {
    ($i:expr) => {{
        let r = $i;
        if unsafe { failed_c() } != 0 {
            // TODO getmsg_c
            unsafe {
                reset_c();
            }
            Err(())
        } else {
            Ok(r)
        }
    }};
}

fn main() {
    unsafe {
        let mut name = SString::new("kernels/all.tm"); //.unwrap();
        let mut set = SString::new("set"); //.unwrap();
        let mut act = SString::new("return"); //.unwrap();
        erract_c(set.as_cs(), 0, act.as_cs());
        cspice_err!(furnsh_c(name.as_cs())).unwrap();
        if let Err(_) = cspice_err!(furnsh_c(std::ptr::null_mut() as _)) {
            println!("Oops...");
        }
        cspice_err!(unload_c(name.as_cs())).unwrap();
    }
}
