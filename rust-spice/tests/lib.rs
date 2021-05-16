#[macro_use]
extern crate approx;
extern crate nalgebra as na;
extern crate spice;
#[macro_use]
extern crate serial_test;

mod core;

use std::ffi::CString;

#[test]
#[serial]
fn test_c() {
    unsafe {
        let kernel = CString::new("rsc/krn/hera_study_PO_EMA_2024.tm")
            .unwrap()
            .into_raw();
        spice::c::furnsh_c(kernel);
        spice::c::unload_c(kernel);
    }
}
