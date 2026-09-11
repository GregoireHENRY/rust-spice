//! Surface geometry: intercepts, sub-points, illumination and fields of view.

use crate::assert_slice_eq;
use crate::common;

#[test]
#[serial]
fn sincpt() {
    common::load();

    let et = common::EPOCH;
    // Look from the Sun straight at the centre of the target.
    let (to_target, _) = spice::spkpos("EARTH", et, "J2000", "NONE", "SUN");
    let (point, epoch, observer_to_point, found) = spice::sincpt(
        "Ellipsoid",
        "EARTH",
        et,
        "IAU_EARTH",
        "NONE",
        "SUN",
        "J2000",
        to_target,
    );
    common::assert_ok("sincpt");

    assert!(found);
    assert_relative_eq!(epoch, et, epsilon = 1e-9);

    // The intercept sits on the ellipsoid.
    let radii = spice::bodvrd("EARTH", "RADII", 3);
    let scaled = (0..3).map(|i| (point[i] / radii[i]).powi(2)).sum::<f64>();
    assert_relative_eq!(scaled, 1.0, epsilon = 1e-9);

    // And the observer-to-point vector has the length one would expect.
    assert_relative_eq!(
        spice::vnorm(observer_to_point),
        spice::vnorm(to_target) - radii[0],
        epsilon = 1e-3
    );

    // Looking the other way misses.
    assert!(
        !spice::sincpt(
            "Ellipsoid",
            "EARTH",
            et,
            "IAU_EARTH",
            "NONE",
            "SUN",
            "J2000",
            spice::vminus(to_target),
        )
        .3
    );

    common::unload();
}

#[test]
#[serial]
fn sincpt_on_the_dsk() {
    common::load();

    let et = common::EPOCH;
    let (to_target, _) = spice::spkpos("EARTH", et, "J2000", "NONE", "SUN");
    let (point, _, _, found) = spice::sincpt(
        "DSK/UNPRIORITIZED",
        "EARTH",
        et,
        "IAU_EARTH",
        "NONE",
        "SUN",
        "J2000",
        to_target,
    );
    common::assert_ok("sincpt on the DSK");

    assert!(found);
    // Every point of the octahedron satisfies |x| + |y| + |z| = r.
    let sum = point.iter().map(|component| component.abs()).sum::<f64>();
    assert_relative_eq!(sum, common::SHAPE_RADIUS, epsilon = 1e-6);

    common::unload();
}

#[test]
#[serial]
fn subpnt() {
    common::load();

    let et = common::EPOCH;
    let (point, epoch, observer_to_point) = spice::subpnt(
        "Near point: ellipsoid",
        "EARTH",
        et,
        "IAU_EARTH",
        "NONE",
        "SUN",
    );
    common::assert_ok("subpnt");

    assert_relative_eq!(epoch, et, epsilon = 1e-9);

    // The sub-observer point lies on the ellipsoid...
    let radii = spice::bodvrd("EARTH", "RADII", 3);
    let scaled = (0..3).map(|i| (point[i] / radii[i]).powi(2)).sum::<f64>();
    assert_relative_eq!(scaled, 1.0, epsilon = 1e-9);

    // ... and the observer sees it along the direction of the body centre.
    let (to_target, _) = spice::spkpos("EARTH", et, "IAU_EARTH", "NONE", "SUN");
    assert_relative_eq!(
        spice::vsep(observer_to_point, to_target),
        0.0,
        epsilon = 1e-9
    );

    common::unload();
}

#[test]
#[serial]
fn subslr() {
    common::load();

    let et = common::EPOCH;
    // The sub-solar point does not depend on where the observer stands, so it is the sub-observer
    // point of the Sun itself.
    let (solar, _, _) = spice::subslr(
        "Near point: ellipsoid",
        "EARTH",
        et,
        "IAU_EARTH",
        "NONE",
        "MOON",
    );
    common::assert_ok("subslr");
    let (observer, _, _) = spice::subpnt(
        "Near point: ellipsoid",
        "EARTH",
        et,
        "IAU_EARTH",
        "NONE",
        "SUN",
    );
    assert_slice_eq(&solar, &observer, 1e-6);

    common::unload();
}

