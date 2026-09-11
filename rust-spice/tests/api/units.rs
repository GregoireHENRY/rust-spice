//! Unit conversion, numeric limits, hexadecimal, string position and the frame/body lookups.

use crate::common;

#[test]
#[serial]
fn convrt() {
    common::reset();

    assert_relative_eq!(spice::convrt(1.0, "KM", "M"), 1000.0, epsilon = 1e-9);
    assert_relative_eq!(
        spice::convrt(1.0, "DAYS", "SECONDS"),
        86400.0,
        epsilon = 1e-9
    );
    assert_relative_eq!(
        spice::convrt(180.0, "DEGREES", "RADIANS"),
        std::f64::consts::PI,
        epsilon = 1e-12
    );
    // Round trip through an awkward pair.
    assert_relative_eq!(
        spice::convrt(spice::convrt(3.5, "AU", "KM"), "KM", "AU"),
        3.5,
        epsilon = 1e-12
    );
    common::assert_ok("convrt");
}

#[test]
#[serial]
fn numeric_limits() {
    common::reset();

    assert!(spice::dpmax() > 1e300);
    assert!(spice::dpmin() < -1e300);
    assert_relative_eq!(spice::dpmin(), -spice::dpmax(), epsilon = 0.0);
}

#[test]
#[serial]
fn hexadecimal() {
    common::reset();

    // A double goes to the portable hex form and back unchanged.
    let (hex, length) = spice::raw::dp2hx(1.0 / 3.0, 64);
    assert_eq!(length as usize, hex.len());
    let (back, error, message) = spice::raw::hx2dp(&hex, 64);
    assert!(!error, "unexpected error {message:?}");
    assert_relative_eq!(back, 1.0 / 3.0, epsilon = 0.0);

    // And a malformed string is reported rather than guessed at.
    let (_, error, message) = spice::raw::hx2dp("not hex", 128);
    assert!(error);
    assert!(!message.is_empty());
    common::assert_ok("hexadecimal conversion");
}

#[test]
#[serial]
fn string_positions() {
    common::reset();

    // `pos` searches forward from an index, `posr` backward.
    assert_eq!(spice::pos("alpha beta alpha", "alpha", 0), 0);
    assert_eq!(spice::pos("alpha beta alpha", "alpha", 1), 11);
    assert_eq!(spice::posr("alpha beta alpha", "alpha", 15), 11);
    assert_eq!(spice::pos("alpha", "zeta", 0), -1);

    // `cpos` looks for any one of a set of characters.
    assert_eq!(spice::cpos("alpha beta", "ea", 0), 0);
    assert_eq!(spice::cpos("alpha beta", " ", 0), 5);
    assert_eq!(spice::cposr("alpha beta", "a", 9), 9);
    assert_eq!(spice::cpos("alpha", "xyz", 0), -1);
}

#[test]
#[serial]
fn etcal_and_tparch() {
    common::load();

    // `etcal` formats without needing a leapseconds kernel, so it reports the TDB calendar date.
    assert_eq!(spice::raw::etcal(0.0, 64), "2000 JAN 01 12:00:00.000");

    // Restricting the parser makes an out of range component an error rather than a rollover.
    spice::tparch("YES");
    let (_, message) = spice::tparse("1998 MAR 32 12:29:57");
    assert!(!message.is_empty(), "an impossible day should be rejected");
    spice::tparch("NO");
    common::assert_ok("tparch");

    common::unload();
}

#[test]
#[serial]
fn frame_lookups() {
    common::load();

    // A body-fixed frame, found from the body it is centred on, by ID and by name.
    let (code, name, found) = spice::raw::cidfrm(399, spice::MAX_LEN_OUT as i32);
    assert!(found);
    assert_eq!(name, "IAU_EARTH");
    assert_eq!(code, spice::namfrm("IAU_EARTH"));

    let (code_by_name, name_by_name, found) =
        spice::raw::cnmfrm("EARTH", spice::MAX_LEN_OUT as i32);
    assert!(found);
    assert_eq!((code_by_name, name_by_name), (code, name));

    // And back the other way, from the frame class and class ID.
    let (frcode, frname, center, found) = spice::raw::ccifrm(2, 399, spice::MAX_LEN_OUT as i32);
    assert!(found);
    assert_eq!(frcode, code);
    assert_eq!(frname, "IAU_EARTH");
    assert_eq!(center, 399);

    // Nothing is found for a body with no associated frame.
    assert!(!spice::raw::cnmfrm("NO SUCH BODY", spice::MAX_LEN_OUT as i32).2);
    common::assert_ok("frame lookups");

    common::unload();
}

#[test]
#[serial]
fn built_in_frames() {
    common::reset();

    // Class 2 is the body-fixed frames; the built in set is not empty and holds IAU_EARTH.
    let mut ids = spice::Cell::<i32>::new(2048);
    spice::bltfrm(2, &mut ids);
    common::assert_ok("bltfrm");

    assert!(!ids.is_empty());
    assert!(ids.to_vec().contains(&spice::namfrm("IAU_EARTH")));

    common::unload();
}

#[test]
#[serial]
fn pool_watches() {
    common::load();

    // An agent is notified the first time it asks, then only when the variables change.
    spice::swpool("TEST_AGENT", &["BODY399_RADII"]);
    common::assert_ok("swpool");
    assert!(
        spice::cvpool("TEST_AGENT"),
        "the first check always reports an update"
    );
    assert!(!spice::cvpool("TEST_AGENT"));

    spice::pdpool("BODY399_RADII", &[1.0, 2.0, 3.0]);
    assert!(
        spice::cvpool("TEST_AGENT"),
        "writing the watched variable notifies the agent"
    );
    assert!(!spice::cvpool("TEST_AGENT"));

    // And the names of the watched variables can be listed back out of the pool.
    let names = spice::gnpool("BODY399_*", 0, 16);
    common::assert_ok("gnpool");
    assert!(names.contains(&"BODY399_RADII".to_string()));
    assert!(spice::gnpool("NO_SUCH_*", 0, 16).is_empty());

    common::unload();
}

#[test]
#[serial]
fn bodvar() {
    common::load();

    // The deprecated form returns the same values as the two that replaced it.
    let radii = spice::bodvar(399, "RADII", 3);
    assert_eq!(radii, spice::bodvrd("EARTH", "RADII", 3));
    assert_eq!(radii, spice::bodvcd(399, "RADII", 3));
    common::assert_ok("bodvar");

    common::unload();
}

#[test]
#[serial]
fn badkpv() {
    common::load();

    // BODY399_RADII holds three numbers, so asking for exactly three succeeds...
    assert!(!spice::badkpv("test", "BODY399_RADII", "=", 3, 1, 'N'));
    // ... and asking for four is reported as bad.
    assert!(spice::badkpv("test", "BODY399_RADII", "=", 4, 1, 'N'));
    spice::errors::reset();

    common::unload();
}
