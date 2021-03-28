#[macro_use]
extern crate approx;
extern crate nalgebra as na;
extern crate spice;

mod core;
mod toolbox;

use std::ffi::CString;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_spicetools() {
    unsafe {
        let kernel = CString::new("rsc/data/hera_PO_EMA_2024.tm").unwrap();
        spice::furnsh_c(kernel.as_ptr() as *mut _);
        println!("kernel loaded successfully");
    }
}
