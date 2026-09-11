//! Time conversions, including the spacecraft clock.

use crate::common;

#[test]
#[serial]
fn str2et() {
    common::load();

    // The J2000 epoch itself.
    assert_relative_eq!(
        spice::str2et("2000-JAN-01 12:00:00 TDB"),
        0.0,
        epsilon = 1e-9
    );
    assert_relative_eq!(
        spice::str2et("2000-JAN-02 12:00:00 TDB"),
        86400.0,
        epsilon = 1e-9
    );

    // UTC is the default, and trails TDB by 32.184 s plus the leap seconds.
    let utc = spice::str2et("2000-JAN-01 12:00:00");
    assert_relative_eq!(utc, 64.183_927_28, epsilon = 1e-6);

    common::unload();
}

#[test]
#[serial]
fn utc2et() {
    common::load();

    assert_relative_eq!(
        spice::utc2et("2000-JAN-01 12:00:00"),
        spice::str2et("2000-JAN-01 12:00:00"),
        epsilon = 1e-9
    );

    common::unload();
}

#[test]
#[serial]
fn et2utc() {
    common::load();

    let et = spice::str2et("2000-JAN-02 12:00:00");
    assert_eq!(spice::et2utc(et, "ISOC", 3), "2000-01-02T12:00:00.000");
    assert_eq!(spice::et2utc(et, "C", 0), "2000 JAN 02 12:00:00");
    assert_eq!(spice::et2utc(et, "J", 6), "JD 2451546.000000");

    common::unload();
}

#[test]
#[serial]
fn timout() {
    common::load();

    let et = spice::str2et("2027-MAR-23 16:00:00");
    assert_eq!(
        spice::timout(et, spice::TIME_FORMAT),
        "2027-MAR-23 16:00:00"
    );
    assert_eq!(spice::timout(et, "YYYY-MM-DD ::TRNC"), "2027-03-23");

    // The raw form is bounded by the length it is handed, and must not overrun it.
    assert_eq!(spice::raw::timout(et, spice::TIME_FORMAT, 5), "2027");

    common::unload();
}

#[test]
#[serial]
fn tparse() {
    common::load();

    // A formal calendar parse: no leap seconds, so the J2000 epoch lands on zero.
    let (seconds, error) = spice::tparse("2000-JAN-01 12:00:00");
    assert!(error.is_empty(), "unexpected error {error:?}");
    assert_relative_eq!(seconds, 0.0, epsilon = 1e-9);

    // An unparsable string reports why, rather than failing.
    let (_, error) = spice::tparse("not a date");
    assert!(!error.is_empty());

    common::unload();
}

#[test]
#[serial]
fn deltet() {
    common::load();

    // ET - UTC is 32.184 s plus the leap seconds, 32 of them at the J2000 epoch.
    let delta = spice::deltet(0.0, "ET");
    assert_relative_eq!(delta, 64.183_927_28, epsilon = 1e-6);

    // The two directions agree to within the periodic term.
    assert_relative_eq!(spice::deltet(0.0, "UTC"), delta, epsilon = 1e-3);

    common::unload();
}

#[test]
#[serial]
fn unitim() {
    common::load();

    // TDT is TAI plus 32.184 s, and TAI is TDB minus the periodic term.
    assert_relative_eq!(
        spice::unitim(0.0, "TDB", "TDB"),
        0.0,
        epsilon = f64::EPSILON
    );
    assert_relative_eq!(
        spice::unitim(0.0, "TDB", "JDTDB"),
        spice::j2000(),
        epsilon = 1e-9
    );
    // TAI trails TDB by 32.184 s plus a periodic term of a couple of milliseconds.
    assert_relative_eq!(spice::unitim(0.0, "ET", "TAI"), -32.184, epsilon = 1e-2);

    common::unload();
}

#[test]
#[serial]
fn lspcn() {
    common::load();

    // The solar longitude is an angle, so it lies in [0, 2*pi).
    let longitude = spice::lspcn("EARTH", common::EPOCH, "NONE");
    common::assert_ok("lspcn");
    assert!(
        (0.0..std::f64::consts::TAU).contains(&longitude),
        "got {longitude}"
    );

    common::unload();
}

#[test]
#[serial]
fn spacecraft_clock() {
    common::load();

    let et = common::EPOCH;
    let ticks = common::ticks(et);

    // The test clock runs at one millisecond per tick from the start of the coverage.
    assert_relative_eq!(spice::sce2c(common::SPACECRAFT, et), ticks, epsilon = 1e-6);
    assert_relative_eq!(spice::sct2e(common::SPACECRAFT, ticks), et, epsilon = 1e-6);

    // Encoding and decoding the string form round trips.
    let clock = spice::sce2s(common::SPACECRAFT, et);
    common::assert_ok("sce2s");
    assert!(!clock.is_empty());
    assert_relative_eq!(spice::scs2e(common::SPACECRAFT, &clock), et, epsilon = 1e-3);
    assert_relative_eq!(
        spice::scencd(common::SPACECRAFT, &clock),
        ticks,
        epsilon = 1.0
    );
    assert_eq!(spice::scdecd(common::SPACECRAFT, ticks), clock);

    common::unload();
}
