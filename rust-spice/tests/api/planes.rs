//! Planes, ellipses, and the ellipsoid geometry built on them.

use crate::assert_slice_eq;
use crate::common;

/// The plane `z = 2`, built three different ways in the tests below.
const NORMAL: [f64; 3] = [0.0, 0.0, 1.0];
const CONSTANT: f64 = 2.0;

#[test]
#[serial]
fn planes_round_trip() {
    common::reset();

    // A plane keeps a *unit* normal, so the constant scales with it.
    let plane = spice::nvc2pl([0.0, 0.0, 3.0], 6.0);
    let (normal, constant) = spice::pl2nvc(plane);
    assert_slice_eq(&normal, &NORMAL, 1e-15);
    assert_relative_eq!(constant, CONSTANT, epsilon = 1e-15);

    // The three constructors describe the same plane.
    let from_point = spice::nvp2pl(NORMAL, [5.0, -7.0, CONSTANT]);
    let (normal, constant) = spice::pl2nvc(from_point);
    assert_slice_eq(&normal, &NORMAL, 1e-15);
    assert_relative_eq!(constant, CONSTANT, epsilon = 1e-15);

    let from_spans = spice::psv2pl([0.0, 0.0, CONSTANT], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let (normal, constant) = spice::pl2nvc(from_spans);
    assert_slice_eq(&normal, &NORMAL, 1e-15);
    assert_relative_eq!(constant, CONSTANT, epsilon = 1e-15);

    // And the three decompositions agree with each other.
    let (normal, point) = spice::pl2nvp(plane);
    assert_slice_eq(&normal, &NORMAL, 1e-15);
    assert_relative_eq!(spice::vdot(normal, point), CONSTANT, epsilon = 1e-14);

    let (point, span1, span2) = spice::pl2psv(plane);
    assert_relative_eq!(spice::vdot(NORMAL, point), CONSTANT, epsilon = 1e-14);
    for span in [span1, span2] {
        assert_relative_eq!(spice::vdot(NORMAL, span), 0.0, epsilon = 1e-14);
        assert_relative_eq!(spice::vnorm(span), 1.0, epsilon = 1e-14);
    }
    common::assert_ok("plane conversions");
}

#[test]
#[serial]
fn ellipses_round_trip() {
    common::reset();

    // A circle of radius 3 in the z = 0 plane, given by two generating vectors that are neither
    // orthogonal nor axis aligned.
    let ellipse = spice::cgv2el([1.0, 2.0, 3.0], [3.0, 0.0, 0.0], [0.0, 3.0, 0.0]);
    let (center, major, minor) = spice::el2cgv(ellipse);

    assert_slice_eq(&center, &[1.0, 2.0, 3.0], 1e-15);
    assert_relative_eq!(spice::vnorm(major), 3.0, epsilon = 1e-14);
    assert_relative_eq!(spice::vnorm(minor), 3.0, epsilon = 1e-14);
    // The semi-axes of an ellipse are always orthogonal.
    assert_relative_eq!(spice::vdot(major, minor), 0.0, epsilon = 1e-14);

    // `saelgv` is the same computation without the ellipse wrapper.
    let (smajor, sminor) = spice::saelgv([3.0, 0.0, 0.0], [0.0, 3.0, 0.0]);
    assert_relative_eq!(spice::vnorm(smajor), 3.0, epsilon = 1e-14);
    assert_relative_eq!(spice::vnorm(sminor), 3.0, epsilon = 1e-14);

    // For a genuine ellipse the major axis is the longer one.
    let (smajor, sminor) = spice::saelgv([5.0, 0.0, 0.0], [0.0, 2.0, 0.0]);
    assert_relative_eq!(spice::vnorm(smajor), 5.0, epsilon = 1e-14);
    assert_relative_eq!(spice::vnorm(sminor), 2.0, epsilon = 1e-14);
    common::assert_ok("ellipse conversions");
}

#[test]
#[serial]
fn intersections() {
    common::reset();

    // A sphere of radius 5 cut by z = 3 gives a circle of radius 4.
    let plane = spice::nvc2pl(NORMAL, 3.0);
    let (section, found) = spice::inedpl(5.0, 5.0, 5.0, plane);
    assert!(found);
    let (center, major, minor) = spice::el2cgv(section);
    assert_slice_eq(&center, &[0.0, 0.0, 3.0], 1e-13);
    assert_relative_eq!(spice::vnorm(major), 4.0, epsilon = 1e-13);
    assert_relative_eq!(spice::vnorm(minor), 4.0, epsilon = 1e-13);

    // A plane beyond the sphere misses it.
    assert!(!spice::inedpl(5.0, 5.0, 5.0, spice::nvc2pl(NORMAL, 9.0)).1);
    common::assert_ok("ellipsoid and plane");

    // A circle of radius 2 about the origin in the x-z plane meets y = 0 at two points.
    let circle = spice::cgv2el([0.0; 3], [2.0, 0.0, 0.0], [0.0, 0.0, 2.0]);
    let (count, first, second) = spice::inelpl(circle, spice::nvc2pl([0.0, 0.0, 1.0], 0.0));
    assert_eq!(count, 2);
    assert_relative_eq!(spice::vnorm(first), 2.0, epsilon = 1e-14);
    assert_relative_eq!(spice::vnorm(second), 2.0, epsilon = 1e-14);
    assert_relative_eq!(first[2], 0.0, epsilon = 1e-14);

    // A ray along +z meets z = 2 once, at (0, 0, 2).
    let (count, point) = spice::inrypl([0.0; 3], NORMAL, spice::nvc2pl(NORMAL, CONSTANT));
    assert_eq!(count, 1);
    assert_slice_eq(&point, &[0.0, 0.0, CONSTANT], 1e-14);

    // A ray in the plane's own direction never meets it.
    assert_eq!(
        spice::inrypl([0.0; 3], [1.0, 0.0, 0.0], spice::nvc2pl(NORMAL, CONSTANT)).0,
        0
    );
    common::assert_ok("ray and plane");
}

#[test]
#[serial]
fn projections() {
    common::reset();

    let plane = spice::nvc2pl(NORMAL, CONSTANT);

    // Projecting drops the component along the normal and lands on the plane.
    let projected = spice::vprjp([1.0, 2.0, 9.0], plane);
    assert_slice_eq(&projected, &[1.0, 2.0, CONSTANT], 1e-14);

    // Inverting the projection back onto a tilted plane finds the point that projects there.
    let tilted = spice::nvc2pl([0.0, 1.0, 1.0], 0.0);
    let (inverted, found) = spice::vprjpi(projected, plane, tilted);
    assert!(found);
    assert_relative_eq!(
        spice::vdot(spice::vhat([0.0, 1.0, 1.0]), inverted),
        0.0,
        epsilon = 1e-13
    );
    assert_slice_eq(&spice::vprjp(inverted, plane), &projected, 1e-13);

    // Projecting a circle in the x-y plane onto a plane tilted 45 degrees squashes one axis.
    let circle = spice::cgv2el([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let shadow = spice::pjelpl(circle, tilted);
    let (_, major, minor) = spice::el2cgv(shadow);
    assert_relative_eq!(spice::vnorm(major), 1.0, epsilon = 1e-14);
    assert_relative_eq!(spice::vnorm(minor), 0.5f64.sqrt(), epsilon = 1e-14);
    common::assert_ok("projections");
}

#[test]
#[serial]
fn nearest_points() {
    common::reset();

    // The point of a unit circle in the x-y plane nearest (5, 0, 0).
    let circle = spice::cgv2el([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let (near, distance) = spice::npelpt([5.0, 0.0, 0.0], circle);
    assert_slice_eq(&near, &[1.0, 0.0, 0.0], 1e-14);
    assert_relative_eq!(distance, 4.0, epsilon = 1e-14);

    // The point of a line nearest the origin, for a line through (0, 3, 0) along +x.
    let (near, distance) = spice::nplnpt([0.0, 3.0, 0.0], [1.0, 0.0, 0.0], [0.0; 3]);
    assert_slice_eq(&near, &[0.0, 3.0, 0.0], 1e-14);
    assert_relative_eq!(distance, 3.0, epsilon = 1e-14);

    // The point of a sphere of radius 2 nearest a line that passes 5 away from its centre.
    let (near, distance) = spice::npedln(2.0, 2.0, 2.0, [0.0, 5.0, 0.0], [1.0, 0.0, 0.0]);
    assert_slice_eq(&near, &[0.0, 2.0, 0.0], 1e-13);
    assert_relative_eq!(distance, 3.0, epsilon = 1e-13);
    common::assert_ok("nearest points");
}

#[test]
#[serial]
fn ellipsoid_surface() {
    common::reset();

    // On a sphere the outward normal is the radial direction.
    let normal = spice::surfnm(3.0, 3.0, 3.0, [3.0, 0.0, 0.0]);
    assert_slice_eq(&normal, &[1.0, 0.0, 0.0], 1e-15);

    // On a genuine ellipsoid it is not, but it still points outward.
    let point = spice::surfpt([10.0, 10.0, 10.0], [-1.0, -1.0, -1.0], 3.0, 2.0, 1.0).0;
    let normal = spice::surfnm(3.0, 2.0, 1.0, point);
    assert_relative_eq!(spice::vnorm(normal), 1.0, epsilon = 1e-14);
    assert!(spice::vdot(normal, point) > 0.0);

    // The limb of a sphere seen from far away is a great circle, near enough.
    let limb = spice::edlimb(3.0, 3.0, 3.0, [1000.0, 0.0, 0.0]);
    let (center, major, minor) = spice::el2cgv(limb);
    assert_relative_eq!(spice::vnorm(major), 3.0, epsilon = 1e-3);
    assert_relative_eq!(spice::vnorm(minor), 3.0, epsilon = 1e-3);
    assert!(spice::vnorm(center) < 1e-2, "the limb is nearly centred");

    // `surfpv` carries a velocity through the intercept `surfpt` finds.
    let state = [10.0, 0.0, 0.0, 0.0, 1.0, 0.0];
    let direction = [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let (intercept, found) = spice::surfpv(state, direction, 3.0, 3.0, 3.0);
    assert!(found);
    assert_slice_eq(&intercept[..3], &[3.0, 0.0, 0.0], 1e-13);
    common::assert_ok("ellipsoid surface");
}
