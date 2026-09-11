//! The assorted routines: state derivatives, ellipsoid helpers, lexing, the error stack and the
//! deprecated geometry entry points.

use crate::common;
use crate::{assert_matrix_eq, assert_slice_eq};

/// Two states, chosen so nothing lines up with an axis.
const S1: [f64; 6] = [1.0, 2.0, 3.0, 0.1, -0.2, 0.3];
const S2: [f64; 6] = [-4.0, 5.0, 6.0, 0.4, 0.5, -0.6];

#[test]
#[serial]
fn state_derivatives() {
    common::reset();

    // The position halves agree with the plain vector routines...
    let cross = spice::dvcrss(S1, S2);
    assert_slice_eq(
        &cross[..3],
        &spice::vcrss([1.0, 2.0, 3.0], [-4.0, 5.0, 6.0]),
        1e-14,
    );
    assert_relative_eq!(spice::dvdot(S1, S2), 0.0, epsilon = 1e6);
    assert_relative_eq!(
        spice::dvnorm(S1),
        spice::vdot(spice::vhat([1.0, 2.0, 3.0]), [0.1, -0.2, 0.3]),
        epsilon = 1e-14
    );

    let hat = spice::dvhat(S1);
    assert_slice_eq(&hat[..3], &spice::vhat([1.0, 2.0, 3.0]), 1e-14);
    assert_relative_eq!(spice::vnorm([hat[0], hat[1], hat[2]]), 1.0, epsilon = 1e-14);

    // ... and the unit cross product is the normalised cross product.
    assert_slice_eq(
        &spice::ucrss([1.0, 2.0, 3.0], [-4.0, 5.0, 6.0]),
        &spice::vhat(spice::vcrss([1.0, 2.0, 3.0], [-4.0, 5.0, 6.0])),
        1e-14,
    );
    let unit_cross = spice::ducrss(S1, S2);
    assert_slice_eq(
        &unit_cross[..3],
        &spice::ucrss([1.0, 2.0, 3.0], [-4.0, 5.0, 6.0]),
        1e-14,
    );

    // The separation derivative of a state with itself is zero, since the angle never changes.
    assert_relative_eq!(spice::dvsep(S1, S1), 0.0, epsilon = 1e-12);
    assert!(spice::dvsep(S1, S2).is_finite());

    common::assert_ok("the state derivatives");
}

#[test]
#[serial]
fn ellipsoid_helpers() {
    common::reset();

    // On a sphere of radius 3 the outward normal at (3,0,0) is +x, so `ednmpt` maps back to it.
    assert_slice_eq(
        &spice::ednmpt(3.0, 3.0, 3.0, [1.0, 0.0, 0.0]),
        &[3.0, 0.0, 0.0],
        1e-13,
    );

    // `edpnt` scales a point onto the ellipsoid along the ray from the origin.
    let on = spice::edpnt([1.0, 1.0, 1.0], 3.0, 2.0, 1.0);
    let scaled = (on[0] / 3.0).powi(2) + (on[1] / 2.0).powi(2) + on[2].powi(2);
    assert_relative_eq!(scaled, 1.0, epsilon = 1e-13);

    // The nearest point of a sphere to a moving observer, and its altitude. The flag says whether
    // the *rates* are meaningful, which they are here because the observer is moving.
    let (near, alt, rates_valid) = spice::dnearp([10.0, 0.0, 0.0, 0.0, 1.0, 0.0], 3.0, 3.0, 3.0);
    assert!(rates_valid);
    assert_slice_eq(&near[..3], &[3.0, 0.0, 0.0], 1e-12);
    assert_relative_eq!(alt[0], 7.0, epsilon = 1e-12);

    // Expanding a plate outward keeps its plane but grows it.
    let plate = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    let bigger = spice::pltexp(plate, 1.0);
    let before = spice::vnorm(spice::vsub(plate[1], plate[0]));
    let after = spice::vnorm(spice::vsub(bigger[1], bigger[0]));
    assert_relative_eq!(after, 2.0 * before, epsilon = 1e-13);

    common::assert_ok("the ellipsoid helpers");
}

