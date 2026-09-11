//! Two body orbital elements and propagation.

use crate::assert_slice_eq;
use crate::common;

#[test]
#[serial]
fn oscelt() {
    common::load();

    // The fixture's orbit is circular and equatorial, by construction.
    let state = common::analytic_state(0.0);
    let elements = spice::oscelt(state, 0.0, common::GM_CENTER);
    common::assert_ok("oscelt");

    assert_relative_eq!(
        elements[0],
        common::ORBIT_RADIUS,
        epsilon = 1.0,
        max_relative = 1e-9
    );
    assert_relative_eq!(elements[1], 0.0, epsilon = 1e-12, max_relative = 1e-9);
    assert_relative_eq!(elements[2], 0.0, epsilon = 1e-12);
    assert_relative_eq!(elements[6], 0.0, epsilon = 1e-12, max_relative = 1e-12);
    assert_relative_eq!(elements[7], common::GM_CENTER, epsilon = 1e-3);

    common::unload();
}

#[test]
#[serial]
fn oscltx() {
    common::load();

    let state = common::analytic_state(0.0);
    let short = spice::oscelt(state, 0.0, common::GM_CENTER);
    let long = spice::oscltx(state, 0.0, common::GM_CENTER);
    common::assert_ok("oscltx");

    // The first eight elements are the ones `oscelt` returns.
    assert_slice_eq(&long[..8], &short, 0.0);

    // Element 9 is the semi-major axis and element 10 the period, for a closed orbit.
    assert_relative_eq!(
        long[9],
        common::ORBIT_RADIUS,
        epsilon = 1.0,
        max_relative = 1e-9
    );
    let period = std::f64::consts::TAU * (common::ORBIT_RADIUS.powi(3) / common::GM_CENTER).sqrt();
    assert_relative_eq!(long[10], period, epsilon = 1e-3, max_relative = 1e-9);

    common::unload();
}

#[test]
#[serial]
fn conics() {
    common::load();

    // Elements back to a state: the round trip returns where it started.
    let state = common::analytic_state(0.0);
    let elements = spice::oscelt(state, 0.0, common::GM_CENTER);
    let recovered = spice::conics(elements, 0.0);
    common::assert_ok("conics");
    assert_slice_eq(&recovered, &state, 1e-6);

    // And propagating the elements matches the analytic orbit a day later.
    let later = spice::conics(elements, 86400.0);
    assert_slice_eq(&later, &common::analytic_state(86400.0), 1e-3);

    common::unload();
}

#[test]
#[serial]
fn prop2b() {
    common::load();

    let state = common::analytic_state(0.0);
    let propagated = spice::prop2b(common::GM_CENTER, state, 86400.0);
    common::assert_ok("prop2b");
    assert_slice_eq(&propagated, &common::analytic_state(86400.0), 1e-3);

    // A zero step is the identity.
    assert_slice_eq(&spice::prop2b(common::GM_CENTER, state, 0.0), &state, 1e-9);

    common::unload();
}
