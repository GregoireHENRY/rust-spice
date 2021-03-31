#[macro_use]
extern crate approx;
extern crate nalgebra as na;
extern crate spice;
#[macro_use]
extern crate serial_test;

mod core;
mod spicetools;
mod toolbox;

use std::ffi::CString;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[serial]
fn test_spice_load() {
    unsafe {
        let kernel = CString::new("rsc/data/hera_PO_EMA_2024.tm")
            .unwrap()
            .into_raw();
        spice::c::furnsh_c(kernel);
        spice::c::unload_c(kernel);
    }
}