#[test]
#[serial]
fn ilumin() {
    common::load();

    let et = common::EPOCH;
    let (point, _, _) = spice::subpnt(
        "Near point: ellipsoid",
        "EARTH",
        et,
        "IAU_EARTH",
        "NONE",
        "SUN",
    );

    // CSPICE insists the observer, the target and the illumination source be distinct, so the
    // satellite watches the sub-solar point of its primary.
    let (epoch, _, phase, incidence, emission) =
        spice::ilumin("Ellipsoid", "EARTH", et, "IAU_EARTH", "NONE", "MOON", point);
    common::assert_ok("ilumin");

    assert_relative_eq!(epoch, et, epsilon = 1e-9);
    // The sub-solar point faces the source head on, so the incidence angle vanishes and the phase
    // angle is the one the observer sees.
    assert_relative_eq!(incidence, 0.0, epsilon = 1e-6);
    assert_relative_eq!(phase, emission, epsilon = 1e-6);

    common::unload();
}

#[test]
#[serial]
fn illumf() {
    common::load();

    let et = common::EPOCH;
    let (point, _, _) = spice::subpnt(
        "Near point: ellipsoid",
        "EARTH",
        et,
        "IAU_EARTH",
        "NONE",
        "SUN",
    );

    let (epoch, _, phase, incidence, emission, visible, lit) = spice::illumf(
        "Ellipsoid",
        "EARTH",
        "SUN",
        et,
        "IAU_EARTH",
        "NONE",
        "MOON",
        point,
    );
    common::assert_ok("illumf");

    assert_relative_eq!(epoch, et, epsilon = 1e-9);
    assert_relative_eq!(incidence, 0.0, epsilon = 1e-6);
    assert!(lit, "the sub-solar point is lit");

    // The angles agree with what `ilumin` reports for the same point.
    let (_, _, reference_phase, reference_incidence, reference_emission) =
        spice::ilumin("Ellipsoid", "EARTH", et, "IAU_EARTH", "NONE", "MOON", point);
    assert_relative_eq!(phase, reference_phase, epsilon = 1e-12);
    assert_relative_eq!(incidence, reference_incidence, epsilon = 1e-12);
    assert_relative_eq!(emission, reference_emission, epsilon = 1e-12);
    let _ = visible;

    // The far side of the body is neither visible nor lit.
    let (_, _, _, _, _, visible, lit) = spice::illumf(
        "Ellipsoid",
        "EARTH",
        "SUN",
        et,
        "IAU_EARTH",
        "NONE",
        "MOON",
        spice::vminus(point),
    );
    common::assert_ok("illumf on the far side");
    assert!(!lit, "the anti-solar point is in shadow");
    let _ = visible;

    common::unload();
}

#[test]
#[serial]
fn surfpt() {
    common::load();

    // A ray along +x from far away hits the sphere of radius 2 at (2, 0, 0).
    let (point, found) = spice::surfpt([10.0, 0.0, 0.0], [-1.0, 0.0, 0.0], 2.0, 2.0, 2.0);
    assert!(found);
    assert_slice_eq(&point, &[2.0, 0.0, 0.0], 1e-12);

    // Pointing away misses.
    assert!(!spice::surfpt([10.0, 0.0, 0.0], [1.0, 0.0, 0.0], 2.0, 2.0, 2.0).1);

    common::unload();
}

#[test]
#[serial]
fn nearpt() {
    common::load();

    // On the +x axis, the nearest point of a sphere of radius 2 is at (2, 0, 0).
    let (point, altitude) = spice::nearpt([10.0, 0.0, 0.0], 2.0, 2.0, 2.0);
    assert_slice_eq(&point, &[2.0, 0.0, 0.0], 1e-12);
    assert_relative_eq!(altitude, 8.0, epsilon = 1e-12);

    // Inside the body, the altitude is negative.
    assert!(spice::nearpt([1.0, 0.0, 0.0], 2.0, 2.0, 2.0).1 < 0.0);

    common::unload();
}

