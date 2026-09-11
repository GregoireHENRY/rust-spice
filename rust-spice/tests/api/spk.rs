//! Ephemerides: reading them, and writing them back out.

use crate::assert_slice_eq;
use crate::common;

#[test]
#[serial]
fn spkpos() {
    common::load();

    let et = common::EPOCH;
    let (position, light_time) = spice::spkpos("EARTH", et, "J2000", "NONE", "SUN");
    common::assert_ok("spkpos");

    // The interpolated position matches the analytic orbit the segment was sampled from.
    assert_slice_eq(&position, &common::analytic_state(et)[..3], 1e-3);

    // With no aberration correction, the light time is just the range over c.
    assert_relative_eq!(
        light_time,
        spice::vnorm(position) / spice::clight(),
        epsilon = 1e-12
    );

    // Swapping observer and target negates the position.
    let (reverse, _) = spice::spkpos("SUN", et, "J2000", "NONE", "EARTH");
    assert_slice_eq(&reverse, &spice::vminus(position), 1e-9);

    common::unload();
}

#[test]
#[serial]
fn spkezr() {
    common::load();

    let et = common::EPOCH;
    let (state, light_time) = spice::spkezr("EARTH", et, "J2000", "NONE", "SUN");
    common::assert_ok("spkezr");

    assert_slice_eq(&state, &common::analytic_state(et), 1e-3);
    assert_relative_eq!(
        light_time,
        spice::vnorm([state[0], state[1], state[2]]) / spice::clight(),
        epsilon = 1e-12
    );

    // The position agrees with `spkpos`.
    let (position, _) = spice::spkpos("EARTH", et, "J2000", "NONE", "SUN");
    assert_slice_eq(&state[..3], &position, 1e-12);

    // States compose: the satellite seen from the Sun is the sum of the two segments.
    let (satellite_from_sun, _) = spice::spkezr("MOON", et, "J2000", "NONE", "SUN");
    let (satellite_from_target, _) = spice::spkezr("MOON", et, "J2000", "NONE", "EARTH");
    let sum = (0..6)
        .map(|i| state[i] + satellite_from_target[i])
        .collect::<Vec<_>>();
    assert_slice_eq(&satellite_from_sun, &sum, 1e-9);

    common::unload();
}

#[test]
#[serial]
fn spkez_and_spkezp() {
    common::load();

    let et = common::EPOCH;
    let (by_name, lt_name) = spice::spkezr("EARTH", et, "J2000", "LT+S", "SUN");
    let (by_code, lt_code) = spice::spkez(common::TARGET, et, "J2000", "LT+S", common::CENTER);
    assert_slice_eq(&by_name, &by_code, 0.0);
    assert_relative_eq!(lt_name, lt_code, epsilon = 0.0);

    let (position, lt_position) =
        spice::spkezp(common::TARGET, et, "J2000", "LT+S", common::CENTER);
    assert_slice_eq(&position, &by_code[..3], 0.0);
    assert_relative_eq!(lt_position, lt_code, epsilon = 0.0);

    common::unload();
}

#[test]
#[serial]
fn spkgeo() {
    common::load();

    let et = common::EPOCH;
    // The geometric state is what an uncorrected `spkez` returns.
    let (geometric, lt) = spice::spkgeo(common::TARGET, et, "J2000", common::CENTER);
    let (uncorrected, lt_uncorrected) =
        spice::spkez(common::TARGET, et, "J2000", "NONE", common::CENTER);
    assert_slice_eq(&geometric, &uncorrected, 0.0);
    assert_relative_eq!(lt, lt_uncorrected, epsilon = 0.0);

    common::unload();
}

#[test]
#[serial]
fn spkcpo_and_spkcpt() {
    common::load();

    let et = common::EPOCH;
    // An observer sitting at the centre of body 399, in its body-fixed frame, is body 399.
    let (state, _) = spice::spkcpo(
        "SUN",
        et,
        "J2000",
        "OBSERVER",
        "NONE",
        [0.0; 6],
        "EARTH",
        "IAU_EARTH",
    );
    common::assert_ok("spkcpo");
    let (reference, _) = spice::spkezr("SUN", et, "J2000", "NONE", "EARTH");
    assert_slice_eq(&state[..3], &reference[..3], 1e-6);

    // Likewise for a target at the centre of body 399.
    let (state, _) = spice::spkcpt(
        [0.0; 3],
        "EARTH",
        "IAU_EARTH",
        et,
        "J2000",
        "OBSERVER",
        "NONE",
        "SUN",
    );
    common::assert_ok("spkcpt");
    let (reference, _) = spice::spkezr("EARTH", et, "J2000", "NONE", "SUN");
    assert_slice_eq(&state[..3], &reference[..3], 1e-6);

    common::unload();
}

