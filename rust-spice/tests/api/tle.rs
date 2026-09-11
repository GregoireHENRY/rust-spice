//! Two-line element sets, and the SGP4 propagator that consumes them.

use crate::common;

/// A two-line element set, from the CSPICE documentation of `evsgp4_c`.
const TLE: [&str; 2] = [
    "1 43908U 18111AJ  20146.60805006  .00000806  00000-0  34965-4 0  9999",
    "2 43908  97.2676  47.2136 0020001 220.6050 139.3698 15.24999521 78544",
];

/// The geophysical constants NORAD's propagator expects, as recommended by the CSPICE
/// documentation: J2, J3, J4, KE, QO, SO, ER, AE.
const GEOPHS: [f64; 8] = [
    1.082616e-3,
    -2.53881e-6,
    -1.65597e-6,
    7.43669161e-2,
    120.0,
    78.0,
    6378.135,
    1.0,
];

#[test]
#[serial]
fn getelm() {
    common::load();

    let (epoch, elements) = spice::getelm(1957, &TLE);
    common::assert_ok("getelm");

    // The set is for day 146.60805006 of 2020, which the parse turns into seconds past J2000.
    let expected = spice::str2et("2020-146 // 14:35:35.525 UTC");
    assert_relative_eq!(epoch, expected, epsilon = 1.0);

    // Element 9 is the epoch again, and element 5 the eccentricity read off line 2.
    assert_relative_eq!(elements[9], epoch, epsilon = 1e-9);
    assert_relative_eq!(elements[5], 0.0020001, epsilon = 1e-9);
    // Element 3 is the inclination, 97.2676 degrees.
    assert_relative_eq!(elements[3].to_degrees(), 97.2676, epsilon = 1e-6);
    assert_eq!(elements.len(), spice::raw::TLE_NELTS);

    common::unload();
}

#[test]
#[serial]
fn evsgp4() {
    common::load();

    let (epoch, elements) = spice::getelm(1957, &TLE);
    let state = spice::evsgp4(epoch, GEOPHS, elements);
    common::assert_ok("evsgp4");

    // A sun-synchronous orbit a few hundred kilometres up: the radius sits just above the Earth,
    // and the speed near the circular value for that radius.
    let radius = spice::vnorm([state[0], state[1], state[2]]);
    let speed = spice::vnorm([state[3], state[4], state[5]]);
    assert!(
        (6600.0..7200.0).contains(&radius),
        "unexpected radius {radius} km"
    );
    assert_relative_eq!(
        speed,
        (spice::bodvrd("EARTH", "GM", 1)[0] / radius).sqrt(),
        epsilon = 0.2
    );

    // Half an orbit later the satellite is on the other side of the Earth.
    let period = std::f64::consts::TAU / (elements[8] / 60.0);
    let opposite = spice::evsgp4(epoch + period / 2.0, GEOPHS, elements);
    let separation = spice::vsep(
        [state[0], state[1], state[2]],
        [opposite[0], opposite[1], opposite[2]],
    );
    assert_relative_eq!(separation, std::f64::consts::PI, epsilon = 0.05);

    assert_eq!(GEOPHS.len(), spice::raw::TLE_NGEOPHS);

    common::unload();
}
