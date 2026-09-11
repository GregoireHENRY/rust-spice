//! Interpolation, polynomials, plates, and the remaining pool, file and time routines.

use crate::assert_slice_eq;
use crate::common;

#[test]
#[serial]
fn chebyshev() {
    common::reset();

    // The expansion 2 + 3*T1(s) on the interval [0, 2], where s = x - 1, is 3x - 1.
    let cp = [2.0, 3.0];
    let x2s = [1.0, 1.0];

    assert_relative_eq!(spice::chbval(&cp, 1, x2s, 1.0), 2.0, epsilon = 1e-14);
    assert_relative_eq!(spice::chbval(&cp, 1, x2s, 2.0), 5.0, epsilon = 1e-14);

    // The derivative is the constant 3.
    let (value, derivative) = spice::chbint(&cp, 1, x2s, 2.0);
    assert_relative_eq!(value, 5.0, epsilon = 1e-14);
    assert_relative_eq!(derivative, 3.0, epsilon = 1e-14);

    // `chbder` reports the value and the derivatives together.
    let derivatives = spice::chbder(&cp, 1, x2s, 2.0, 2);
    assert_eq!(derivatives.len(), 3);
    assert_relative_eq!(derivatives[0], 5.0, epsilon = 1e-14);
    assert_relative_eq!(derivatives[1], 3.0, epsilon = 1e-14);
    assert_relative_eq!(derivatives[2], 0.0, epsilon = 1e-14);

    // And `chbigr` the value and its integral from the interval midpoint.
    let (value, integral) = spice::chbigr(1, &cp, x2s, 2.0);
    assert_relative_eq!(value, 5.0, epsilon = 1e-14);
    assert_relative_eq!(integral, 3.5, epsilon = 1e-12);

    common::assert_ok("the Chebyshev routines");
}

#[test]
#[serial]
fn polynomials() {
    common::reset();

    // 1 + 2t + 3t^2 at t = 2 is 17, with derivatives 14 and 6.
    let derivatives = spice::polyds(&[1.0, 2.0, 3.0], 2, 2, 2.0);
    common::assert_ok("polyds");
    assert_slice_eq(&derivatives, &[17.0, 14.0, 6.0], 1e-13);
}

#[test]
#[serial]
fn interpolation() {
    common::reset();

    // A straight line through three points, so every scheme reproduces it exactly.
    let xvals = [0.0, 1.0, 2.0];
    let yvals = [1.0, 3.0, 5.0];

    assert_relative_eq!(spice::lgrint(&xvals, &yvals, 0.5), 2.0, epsilon = 1e-13);
    assert_relative_eq!(spice::lgresp(0.0, 1.0, &yvals, 0.5), 2.0, epsilon = 1e-13);

    let (value, slope) = spice::lgrind(&xvals, &yvals, 0.5);
    assert_relative_eq!(value, 2.0, epsilon = 1e-13);
    assert_relative_eq!(slope, 2.0, epsilon = 1e-13);

    // Hermite interpolation takes the value and the derivative at each point.
    let paired = [1.0, 2.0, 3.0, 2.0, 5.0, 2.0];
    let (value, slope) = spice::hrmint(&xvals, &paired, 0.5);
    assert_relative_eq!(value, 2.0, epsilon = 1e-13);
    assert_relative_eq!(slope, 2.0, epsilon = 1e-13);

    let (value, slope) = spice::hrmesp(0.0, 1.0, &paired, 0.5);
    assert_relative_eq!(value, 2.0, epsilon = 1e-13);
    assert_relative_eq!(slope, 2.0, epsilon = 1e-13);

    // A central difference of a function sampled either side of a point.
    let derivative = spice::qderiv(&[0.0, 0.0], &[2.0, 4.0], 1.0);
    assert_slice_eq(&derivative, &[1.0, 2.0], 1e-13);

    common::assert_ok("the interpolation routines");
}

