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

#[test]
#[serial]
fn conversions_between_curvilinear_systems() {
    // Every pair round trips, and agrees with going through rectangular coordinates.
    let (radius, lon, lat) = (7.0, 0.6, -0.3);
    let rectangular = spice::latrec(radius, lon, lat);

    let (r, lonc, z) = spice::latcyl(radius, lon, lat);
    assert_slice_eq(&spice::cylrec(r, lonc, z), &rectangular, 1e-13);
    let back = spice::cyllat(r, lonc, z);
    assert_slice_eq(&[back.0, back.1, back.2], &[radius, lon, lat], 1e-13);

    let (rho, colat, lons) = spice::latsph(radius, lon, lat);
    assert_slice_eq(&spice::sphrec(rho, colat, lons), &rectangular, 1e-13);
    let back = spice::sphlat(rho, colat, lons);
    assert_slice_eq(&[back.0, back.1, back.2], &[radius, lon, lat], 1e-13);

    let (sr, scolat, slon) = spice::cylsph(r, lonc, z);
    assert_slice_eq(&[sr, scolat, slon], &[rho, colat, lons], 1e-13);
    let back = spice::sphcyl(rho, colat, lons);
    assert_slice_eq(&[back.0, back.1, back.2], &[r, lonc, z], 1e-13);
}

#[test]
#[serial]
fn azimuth_and_elevation() {
    // Azimuth clockwise from +x, elevation positive toward +z: +y is at azimuth 3*pi/2.
    let (range, az, el) = spice::recazl([0.0, 1.0, 0.0], false, true);
    assert_relative_eq!(range, 1.0, epsilon = 1e-15);
    assert_relative_eq!(az, 3.0 * std::f64::consts::FRAC_PI_2, epsilon = 1e-15);
    assert_relative_eq!(el, 0.0, epsilon = 1e-15);

    // Counting the other way puts it at pi/2 instead.
    let (_, az, _) = spice::recazl([0.0, 1.0, 0.0], true, true);
    assert_relative_eq!(az, std::f64::consts::FRAC_PI_2, epsilon = 1e-15);

    // And the round trip holds for both conventions.
    for azccw in [true, false] {
        for elplsz in [true, false] {
            let point = [3.0, -4.0, 5.0];
            let (range, az, el) = spice::recazl(point, azccw, elplsz);
            assert_slice_eq(&spice::azlrec(range, az, el, azccw, elplsz), &point, 1e-13);
        }
    }
}

/// Compare an analytic Jacobian against a central difference of the conversion it differentiates.
#[track_caller]
fn assert_jacobian(jacobian: [[f64; 3]; 3], at: [f64; 3], forward: impl Fn([f64; 3]) -> [f64; 3]) {
    const H: f64 = 1e-6;
    for column in 0..3 {
        let (mut plus, mut minus) = (at, at);
        plus[column] += H;
        minus[column] -= H;
        let (plus, minus) = (forward(plus), forward(minus));
        for row in 0..3 {
            let numeric = (plus[row] - minus[row]) / (2.0 * H);
            assert_relative_eq!(
                jacobian[row][column],
                numeric,
                epsilon = 1e-5,
                max_relative = 1e-5
            );
        }
    }
}