#[test]
#[serial]
fn spkcvo_and_spkcvt() {
    common::load();

    let et = common::EPOCH;
    let (state, _) = spice::spkcvo(
        "SUN", et, "J2000", "OBSERVER", "NONE", [0.0; 6], et, "EARTH", "J2000",
    );
    common::assert_ok("spkcvo");
    let (reference, _) = spice::spkezr("SUN", et, "J2000", "NONE", "EARTH");
    assert_slice_eq(&state, &reference, 1e-6);

    let (state, _) = spice::spkcvt(
        [0.0; 6], et, "EARTH", "J2000", et, "J2000", "OBSERVER", "NONE", "SUN",
    );
    common::assert_ok("spkcvt");
    let (reference, _) = spice::spkezr("EARTH", et, "J2000", "NONE", "SUN");
    assert_slice_eq(&state, &reference, 1e-6);

    common::unload();
}

#[test]
#[serial]
fn spkobj_and_spkcov() {
    common::load();

    let spk = common::kernel("test.bsp");

    let mut ids = spice::spkobj(&spk).to_vec();
    ids.sort_unstable();
    assert_eq!(ids, vec![common::CENTER, common::SATELLITE, common::TARGET]);

    let coverage = spice::spkcov(&spk, common::TARGET);
    assert_eq!(coverage.len(), 2, "one interval, so two endpoints");
    assert_relative_eq!(coverage.get(0).unwrap(), common::FIRST, epsilon = 1e-6);
    assert_relative_eq!(coverage.get(1).unwrap(), common::LAST, epsilon = 1e-6);

    // A body the file knows nothing about has an empty coverage.
    assert!(spice::spkcov(&spk, 499).is_empty());

    common::unload();
}

#[test]
#[serial]
fn write_and_read_back() {
    common::load();

    let path = common::kernel("written.bsp");
    let _ = std::fs::remove_file(&path);

    let epochs = (0..16).map(|index| index as f64 * 60.0).collect::<Vec<_>>();
    let states = epochs
        .iter()
        .map(|&et| [et, 2.0 * et, 3.0 * et, 1.0, 2.0, 3.0])
        .collect::<Vec<_>>();

    let handle = spice::spkopn(&path, "rust-spice written SPK", 0);
    common::assert_ok("spkopn");
    // A DAF opened for writing gets a negative handle.
    assert!(handle != 0);
    spice::spkw09(
        handle,
        common::SPACECRAFT,
        common::TARGET,
        "J2000",
        epochs[0],
        epochs[epochs.len() - 1],
        "WRITTEN",
        3,
        states.len() as i32,
        &states,
        &epochs,
    );
    spice::spkcls(handle);
    common::assert_ok("spkw09");
    assert!(std::path::Path::new(&path).exists());

    // Re-open for addition, to exercise `spkopa`, then close again.
    let handle = spice::spkopa(&path);
    common::assert_ok("spkopa");
    assert!(handle != 0);
    spice::spkcls(handle);

    spice::furnsh(&path);
    let (state, _) = spice::spkezr("TEST_SPACECRAFT", 300.0, "J2000", "NONE", "EARTH");
    common::assert_ok("reading back the written SPK");
    assert_slice_eq(&state, &[300.0, 600.0, 900.0, 1.0, 2.0, 3.0], 1e-6);

    common::unload();
    let _ = std::fs::remove_file(&path);
}

#[test]
#[serial]
#[should_panic(expected = "spkw09 was asked for")]
fn spkw09_rejects_a_count_longer_than_its_arrays() {
    common::load();

    // CSPICE would read past the end of both arrays; the wrapper refuses instead.
    spice::spkw09(
        1,
        common::SPACECRAFT,
        common::TARGET,
        "J2000",
        0.0,
        1.0,
        "TOO SHORT",
        3,
        8,
        &[[0.0; 6]; 2],
        &[0.0, 1.0],
    );
}
