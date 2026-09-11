//! The routines that only return a constant, plus the toolkit version.

use std::f64::consts;

#[test]
#[serial]
fn angles() {
    assert_relative_eq!(spice::pi(), consts::PI, epsilon = f64::EPSILON);
    assert_relative_eq!(spice::twopi(), consts::TAU, epsilon = f64::EPSILON);
    assert_relative_eq!(spice::halfpi(), consts::FRAC_PI_2, epsilon = f64::EPSILON);
    assert_relative_eq!(spice::dpr(), 180.0 / consts::PI, epsilon = 1e-12);
    assert_relative_eq!(spice::rpd(), consts::PI / 180.0, epsilon = f64::EPSILON);
    assert_relative_eq!(spice::dpr() * spice::rpd(), 1.0, epsilon = f64::EPSILON);
}

#[test]
#[serial]
fn durations() {
    assert_relative_eq!(spice::spd(), 86400.0, epsilon = f64::EPSILON);
    assert_relative_eq!(spice::jyear(), 365.25 * 86400.0, epsilon = f64::EPSILON);
    // The tropical year is a little shorter than the Julian one.
    assert!(spice::tyear() < spice::jyear());
    assert_relative_eq!(spice::tyear(), 31_556_925.974_7, epsilon = 1e-3);
    assert_relative_eq!(spice::clight(), 299_792.458, epsilon = 1e-9);
}

#[test]
#[serial]
fn epochs() {
    // Julian dates, in days.
    assert_relative_eq!(spice::j2000(), 2_451_545.0, epsilon = f64::EPSILON);
    assert_relative_eq!(spice::j1900(), 2_415_020.0, epsilon = f64::EPSILON);
    assert_relative_eq!(spice::j1950(), 2_433_282.5, epsilon = f64::EPSILON);
    assert_relative_eq!(spice::j2100(), 2_488_070.0, epsilon = f64::EPSILON);
    assert_relative_eq!(spice::b1900(), 2_415_020.313_52, epsilon = 1e-5);
    assert_relative_eq!(spice::b1950(), 2_433_282.42345905, epsilon = 1e-8);

    // A Julian century separates J2000 from J2100.
    assert_relative_eq!(
        spice::j2100() - spice::j2000(),
        36_525.0,
        epsilon = f64::EPSILON
    );
}

#[test]
#[serial]
fn tkvrsn() {
    let version = spice::tkvrsn("TOOLKIT");
    assert!(
        version.starts_with("CSPICE_N"),
        "unexpected version {version:?}"
    );
}