#[test]
#[serial]
fn plates() {
    common::reset();

    let vertices = common::octahedron_vertices();
    let plates = common::octahedron_plates();

    // An octahedron of "radius" r has eight faces, each an equilateral triangle of side r*sqrt(2).
    let side = common::SHAPE_RADIUS * 2f64.sqrt();
    let face = 3f64.sqrt() / 4.0 * side * side;
    assert_relative_eq!(
        spice::pltar(&vertices, &plates),
        8.0 * face,
        max_relative = 1e-12
    );

    // And a volume of 4/3 r^3.
    assert_relative_eq!(
        spice::pltvol(&vertices, &plates),
        4.0 / 3.0 * common::SHAPE_RADIUS.powi(3),
        max_relative = 1e-12
    );

    // The normal of one face, scaled by twice its area.
    let normal = spice::pltnrm([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
    assert_slice_eq(&spice::vhat(normal), &[1.0 / 3f64.sqrt(); 3], 1e-14);

    // The point of that plate nearest the origin is its centroid, at (1/3, 1/3, 1/3).
    let (near, distance) =
        spice::pltnp([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
    assert_slice_eq(&near, &[1.0 / 3.0; 3], 1e-13);
    assert_relative_eq!(distance, 1.0 / 3f64.sqrt(), epsilon = 1e-13);

    common::assert_ok("the plate routines");
}

#[test]
#[serial]
fn diagonalise() {
    common::reset();

    // [[2, 1], [1, 2]] has eigenvalues 3 and 1.
    let (diag, rotation) = spice::diags2([[2.0, 1.0], [1.0, 2.0]]);
    common::assert_ok("diags2");

    let mut eigen = [diag[0][0], diag[1][1]];
    eigen.sort_by(f64::total_cmp);
    assert_slice_eq(&eigen, &[1.0, 3.0], 1e-13);
    assert_relative_eq!(diag[0][1], 0.0, epsilon = 1e-14);

    // The rotation is orthogonal.
    let det = rotation[0][0] * rotation[1][1] - rotation[0][1] * rotation[1][0];
    assert_relative_eq!(det.abs(), 1.0, epsilon = 1e-13);
}

#[test]
#[serial]
fn linear_searches_and_sets() {
    common::reset();

    // An unordered array, searched linearly.
    assert_eq!(spice::isrchd(3.0, &[5.0, 3.0, 1.0]), 1);
    assert_eq!(spice::isrchi(1, &[5, 3, 1]), 2);
    assert_eq!(spice::isrchc("beta", &["gamma", "beta"]), 1);
    assert_eq!(spice::isrchd(9.0, &[5.0, 3.0]), -1);

    // The ordinal position of an item within a set.
    let mut numbers = spice::Cell::<f64>::new(8);
    for item in [3.0, 1.0, 2.0] {
        spice::insrtd(item, &mut numbers);
    }
    // Positions run from zero, and an absent item gives -1.
    assert_eq!(
        spice::ordd(2.0, &mut numbers),
        1,
        "the set is sorted, so 2.0 is at index 1"
    );
    assert_eq!(spice::ordd(9.0, &mut numbers), -1);

    let mut integers = spice::Cell::<i32>::new(8);
    for item in [3, 1, 2] {
        spice::insrti(item, &mut integers);
    }
    assert_eq!(spice::ordi(3, &mut integers), 2);

    let mut names = spice::Cell::<String>::with_length(8, 16);
    for item in ["gamma", "alpha"] {
        spice::insrtc(item, &mut names);
    }
    assert_eq!(spice::ordc("gamma", &mut names), 1);

    // The symmetric difference, and set comparison.
    let mut a = spice::Cell::<i32>::new(8);
    let mut b = spice::Cell::<i32>::new(8);
    for item in [1, 2, 3] {
        spice::insrti(item, &mut a);
    }
    for item in [3, 4] {
        spice::insrti(item, &mut b);
    }
    let mut result = spice::Cell::<i32>::new(8);
    spice::sdiff(&mut a, &mut b, &mut result);
    assert_eq!(result.to_vec(), vec![1, 2, 4]);

    assert!(spice::set(&mut a, "<>", &mut b));
    assert!(!spice::set(&mut a, "=", &mut b));
    let mut same = a.clone();
    assert!(spice::set(&mut a, "=", &mut same));

    // Parsing a list straight into a set drops the duplicates.
    let mut items = spice::Cell::<String>::with_length(8, 32);
    spice::lparss("beta, alpha, beta", ", ", &mut items);
    common::assert_ok("lparss");
    assert_eq!(
        items.to_vec(),
        vec!["alpha".to_string(), "beta".to_string()]
    );
}

#[test]
#[serial]
fn pool_extras() {
    common::load();

    // `szpool` reports the pool's own limits, not the size of a variable.
    let (maxvar, found) = spice::szpool("MAXVAR");
    common::assert_ok("szpool");
    assert!(found && maxvar > 0);
    assert!(!spice::szpool("NO_SUCH_PARAMETER").1);
    spice::errors::reset();

    // How many values a variable holds comes from `dtpool`.
    assert_eq!(spice::dtpool("BODY399_RADII"), (true, 3, "N".to_string()));

    // A text kernel held in memory rather than in a file.
    spice::lmpool(&[
        "TEST_MEMORY_VALUE = ( 42 )",
        "TEST_MEMORY_TEXT = ( 'alpha' )",
    ]);
    common::assert_ok("lmpool");
    assert_eq!(spice::gipool("TEST_MEMORY_VALUE", 0, 4), vec![42]);
    assert_eq!(
        spice::gcpool("TEST_MEMORY_TEXT", 0, 4),
        vec!["alpha".to_string()]
    );

    // A long string split across continuation lines is rejoined.
    spice::lmpool(&[
        "TEST_LONG = ( 'the first part //'",
        "              'and the second' )",
    ]);
    let (joined, size, found) = spice::stpool("TEST_LONG", 0, "//");
    common::assert_ok("stpool");
    assert!(found);
    assert_eq!(joined, "the first part and the second");
    assert_eq!(size as usize, joined.len());

    // The built-in frames of a class that the pool knows about.
    let mut ids = spice::Cell::<i32>::new(256);
    spice::kplfrm(2, &mut ids);
    common::assert_ok("kplfrm");
    let _ = ids.len();

    common::unload();
}

#[test]
#[serial]
fn files_and_frames() {
    common::load();

    // A kernel's architecture and type, straight from its file record.
    assert_eq!(
        spice::getfat(&common::kernel("test.bsp")),
        ("DAF".to_string(), "SPK".to_string())
    );
    assert_eq!(
        spice::getfat(&common::kernel("test.bds")),
        ("DAS".to_string(), "DSK".to_string())
    );
    assert_eq!(
        spice::getfat(&common::kernel("naif.tls")),
        ("KPL".to_string(), "LSK".to_string())
    );
    common::assert_ok("getfat");

    assert!(spice::exists(&common::kernel("test.bsp")));
    assert!(!spice::exists(&common::kernel("no-such-file")));

    // Reading a text kernel line by line. The first line is the identification word.
    let path = common::kernel("naif.tls");
    let (first, eof) = spice::rdtext(&path, spice::MAX_LEN_OUT);
    common::assert_ok("rdtext");
    assert!(!eof);
    assert_eq!(first, "KPL/LSK");

    // The file stays open until it is read to the end, and CSPICE will not load a file it
    // already has open, so the read has to be finished.
    let mut lines = 1;
    while !spice::rdtext(&path, spice::MAX_LEN_OUT).1 {
        lines += 1;
    }
    common::assert_ok("reading to the end");
    assert!(
        lines > 10,
        "the leapseconds kernel has more than ten lines, got {lines}"
    );

    // The centre, class and class ID of a frame.
    let (center, class, class_id, found) = spice::frinfo(spice::namfrm("IAU_EARTH"));
    common::assert_ok("frinfo");
    assert!(found);
    assert_eq!((center, class, class_id), (399, 2, 399));

    // A right handed frame built from one vector.
    let (x, y, z) = spice::frame([2.0, 0.0, 0.0]);
    assert_slice_eq(&x, &[1.0, 0.0, 0.0], 1e-15);
    assert_relative_eq!(spice::vdot(x, y), 0.0, epsilon = 1e-14);
    assert_slice_eq(&spice::vcrss(x, y), &z, 1e-14);

    common::unload();
}

#[test]
#[serial]
fn fields_of_view() {
    common::load();

    let et = common::EPOCH;

    // The instrument frame is the identity, so its boresight is +z in J2000 too.
    assert!(spice::fovray(
        "TEST_INSTRUMENT",
        [0.0, 0.0, 1.0],
        "J2000",
        "NONE",
        "TEST_SPACECRAFT",
        et
    ));
    assert!(!spice::fovray(
        "TEST_INSTRUMENT",
        [0.0, 0.0, -1.0],
        "J2000",
        "NONE",
        "TEST_SPACECRAFT",
        et
    ));
    common::assert_ok("fovray");

    // `getfvn` takes the instrument by name and reports the same field of view as `getfov`.
    let (shape, frame, bsight, bounds) = spice::getfvn("TEST_INSTRUMENT", 4);
    common::assert_ok("getfvn");
    let (shape2, frame2, bsight2, bounds2) = spice::getfov(common::INSTRUMENT, 4);
    assert_eq!((shape, frame), (shape2, frame2));
    assert_slice_eq(&bsight, &bsight2, 0.0);
    assert_eq!(bounds.len(), bounds2.len());

    common::unload();
}

#[test]
#[serial]
fn illumination_with_a_named_source() {
    common::load();

    let et = common::EPOCH;
    let (point, _, _) = spice::subpnt(
        "Near point: ellipsoid",
        "EARTH",
        et,
        "IAU_EARTH",
        "NONE",
        "MOON",
    );

    // `illumg` is `illumf` without the visibility flags.
    let (epoch, _, phase, incidence, emission) = spice::illumg(
        "Ellipsoid",
        "EARTH",
        "SUN",
        et,
        "IAU_EARTH",
        "NONE",
        "MOON",
        point,
    );
    common::assert_ok("illumg");
    assert_relative_eq!(epoch, et, epsilon = 1e-9);

    let (_, _, p2, i2, e2, _, _) = spice::illumf(
        "Ellipsoid",
        "EARTH",
        "SUN",
        et,
        "IAU_EARTH",
        "NONE",
        "MOON",
        point,
    );
    assert_relative_eq!(phase, p2, epsilon = 1e-12);
    assert_relative_eq!(incidence, i2, epsilon = 1e-12);
    assert_relative_eq!(emission, e2, epsilon = 1e-12);

    common::unload();
}

#[test]
#[serial]
fn time_extras() {
    common::load();

    // The local solar time at a longitude on a body.
    let (hr, mn, sc, time, ampm) = spice::et2lst(common::EPOCH, 399, 0.0, "PLANETOCENTRIC");
    common::assert_ok("et2lst");
    assert!((0..24).contains(&hr));
    assert!((0..60).contains(&mn) && (0..60).contains(&sc));
    assert!(!time.is_empty() && !ampm.is_empty());

    // A format picture built from a sample.
    let (picture, ok, error) = spice::tpictr("1988 JUN 13 03:29:48");
    common::assert_ok("tpictr");
    assert!(ok, "unexpected error {error:?}");
    assert_eq!(
        spice::timout(spice::str2et("1988 JUN 13 03:29:48"), &picture),
        "1988 JUN 13 03:29:48"
    );

    // A sample the parser cannot understand is reported rather than guessed at.
    let (_, ok, error) = spice::tpictr("not a time");
    assert!(!ok);
    assert!(!error.is_empty());

    // The time defaults can be read and set.
    assert_eq!(spice::timdef("GET", "SYSTEM", ""), "UTC");
    spice::timdef("SET", "SYSTEM", "TDB");
    assert_eq!(spice::timdef("GET", "SYSTEM", ""), "TDB");
    spice::timdef("SET", "SYSTEM", "UTC");
    common::assert_ok("timdef");

    common::unload();
}

#[test]
#[serial]
fn text_extras() {
    common::load();

    // The cardinal form of a number, next to the ordinal form.
    assert_eq!(
        spice::repmct("There are # items", "#", 3, 'L'),
        "There are three items"
    );
    assert_eq!(
        spice::raw::repmot("The # item", "#", 3, 'L', 64),
        "The third item"
    );

    // Pulling a keyed value out of a command-like string.
    let (rest, found, value) = spice::kxtrct(
        "FROM",
        &["TO", "FROM", "BEGINNING", "ENDING"],
        "FROM 1 October 1984 TO 1 January 1987",
        128,
        64,
    );
    common::assert_ok("kxtrct");
    assert!(found);
    assert_eq!(value.trim(), "1 October 1984");
    assert_eq!(rest.trim(), "TO 1 January 1987");

    // The traceback names, alongside the depth.
    spice::chkin("outer");
    assert_eq!(spice::trcdep(), 1);
    assert_eq!(spice::trcnam(0), "outer");
    spice::chkout("outer");
    common::assert_ok("trcnam");

    common::unload();
}