#[test]
#[serial]
fn rotations_and_quaternion_rates() {
    common::reset();

    // A state transformation with no angular velocity inverts to the transpose of its rotation.
    let rotation = spice::eul2m(0.3, -0.7, 1.1, 3, 2, 1);
    let xform = spice::rav2xf(rotation, [0.0; 3]);
    let inverse = spice::invstm(xform);
    assert_matrix_eq(&spice::xf2rav(inverse).0, &spice::xpose(rotation), 1e-14);

    // Rotating a vector about an axis, the other way round from `vrotv`: this rotates the frame.
    assert_slice_eq(
        &spice::rotvec([1.0, 0.0, 0.0], std::f64::consts::FRAC_PI_2, 3),
        &[0.0, -1.0, 0.0],
        1e-15,
    );

    // A constant quaternion has no angular velocity.
    let q = spice::m2q(rotation);
    assert_slice_eq(&spice::qdq2av(q, [0.0; 4]), &[0.0; 3], 1e-14);

    // `twovxf` is `twovec` carried through to states.
    let frame = spice::twovxf(
        [1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        1,
        [0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
        2,
    );
    assert_matrix_eq(&spice::xf2rav(frame).0, &spice::ident(), 1e-14);

    common::assert_ok("the rotation helpers");
}

#[test]
#[serial]
fn lexing_and_parsing() {
    common::reset();

    assert!(spice::iswhsp("   \t "));
    assert!(!spice::iswhsp(" a "));
    assert_eq!(spice::lastnb("abc   "), 2);

    // `ncpos` finds the first character *not* in the set.
    assert_eq!(spice::ncpos("  abc", " ", 0), 2);
    assert_eq!(spice::ncposr("abc  ", " ", 4), 2);
    assert_eq!(spice::ncpos("aaa", "a", 0), -1);

    // The lexers return the last index of the token and how many characters it spans.
    assert_eq!(spice::lx4uns("12345 rest", 0), (4, 5));
    assert_eq!(spice::lx4sgn("-123 rest", 0), (3, 4));
    assert_eq!(spice::lx4dec("-12.5 rest", 0), (4, 5));
    assert_eq!(spice::lx4num("-12.5E3 rest", 0), (6, 7));
    // A quoted string, with the quote character doubled to escape it.
    assert_eq!(spice::lxqstr("'alpha' rest", '\'', 0), (6, 7));

    // And the parsers turn text into numbers.
    assert_relative_eq!(spice::prsdp(" 3.25 "), 3.25, epsilon = 1e-14);
    assert_eq!(spice::prsint(" -42 "), -42);

    common::assert_ok("lexing and parsing");
}

#[test]
#[serial]
fn quadratic_roots() {
    common::reset();

    // x^2 - 3x + 2 has the real roots 1 and 2, reported as (real, imaginary) pairs.
    let (r1, r2) = spice::rquad(1.0, -3.0, 2.0);
    assert_relative_eq!(r1[1], 0.0, epsilon = 1e-14);
    assert_relative_eq!(r2[1], 0.0, epsilon = 1e-14);
    let mut roots = [r1[0], r2[0]];
    roots.sort_by(f64::total_cmp);
    assert_slice_eq(&roots, &[1.0, 2.0], 1e-13);

    // x^2 + 1 has a conjugate pair.
    let (r1, r2) = spice::rquad(1.0, 0.0, 1.0);
    assert_relative_eq!(r1[0], 0.0, epsilon = 1e-14);
    assert_relative_eq!(r1[1], 1.0, epsilon = 1e-14);
    assert_relative_eq!(r2[1], -1.0, epsilon = 1e-14);

    common::assert_ok("rquad");
}

#[test]
#[serial]
fn the_error_stack() {
    common::load();
    spice::errors::quiet();

    // The traceback follows the routines that check in and out.
    assert_eq!(spice::trcdep(), 0);
    spice::chkin("outer");
    spice::chkin("inner");
    assert_eq!(spice::trcdep(), 2);
    assert!(spice::errors::qcktrc().contains("inner"));
    spice::chkout("inner");
    spice::chkout("outer");
    assert_eq!(spice::trcdep(), 0);

    // A message can be built up with markers and then signalled.
    assert!(!spice::return_c(), "nothing has failed yet");
    spice::setmsg("Something went wrong with #");
    spice::errch("#", "the widget");
    spice::sigerr("SPICE(TESTERROR)");
    assert!(spice::errors::failed());
    assert!(spice::return_c(), "a failed state tells routines to return");

    let error = spice::errors::check().expect_err("the error was signalled");
    assert_eq!(error.short, "SPICE(TESTERROR)");
    assert!(error.long.contains("the widget"), "got {:?}", error.long);

    common::unload();
}

#[test]
#[serial]
fn pool_and_body_names() {
    common::load();

    // A body can be given a name at run time, without a kernel.
    spice::boddef("TEST_ASTEROID", -999_777);
    assert_eq!(spice::bodn2c("TEST_ASTEROID"), (-999_777, true));
    assert_eq!(spice::bodc2n(-999_777), ("TEST_ASTEROID".to_string(), true));

    // And a pool variable can be deleted.
    spice::pdpool("TEST_DOUBLES", &[1.0]);
    assert!(spice::expool("TEST_DOUBLES"));
    spice::dvpool("TEST_DOUBLES");
    assert!(!spice::expool("TEST_DOUBLES"));
    common::assert_ok("boddef and dvpool");

    common::unload();
}

#[test]
#[serial]
fn clocks_and_years() {
    common::load();

    // `sce2t` rounds to the nearest tick, `sce2c` keeps the fraction.
    let et = common::EPOCH;
    assert_relative_eq!(
        spice::sce2t(common::SPACECRAFT, et),
        common::ticks(et),
        epsilon = 1.0
    );
    assert_relative_eq!(
        spice::sctiks(common::SPACECRAFT, "1:000"),
        1000.0,
        epsilon = 1e-6
    );

    // The two digit year cutoff is settable; put it back afterwards.
    spice::tsetyr(1980);
    spice::tsetyr(1969);
    common::assert_ok("the clock helpers");

    common::unload();
}

#[test]
#[serial]
fn deprecated_geometry() {
    common::load();

    let et = common::EPOCH;

    // `subpt` and `subsol` are what `subpnt` and `subslr` replaced.
    let (point, alt) = spice::subpt("Near point", "EARTH", et, "NONE", "MOON");
    common::assert_ok("subpt");
    let (modern, _, _) = spice::subpnt(
        "Near point: ellipsoid",
        "EARTH",
        et,
        "IAU_EARTH",
        "NONE",
        "MOON",
    );
    assert_slice_eq(&point, &modern, 1e-6);
    assert!(alt > 0.0);

    let solar = spice::subsol("Near point", "EARTH", et, "NONE", "MOON");
    let (modern, _, _) = spice::subslr(
        "Near point: ellipsoid",
        "EARTH",
        et,
        "IAU_EARTH",
        "NONE",
        "MOON",
    );
    assert_slice_eq(&solar, &modern, 1e-6);

    // `illum` is what `ilumin` replaced.
    let (phase, solar_angle, emission) = spice::illum("EARTH", et, "NONE", "MOON", point);
    let (_, _, p2, s2, e2) =
        spice::ilumin("Ellipsoid", "EARTH", et, "IAU_EARTH", "NONE", "MOON", point);
    assert_relative_eq!(phase, p2, epsilon = 1e-9);
    assert_relative_eq!(solar_angle, s2, epsilon = 1e-9);
    assert_relative_eq!(emission, e2, epsilon = 1e-9);

    // And `srfxpt` is what `sincpt` replaced.
    let (to_target, _) = spice::spkpos("EARTH", et, "J2000", "NONE", "SUN");
    let (spoint, _, _, _, found) =
        spice::srfxpt("Ellipsoid", "EARTH", et, "NONE", "SUN", "J2000", to_target);
    assert!(found);
    let (modern, _, _, found) = spice::sincpt(
        "Ellipsoid",
        "EARTH",
        et,
        "IAU_EARTH",
        "NONE",
        "SUN",
        "J2000",
        to_target,
    );
    assert!(found);
    assert_slice_eq(&spoint, &modern, 1e-6);
    common::assert_ok("the deprecated geometry entry points");

    common::unload();
}

#[test]
#[serial]
fn observation_geometry() {
    common::load();

    let et = common::EPOCH;

    // Light time to and from a target, one the negative of the other.
    let (forward, lt) = spice::ltime(et, 399, "->", 10);
    common::assert_ok("ltime");
    assert_relative_eq!(forward, et + lt, epsilon = 1e-9);
    let (backward, lt2) = spice::ltime(et, 399, "<-", 10);
    assert_relative_eq!(backward, et - lt2, epsilon = 1e-9);

    // Stellar aberration displaces an apparent position, and the inverse form undoes it.
    let (position, _) = spice::spkpos("EARTH", et, "J2000", "NONE", "SUN");
    let (state, _) = spice::spkezr("EARTH", et, "J2000", "NONE", "SUN");
    let velocity = [state[3], state[4], state[5]];
    let apparent = spice::stelab(position, velocity);
    assert!(spice::vsep(apparent, position) > 0.0);
    let restored = spice::stlabx(apparent, velocity);
    for (a, b) in restored.iter().zip(&position) {
        assert_relative_eq!(a, b, max_relative = 1e-9);
    }

    // The angular separation of two bodies, and the same pair as points.
    let separation = spice::trgsep(
        et, "EARTH", "POINT", "NULL", "MOON", "POINT", "NULL", "SUN", "NONE",
    );
    common::assert_ok("trgsep");
    assert!((0.0..std::f64::consts::PI).contains(&separation));

    // Azimuth and elevation of a target from a fixed position.
    let (state, _) = spice::azlcpo(
        "ELLIPSOID",
        "MOON",
        et,
        "NONE",
        true,
        true,
        [0.0, 0.0, 6378.0],
        "EARTH",
        "IAU_EARTH",
    );
    common::assert_ok("azlcpo");
    assert!(state[0] > 0.0, "the range should be positive");

    common::unload();
}

#[test]
#[serial]
fn body_orientation() {
    common::load();

    let et = common::EPOCH;

    // `tipbod` is the body-fixed rotation, which is what `pxform` reports.
    let matrix = spice::tipbod("J2000", 399, et);
    common::assert_ok("tipbod");
    assert_matrix_eq(&matrix, &spice::pxform("J2000", "IAU_EARTH", et), 1e-12);

    // `tisbod` carries it to states, so its rotation block matches.
    let state = spice::tisbod("J2000", 399, et);
    assert_matrix_eq(&spice::xf2rav(state).0, &matrix, 1e-12);

    // A text kernel frame reports the fixed rotation to its relative frame.
    let (rotation, relative, found) = spice::tkfram(common::FIXED_FRAME);
    common::assert_ok("tkfram");
    assert!(found);
    assert_eq!(relative, 1, "TEST_FIXED is defined relative to J2000");
    // The matrix takes positions from the TK frame to its base frame, so it is the transpose of
    // the one `pxform` reports for the other direction.
    assert_matrix_eq(
        &rotation,
        &spice::xpose(spice::pxform("J2000", "TEST_FIXED", et)),
        1e-14,
    );

    common::unload();
}

#[test]
#[serial]
fn tangent_point() {
    common::load();

    let et = common::EPOCH;
    // A ray from the Sun that misses the Earth has a tangent point above its surface.
    let (to_target, _) = spice::spkpos("EARTH", et, "J2000", "NONE", "SUN");
    let offset = spice::vadd(to_target, spice::vscl(20000.0, [0.0, 0.0, 1.0]));

    let (tangent, altitude, range, surface, _, _) = spice::tangpt(
        "ELLIPSOID",
        "EARTH",
        et,
        "IAU_EARTH",
        "NONE",
        "TANGENT POINT",
        "SUN",
        "J2000",
        offset,
    );
    common::assert_ok("tangpt");

    assert!(
        altitude > 0.0,
        "the ray misses, so the tangent point is above the surface"
    );
    assert!(range > 0.0);
    assert!(spice::vnorm(tangent) > spice::vnorm(surface));

    common::unload();
}

#[test]
#[serial]
fn equinoctial_elements() {
    common::reset();

    // A circular equatorial orbit in equinoctial elements: only the semi-major axis and the mean
    // longitude rate are non-zero.
    let a: f64 = 1.0e4;
    let mu: f64 = 398600.435436;
    let n = (mu / a.powi(3)).sqrt();
    // a, h, k, mean longitude, p, q, d(periapse)/dt, d(mean longitude)/dt, d(node)/dt.
    let elements = [a, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, n, 0.0];

    let state = spice::eqncpv(0.0, 0.0, elements, 0.0, std::f64::consts::FRAC_PI_2);
    common::assert_ok("eqncpv");
    assert_relative_eq!(
        spice::vnorm([state[0], state[1], state[2]]),
        a,
        epsilon = 1.0
    );
    assert_relative_eq!(
        spice::vnorm([state[3], state[4], state[5]]),
        (mu / a).sqrt(),
        epsilon = 1e-3
    );
}

#[test]
#[serial]
fn ftncls() {
    common::reset();
    // Closing a unit that was never opened is a no-op rather than an error.
    spice::ftncls(99);
    common::assert_ok("ftncls");
}
