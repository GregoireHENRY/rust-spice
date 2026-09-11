//! Orientation kernels.

use crate::assert_matrix_eq;
use crate::common;

const IDENTITY: [[f64; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

#[test]
#[serial]
fn ckgp() {
    common::load();

    let ticks = common::ticks(common::EPOCH);
    let (matrix, clock, found) = spice::ckgp(common::INSTRUMENT, ticks, 0.0, "J2000");
    common::assert_ok("ckgp");

    assert!(found);
    assert_matrix_eq(&matrix, &IDENTITY, 1e-15);
    assert_relative_eq!(clock, ticks, epsilon = 1e-6);

    // Outside the coverage, with no tolerance, there is no pointing.
    let outside = common::ticks(common::LAST + 86400.0);
    assert!(!spice::ckgp(common::INSTRUMENT, outside, 0.0, "J2000").2);

    common::unload();
}

#[test]
#[serial]
fn ckgpav() {
    common::load();

    let ticks = common::ticks(common::EPOCH);
    let (matrix, angular_velocity, clock, found) =
        spice::ckgpav(common::INSTRUMENT, ticks, 0.0, "J2000");
    common::assert_ok("ckgpav");

    assert!(found);
    assert_matrix_eq(&matrix, &IDENTITY, 1e-15);
    // The test CK holds a constant attitude.
    crate::assert_slice_eq(&angular_velocity, &[0.0; 3], 1e-15);
    assert_relative_eq!(clock, ticks, epsilon = 1e-6);

    // The pointing agrees with what `ckgp` reports.
    assert_matrix_eq(
        &matrix,
        &spice::ckgp(common::INSTRUMENT, ticks, 0.0, "J2000").0,
        0.0,
    );

    common::unload();
}

#[test]
#[serial]
fn ckobj_and_ckcov() {
    common::load();

    let ck = common::kernel("test.bc");

    assert_eq!(spice::ckobj(&ck).to_vec(), vec![common::INSTRUMENT]);

    let coverage = spice::ckcov(&ck, common::INSTRUMENT, false, "SEGMENT", 0.0, "SCLK");
    common::assert_ok("ckcov");
    assert_eq!(coverage.len(), 2);
    assert_relative_eq!(
        coverage.get(0).unwrap(),
        common::ticks(common::FIRST),
        epsilon = 1.0
    );
    assert_relative_eq!(
        coverage.get(1).unwrap(),
        common::ticks(common::LAST),
        epsilon = 1.0
    );

    // The same window, expressed in ephemeris seconds.
    let coverage = spice::ckcov(&ck, common::INSTRUMENT, false, "SEGMENT", 0.0, "TDB");
    assert_relative_eq!(coverage.get(0).unwrap(), common::FIRST, epsilon = 1e-3);
    assert_relative_eq!(coverage.get(1).unwrap(), common::LAST, epsilon = 1e-3);

    // An instrument the file knows nothing about has no coverage.
    assert!(spice::ckcov(&ck, -1000, false, "SEGMENT", 0.0, "SCLK").is_empty());

    common::unload();
}

#[test]
#[serial]
fn write_and_read_back() {
    common::load();

    let path = common::kernel("written.bc");
    let _ = std::fs::remove_file(&path);

    let instrument = -999_100;
    let ticks = (0..8)
        .map(|index| index as f64 * 1000.0)
        .collect::<Vec<_>>();
    // A quarter turn about the third axis, as a SPICE quaternion.
    let half = std::f64::consts::FRAC_PI_4;
    let quaternions = vec![[half.cos(), 0.0, 0.0, half.sin()]; ticks.len()];
    let angular_velocities = vec![[0.0; 3]; ticks.len()];

    let handle = spice::ckopn(&path, "rust-spice written CK", 0);
    common::assert_ok("ckopn");
    // A DAF opened for writing gets a negative handle.
    assert!(handle != 0);
    spice::ckw03(
        handle,
        ticks[0],
        ticks[ticks.len() - 1],
        instrument,
        "J2000",
        true,
        "WRITTEN",
        ticks.len() as i32,
        &ticks,
        &quaternions,
        &angular_velocities,
        1,
        &ticks[..1],
    );
    spice::ckcls(handle);
    common::assert_ok("ckw03");
    assert!(std::path::Path::new(&path).exists());

    assert_eq!(spice::ckobj(&path).to_vec(), vec![instrument]);

    spice::furnsh(&path);
    let (matrix, _, found) = spice::ckgp(instrument, 2000.0, 0.0, "J2000");
    common::assert_ok("reading back the written CK");
    assert!(found);
    // A SPICE quaternion (cos(t/2), sin(t/2) * axis) gives the matrix that rotates coordinates
    // by -t about the axis, so this is the inverse of `rotate(t, 3)`.
    assert_matrix_eq(&matrix, &spice::xpose(spice::rotate(2.0 * half, 3)), 1e-12);

    common::unload();
    let _ = std::fs::remove_file(&path);
}

#[test]
#[serial]
#[should_panic(expected = "ckw03 was asked for")]
fn ckw03_rejects_a_count_longer_than_its_arrays() {
    common::load();

    spice::ckw03(
        1,
        0.0,
        1.0,
        common::INSTRUMENT,
        "J2000",
        false,
        "TOO SHORT",
        8,
        &[0.0, 1.0],
        &[[1.0, 0.0, 0.0, 0.0]; 2],
        &[],
        1,
        &[0.0],
    );
}