#[test]
#[serial]
fn jacobians() {
    crate::common::load();

    let point = [3.0, -4.0, 5.0];
    let (x, y, z) = (point[0], point[1], point[2]);

    // Rectangular to each curvilinear system, against a numerical derivative.
    assert_jacobian(spice::dlatdr(x, y, z), point, |p| {
        let (r, lon, lat) = spice::reclat(p);
        [r, lon, lat]
    });
    assert_jacobian(spice::dsphdr(x, y, z), point, |p| {
        let (r, colat, lon) = spice::recsph(p);
        [r, colat, lon]
    });
    assert_jacobian(spice::dcyldr(x, y, z), point, |p| {
        let (r, lon, z) = spice::reccyl(p);
        [r, lon, z]
    });
    assert_jacobian(
        spice::dgeodr(x, y, z, CLARK66_RADIUS, CLARK66_FLATTENING),
        point,
        |p| {
            let (lon, lat, alt) = spice::recgeo(p, CLARK66_RADIUS, CLARK66_FLATTENING);
            [lon, lat, alt]
        },
    );
    assert_jacobian(
        spice::dpgrdr("EARTH", x, y, z, CLARK66_RADIUS, CLARK66_FLATTENING),
        point,
        |p| {
            let (lon, lat, alt) = spice::recpgr("EARTH", p, CLARK66_RADIUS, CLARK66_FLATTENING);
            [lon, lat, alt]
        },
    );
    assert_jacobian(spice::dazldr(x, y, z, true, true), point, |p| {
        let (range, az, el) = spice::recazl(p, true, true);
        [range, az, el]
    });
    crate::common::assert_ok("the rectangular to curvilinear Jacobians");

    // And each inverse, which is the inverse matrix of the forward one.
    let (r, lon, lat) = spice::reclat(point);
    assert_jacobian(spice::drdlat(r, lon, lat), [r, lon, lat], |p| {
        spice::latrec(p[0], p[1], p[2])
    });

    let (r, colat, slon) = spice::recsph(point);
    assert_jacobian(spice::drdsph(r, colat, slon), [r, colat, slon], |p| {
        spice::sphrec(p[0], p[1], p[2])
    });

    let (rc, lonc, zc) = spice::reccyl(point);
    assert_jacobian(spice::drdcyl(rc, lonc, zc), [rc, lonc, zc], |p| {
        spice::cylrec(p[0], p[1], p[2])
    });

    let (glon, glat, galt) = spice::recgeo(point, CLARK66_RADIUS, CLARK66_FLATTENING);
    assert_jacobian(
        spice::drdgeo(glon, glat, galt, CLARK66_RADIUS, CLARK66_FLATTENING),
        [glon, glat, galt],
        |p| spice::georec(p[0], p[1], p[2], CLARK66_RADIUS, CLARK66_FLATTENING),
    );

    let (plon, plat, palt) = spice::recpgr("EARTH", point, CLARK66_RADIUS, CLARK66_FLATTENING);
    assert_jacobian(
        spice::drdpgr(
            "EARTH",
            plon,
            plat,
            palt,
            CLARK66_RADIUS,
            CLARK66_FLATTENING,
        ),
        [plon, plat, palt],
        |p| {
            spice::pgrrec(
                "EARTH",
                p[0],
                p[1],
                p[2],
                CLARK66_RADIUS,
                CLARK66_FLATTENING,
            )
        },
    );

    let (range, az, el) = spice::recazl(point, true, true);
    assert_jacobian(
        spice::drdazl(range, az, el, true, true),
        [range, az, el],
        |p| spice::azlrec(p[0], p[1], p[2], true, true),
    );
    crate::common::assert_ok("the curvilinear to rectangular Jacobians");

    // A forward Jacobian and its inverse multiply to the identity.
    let forward = spice::dlatdr(x, y, z);
    let inverse = spice::drdlat(r, lon, lat);
    crate::assert_matrix_eq(&spice::mxm(forward, inverse), &spice::ident(), 1e-9);

    crate::common::unload();
}

#[test]
#[serial]
fn xfmsta() {
    crate::common::load();

    let state = [3.0, -4.0, 5.0, 0.1, 0.2, -0.3];
    let latitudinal = spice::xfmsta(state, "RECTANGULAR", "LATITUDINAL", "EARTH");
    crate::common::assert_ok("xfmsta");

    // The position half matches the plain conversion...
    let (r, lon, lat) = spice::reclat([state[0], state[1], state[2]]);
    assert_slice_eq(&latitudinal[..3], &[r, lon, lat], 1e-12);

    // ... and the velocity half is the Jacobian applied to the velocity.
    let expected = spice::mxv(
        spice::dlatdr(state[0], state[1], state[2]),
        [state[3], state[4], state[5]],
    );
    assert_slice_eq(&latitudinal[3..], &expected, 1e-12);

    // Converting back returns the state it started from.
    let back = spice::xfmsta(latitudinal, "LATITUDINAL", "RECTANGULAR", "EARTH");
    assert_slice_eq(&back, &state, 1e-12);

    crate::common::unload();
}
