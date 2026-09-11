/*!
Integration tests for the safe API.

Every wrapped routine is exercised at least once. Where CSPICE documents a value, or the fixture
makes one analytically predictable, the test asserts that value; otherwise it asserts that the
wrapper agrees, bit for bit, with a direct call to the C routine through [`spice::c`], which is
what catches a marshalling mistake.
*/

#![cfg(not(feature = "lock"))]

#[macro_use]
extern crate approx;
#[macro_use]
extern crate serial_test;

#[path = "../common/mod.rs"]
mod common;

mod against_c;
mod bodies;
mod cells;
mod ck;
mod constants;
mod coords;
mod dsk;
mod errors;
mod frames;
mod geometry;
mod gf;
mod orbits;
mod planes;
mod pool;
mod rotations;
mod spk;
mod strings;
mod time;
mod tle;
mod units;
mod vectors;

/// Assert that two slices of floats agree to within `epsilon`.
#[track_caller]
pub fn assert_slice_eq(left: &[f64], right: &[f64], epsilon: f64) {
    assert_eq!(left.len(), right.len(), "lengths differ");
    for (index, (left, right)) in left.iter().zip(right).enumerate() {
        assert_relative_eq!(left, right, epsilon = epsilon);
        let _ = index;
    }
}

/// Assert that two 3x3 matrices agree to within `epsilon`.
#[track_caller]
pub fn assert_matrix_eq(left: &[[f64; 3]; 3], right: &[[f64; 3]; 3], epsilon: f64) {
    for (left, right) in left.iter().zip(right) {
        assert_slice_eq(left, right, epsilon);
    }
}

/// Build the null terminated string the C API wants, kept alive by the caller.
pub fn cs(value: &str) -> std::ffi::CString {
    std::ffi::CString::new(value).unwrap()
}

/// Shorthand for handing a [`cs`] to a C routine.
#[macro_export]
macro_rules! cptr {
    ($value:expr) => {
        $value.as_ptr() as *mut spice::c::SpiceChar
    };
}
