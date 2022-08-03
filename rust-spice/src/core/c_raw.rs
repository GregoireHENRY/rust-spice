pub fn erract(op: &str, actlen: i32, action: &str) -> () {
    #[allow(unused_unsafe)]
    unsafe {
        crate::c::erract_c(
            std::ffi::CString::new(op.to_string()).unwrap().into_raw(),
            actlen,
            std::ffi::CString::new(action.to_string())
                .unwrap()
                .into_raw(),
        );
    }
}


pub fn failed() -> bool {
    #[allow(unused_unsafe)]
    unsafe {
        crate::c::failed_c() != 0
    }
}

pub fn getmsg(option: &str, msglen: i32) -> String {
    let varout_0 = unsafe {
        libc::malloc(std::mem::size_of::<i8>() * (crate::MAX_LEN_OUT + 1) as usize)
            as *mut i8
    };
    #[allow(unused_unsafe)]
    unsafe {
        crate::c::getmsg_c(
            std::ffi::CString::new(option.to_string())
                .unwrap()
                .into_raw(),
            msglen,
            varout_0,
        );
        std::ffi::CStr::from_ptr(varout_0)
            .to_str()
            .unwrap()
            .to_string()
    }
}


pub fn reset() -> () {
    #[allow(unused_unsafe)]
    unsafe {
        crate::c::reset_c();
    }
}