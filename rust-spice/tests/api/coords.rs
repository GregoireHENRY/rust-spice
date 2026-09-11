//! Coordinate system conversions.

use crate::assert_slice_eq;

/// Radius and flattening of the Clark 66 spheroid, as used by the CSPICE documentation.
const CLARK66_RADIUS: f64 = 6378.2064;
const CLARK66_FLATTENING: f64 = 1.0 / 294.9787;

#[test]
#[serial]
fn georec() {
    // Test vectors from https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/georec_c.html
    // lon, lat, alt -> x, y, z
    let cases: [[f64; 6]; 11] = [
        [0.0000, 90.0000, -6356.5838, 0.0000, 0.0000, 0.0000],
        [0.0000, 0.0000, -6377.2063, 1.0000, 0.0000, 0.0000],
        [90.0000, 0.0000, -6377.2063, 0.0000, 1.0000, 0.0000],
        [0.0000, 90.0000, -6355.5838, 0.0000, 0.0000, 1.0000],
        [180.0000, 0.0000, -6377.2063, -1.0000, 0.0000, 0.0000],
        [-90.0000, 0.0000, -6377.2063, 0.0000, -1.0000, 0.0000],
        [0.0000, -90.0000, -6355.5838, 0.0000, 0.0000, -1.0000],
        [45.0000, 0.0000, -6376.7921, 1.0000, 1.0000, 0.0000],
        [0.0000, 88.7070, -6355.5725, 1.0000, 0.0000, 1.0000],
        [90.0000, 88.7070, -6355.5725, 0.0000, 1.0000, 1.0000],
        [45.0000, 88.1713, -6355.5612, 1.0000, 1.0000, 1.0000],
    ];

    for case in cases {
        let rectangular = spice::georec(
            case[0].to_radians(),
            case[1].to_radians(),
            case[2],
            CLARK66_RADIUS,
            CLARK66_FLATTENING,
        );
        assert_slice_eq(&rectangular, &case[3..], 1e-4);
    }
}

#[test]
#[serial]
fn recgeo() {
    // `recgeo` is the inverse of `georec`.
    let (lon, lat, alt) = (0.7, 0.4, 12.5);
    let rectangular = spice::georec(lon, lat, alt, CLARK66_RADIUS, CLARK66_FLATTENING);
    let back = spice::recgeo(rectangular, CLARK66_RADIUS, CLARK66_FLATTENING);
    assert_slice_eq(&[back.0, back.1, back.2], &[lon, lat, alt], 1e-9);
}

#[test]
#[serial]
fn latitudinal() {
    assert_slice_eq(&spice::latrec(1.0, 0.0, 0.0), &[1.0, 0.0, 0.0], 1e-15);
    assert_slice_eq(
        &spice::latrec(1.0, std::f64::consts::FRAC_PI_2, 0.0),
        &[0.0, 1.0, 0.0],
        1e-15,
    );

    let (radius, longitude, latitude) = spice::reclat([1.0, 1.0, 0.0]);
    assert_relative_eq!(radius, std::f64::consts::SQRT_2, epsilon = 1e-15);
    assert_relative_eq!(longitude, std::f64::consts::FRAC_PI_4, epsilon = 1e-15);
    assert_relative_eq!(latitude, 0.0, epsilon = 1e-15);

    // Round trip.
    let point = [3.0, -4.0, 5.0];
    let (radius, longitude, latitude) = spice::reclat(point);
    assert_slice_eq(&spice::latrec(radius, longitude, latitude), &point, 1e-13);
}

#[test]
#[serial]
fn spherical() {
    // Colatitude is measured from the +z axis.
    assert_slice_eq(&spice::sphrec(1.0, 0.0, 0.0), &[0.0, 0.0, 1.0], 1e-15);

    let point = [3.0, -4.0, 5.0];
    let (radius, colatitude, longitude) = spice::recsph(point);
    assert_relative_eq!(radius, spice::vnorm(point), epsilon = 1e-14);
    assert_slice_eq(&spice::sphrec(radius, colatitude, longitude), &point, 1e-13);
}

#[test]
#[serial]
fn cylindrical() {
    assert_slice_eq(
        &spice::cylrec(2.0, std::f64::consts::FRAC_PI_2, 7.0),
        &[0.0, 2.0, 7.0],
        1e-15,
    );

    let point = [3.0, -4.0, 5.0];
    let (radius, longitude, z) = spice::reccyl(point);
    assert_relative_eq!(radius, 5.0, epsilon = 1e-14);
    assert_relative_eq!(z, 5.0, epsilon = f64::EPSILON);
    assert_slice_eq(&spice::cylrec(radius, longitude, z), &point, 1e-13);
}

#[test]
#[serial]
fn planetographic() {
    crate::common::load();

    let point = [7000.0, 1500.0, 2500.0];
    let (lon, lat, alt) = spice::recpgr("EARTH", point, CLARK66_RADIUS, CLARK66_FLATTENING);
    let back = spice::pgrrec("EARTH", lon, lat, alt, CLARK66_RADIUS, CLARK66_FLATTENING);
    assert_slice_eq(&back, &point, 1e-9);

    crate::common::unload();
}

#[test]
#[serial]
fn right_ascension() {
    // The +x axis is at zero right ascension and declination.
    assert_slice_eq(&spice::radrec(1.0, 0.0, 0.0), &[1.0, 0.0, 0.0], 1e-15);

    let (range, ra, dec) = spice::recrad([1.0, 1.0, 1.0]);
    assert_relative_eq!(range, 3f64.sqrt(), epsilon = 1e-15);
    assert_relative_eq!(ra, std::f64::consts::FRAC_PI_4, epsilon = 1e-15);
    assert_relative_eq!(dec, (1.0f64 / 2f64.sqrt()).atan(), epsilon = 1e-15);

    // Right ascension is reported in [0, 2*pi).
    let (_, ra, _) = spice::recrad([-1.0, 0.0, 0.0]);
    assert_relative_eq!(ra, std::f64::consts::PI, epsilon = 1e-15);
}

#[test]
#[serial]
fn srfrec() {
    crate::common::load();

    // A point on the equator at zero longitude, in the body-fixed frame.
    let point = spice::srfrec(crate::common::TARGET, 0.0, 0.0);
    let radii = spice::bodvrd("EARTH", "RADII", 3);
    assert_slice_eq(&point, &[radii[0], 0.0, 0.0], 1e-9);

    // And one at the north pole.
    let pole = spice::srfrec(crate::common::TARGET, 0.0, std::f64::consts::FRAC_PI_2);
    assert_slice_eq(&pole, &[0.0, 0.0, radii[2]], 1e-9);

    crate::common::unload();
}
