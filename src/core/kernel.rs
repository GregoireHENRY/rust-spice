use std::ffi::CString;

pub fn load<S: AsRef<str>>(name: S) {
    unsafe {
        let kernel = CString::new(name.as_ref().to_string()).unwrap();
        crate::furnsh_c(kernel.as_ptr() as *mut _);
    }
}

pub fn unload<S: AsRef<str>>(name: S) {
    unsafe {
        let kernel = CString::new(name.as_ref().to_string()).unwrap();
        crate::unload_c(kernel.as_ptr() as *mut _);
    }
}