#[test]
#[serial]
fn occult() {
    common::load();

    let et = common::EPOCH;
    // The satellite is far too small, and far too close to its primary, to occult the Sun here.
    let code = spice::occult(
        "MOON",
        "ELLIPSOID",
        "IAU_MOON",
        "SUN",
        "ELLIPSOID",
        "IAU_SUN",
        "NONE",
        "EARTH",
        et,
    );
    common::assert_ok("occult");
    assert_eq!(code, 0, "no occultation");

    // A body always occults itself totally; codes run from -3 to 3.
    assert!((-3..=3).contains(&code));

    common::unload();
}

#[test]
#[serial]
fn phaseq() {
    common::load();

    let et = common::EPOCH;
    let angle = spice::phaseq(et, "EARTH", "SUN", "MOON", "NONE");
    common::assert_ok("phaseq");
    assert!((0.0..=std::f64::consts::PI).contains(&angle), "got {angle}");

    // Seen from the illumination source, the phase angle vanishes.
    let angle = spice::phaseq(et, "EARTH", "SUN", "SUN", "NONE");
    assert_relative_eq!(angle, 0.0, epsilon = 1e-9);

    common::unload();
}

#[test]
#[serial]
fn srfnrm() {
    common::load();

    let et = common::EPOCH;
    // Points on the (+,+,+) face of the octahedron share its normal.
    let third = common::SHAPE_RADIUS / 3.0;
    let normals = spice::srfnrm(
        "DSK/UNPRIORITIZED",
        "EARTH",
        et,
        "IAU_EARTH",
        &[[third; 3], [common::SHAPE_RADIUS, 0.0, 0.0]],
    );
    common::assert_ok("srfnrm");

    assert_eq!(normals.len(), 2);
    let unit = 1.0 / 3f64.sqrt();
    assert_slice_eq(&normals[0], &[unit; 3], 1e-9);
    for normal in &normals {
        assert_relative_eq!(spice::vnorm(*normal), 1.0, epsilon = 1e-12);
    }

    common::unload();
}

#[test]
#[serial]
fn latsrf() {
    common::load();

    let et = common::EPOCH;
    // Longitude and latitude zero points at the (+x) vertex of the octahedron.
    let points = spice::latsrf(
        "DSK/UNPRIORITIZED",
        "EARTH",
        et,
        "IAU_EARTH",
        &[[0.0, 0.0], [std::f64::consts::FRAC_PI_2, 0.0]],
    );
    common::assert_ok("latsrf");

    assert_eq!(points.len(), 2);
    assert_slice_eq(&points[0], &[common::SHAPE_RADIUS, 0.0, 0.0], 1e-6);
    assert_slice_eq(&points[1], &[0.0, common::SHAPE_RADIUS, 0.0], 1e-6);

    common::unload();
}

#[test]
#[serial]
fn edterm() {
    common::load();

    let et = common::EPOCH;
    let (epoch, observer, points) = spice::edterm(
        "UMBRAL",
        "SUN",
        "EARTH",
        et,
        "IAU_EARTH",
        "NONE",
        "MOON",
        12,
    );
    common::assert_ok("edterm");

    assert_relative_eq!(epoch, et, epsilon = 1e-9);
    assert_eq!(points.len(), 12);
    assert!(spice::vnorm(observer) > 0.0);

    // Every terminator point lies on the ellipsoid.
    let radii = spice::bodvrd("EARTH", "RADII", 3);
    for point in &points {
        let scaled = (0..3).map(|i| (point[i] / radii[i]).powi(2)).sum::<f64>();
        assert_relative_eq!(scaled, 1.0, epsilon = 1e-9);
    }

    common::unload();
}

#[test]
#[serial]
fn getfov() {
    common::load();

    let (shape, frame, boresight, bounds) = spice::getfov(common::INSTRUMENT, 4);
    common::assert_ok("getfov");

    assert_eq!(shape, "RECTANGLE");
    assert_eq!(frame, "TEST_INSTRUMENT");
    assert_slice_eq(&boresight, &[0.0, 0.0, 1.0], 1e-15);
    assert_eq!(bounds.len(), 4, "a rectangle has four corners");

    // Each corner sits five degrees off the boresight in both directions.
    let expected = (5f64.to_radians().tan().powi(2) * 2.0).sqrt().atan();
    for corner in &bounds {
        assert_relative_eq!(spice::vsep(*corner, boresight), expected, epsilon = 1e-12);
    }

    common::unload();
}
