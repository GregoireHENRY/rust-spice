//! The geometry finder, and the windows it reports its answers in.

use crate::assert_slice_eq;
use crate::common;

#[test]
#[serial]
fn windows() {
    common::reset();

    let mut window = spice::Cell::<f64>::new(64);
    assert_eq!(spice::wncard(&mut window), 0);

    spice::wninsd(10.0, 20.0, &mut window);
    spice::wninsd(30.0, 40.0, &mut window);
    common::assert_ok("wninsd");
    assert_eq!(spice::wncard(&mut window), 2);
    assert_eq!(spice::wnfetd(&mut window, 0), (10.0, 20.0));
    assert_eq!(spice::wnfetd(&mut window, 1), (30.0, 40.0));

    // Overlapping insertions merge, which is what makes it a window rather than a list.
    spice::wninsd(15.0, 35.0, &mut window);
    assert_eq!(spice::wncard(&mut window), 1);
    assert_eq!(spice::wnfetd(&mut window, 0), (10.0, 40.0));

    // A window is a cell of doubles underneath: two endpoints per interval.
    assert_eq!(window.len(), 2);
    assert_eq!(window.to_vec(), vec![10.0, 40.0]);

    common::unload();
}

#[test]
#[serial]
fn gfoclt() {
    common::load();

    // The three bodies are coplanar, so the satellite crosses the line to the Sun once per
    // synodic period, about 14.75 days after the epoch.
    let mut confinement = spice::Cell::<f64>::new(64);
    spice::wninsd(0.0, common::LAST, &mut confinement);

    let found = spice::gfoclt(
        "ANY",
        "MOON",
        "ELLIPSOID",
        "IAU_MOON",
        "SUN",
        "ELLIPSOID",
        "IAU_SUN",
        "NONE",
        "EARTH",
        3600.0,
        &mut confinement,
    );
    common::assert_ok("gfoclt");

    let mut found = found;
    let intervals = spice::wncard(&mut found);
    assert!(
        intervals >= 1,
        "the satellite should cross the solar disc once"
    );

    let (start, stop) = spice::wnfetd(&mut found, 0);
    assert!(start < stop);
    assert!(
        (13.0 * 86400.0..16.0 * 86400.0).contains(&start),
        "expected the crossing around 14.75 days, got {}",
        start / 86400.0
    );

    // `occult` agrees at the middle of the interval, and disagrees outside it.
    let inside = spice::occult(
        "MOON",
        "ELLIPSOID",
        "IAU_MOON",
        "SUN",
        "ELLIPSOID",
        "IAU_SUN",
        "NONE",
        "EARTH",
        (start + stop) / 2.0,
    );
    assert_ne!(inside, 0, "the geometry finder and occult should agree");

    let outside = spice::occult(
        "MOON",
        "ELLIPSOID",
        "IAU_MOON",
        "SUN",
        "ELLIPSOID",
        "IAU_SUN",
        "NONE",
        "EARTH",
        0.0,
    );
    assert_eq!(outside, 0);

    // A confinement window holding nothing can hold no answer either.
    let mut empty = spice::Cell::<f64>::new(8);
    let mut nothing = spice::gfoclt(
        "ANY",
        "MOON",
        "ELLIPSOID",
        "IAU_MOON",
        "SUN",
        "ELLIPSOID",
        "IAU_SUN",
        "NONE",
        "EARTH",
        3600.0,
        &mut empty,
    );
    common::assert_ok("gfoclt over an empty confinement");
    assert_eq!(spice::wncard(&mut nothing), 0);

    common::unload();
}

#[test]
#[serial]
fn window_arithmetic() {
    common::reset();

    // Lengths 2, 4 and 3, so that the shortest and the longest are unambiguous.
    let mut a = spice::Cell::<f64>::new(64);
    let mut b = spice::Cell::<f64>::new(64);
    for (left, right) in [(1.0, 3.0), (7.0, 11.0), (23.0, 26.0)] {
        spice::wninsd(left, right, &mut a);
    }
    for (left, right) in [(2.0, 4.0), (8.0, 10.0), (16.0, 18.0)] {
        spice::wninsd(left, right, &mut b);
    }
    common::assert_ok("building the windows");

    let mut result = spice::Cell::<f64>::new(64);
    spice::wnunid(&mut a, &mut b, &mut result);
    assert_eq!(
        result.to_vec(),
        vec![1.0, 4.0, 7.0, 11.0, 16.0, 18.0, 23.0, 26.0]
    );

    spice::wnintd(&mut a, &mut b, &mut result);
    assert_eq!(result.to_vec(), vec![2.0, 3.0, 8.0, 10.0]);

    spice::wndifd(&mut a, &mut b, &mut result);
    assert_eq!(
        result.to_vec(),
        vec![1.0, 2.0, 7.0, 8.0, 10.0, 11.0, 23.0, 26.0]
    );

    spice::wncomd(0.0, 30.0, &mut a, &mut result);
    assert_eq!(
        result.to_vec(),
        vec![0.0, 1.0, 3.0, 7.0, 11.0, 23.0, 26.0, 30.0]
    );
    common::assert_ok("window set operations");

    // Membership.
    assert!(spice::wnelmd(2.0, &mut a));
    assert!(!spice::wnelmd(5.0, &mut a));
    assert!(spice::wnincd(1.5, 2.5, &mut a));
    assert!(!spice::wnincd(1.5, 5.0, &mut a));

    // Comparison.
    let mut same = a.clone();
    assert!(spice::wnreld(&mut a, "=", &mut same));
    assert!(!spice::wnreld(&mut a, "<>", &mut same));
    assert!(spice::wnreld(&mut a, "<>", &mut b));

    // Summary, over the lengths 2, 4 and 3. The two indices point at left endpoints in the flat
    // endpoint array, so the second interval is reported as 2 rather than as 1.
    let (measure, average, deviation, shortest, longest) = spice::wnsumd(&mut a);
    assert_relative_eq!(measure, 9.0, epsilon = 1e-12);
    assert_relative_eq!(average, 3.0, epsilon = 1e-12);
    assert_relative_eq!(deviation, (2.0f64 / 3.0).sqrt(), epsilon = 1e-12);
    assert_eq!(shortest, 0, "the interval of length 2 is the first");
    assert_eq!(longest, 2, "the interval of length 4 is the second");

    common::unload();
}

#[test]
#[serial]
fn window_reshaping() {
    common::reset();

    let mut window = spice::Cell::<f64>::new(64);
    for (left, right) in [(1.0, 3.0), (7.0, 11.0), (23.0, 27.0)] {
        spice::wninsd(left, right, &mut window);
    }

    // Expanding by one each way merges nothing here, but widens every interval.
    let mut expanded = window.clone();
    spice::wnexpd(1.0, 1.0, &mut expanded);
    assert_eq!(expanded.to_vec(), vec![0.0, 4.0, 6.0, 12.0, 22.0, 28.0]);

    // Contracting undoes it.
    spice::wncond(1.0, 1.0, &mut expanded);
    assert_eq!(expanded.to_vec(), window.to_vec());

    // Filling gaps shorter than five merges the first two intervals.
    let mut filled = window.clone();
    spice::wnfild(5.0, &mut filled);
    assert_eq!(filled.to_vec(), vec![1.0, 11.0, 23.0, 27.0]);

    // Filtering drops the intervals shorter than three.
    let mut filtered = window.clone();
    spice::wnfltd(3.0, &mut filtered);
    assert_eq!(filtered.to_vec(), vec![7.0, 11.0, 23.0, 27.0]);

    // Extracting keeps one endpoint of each interval.
    let mut left = window.clone();
    spice::wnextd('L', &mut left);
    assert_eq!(left.to_vec(), vec![1.0, 1.0, 7.0, 7.0, 23.0, 23.0]);

    let mut right = window.clone();
    spice::wnextd('R', &mut right);
    assert_eq!(right.to_vec(), vec![3.0, 3.0, 11.0, 11.0, 27.0, 27.0]);
    common::assert_ok("reshaping windows");

    common::unload();
}

/// Start of the confinement window. The searches that look at a rate difference the quantity over
/// a step of their own, so the window has to stop short of the edge of the coverage.
const BEGIN: f64 = common::FIRST + common::STEP;

/// End of the confinement window.
const END: f64 = common::LAST - common::STEP;

/// The confinement window every search below starts from: the whole span the kernels cover.
fn confinement() -> spice::Cell<f64> {
    span(BEGIN, END)
}

/// A window holding one interval.
fn span(start: f64, stop: f64) -> spice::Cell<f64> {
    let mut window = spice::Cell::<f64>::new(64);
    spice::wninsd(start, stop, &mut window);
    window
}

/// The midpoint of each interval of a window.
fn midpoints(window: &mut spice::Cell<f64>) -> Vec<f64> {
    (0..spice::wncard(window))
        .map(|index| {
            let (start, stop) = spice::wnfetd(window, index);
            (start + stop) / 2.0
        })
        .collect()
}

/**
The endpoints of a window that lie inside the confinement, so are crossings of the reference value
rather than the edges of the search.

The margin is there because a search does not always report the edge of the confinement to the
second: the coordinate searches offset it to keep the branch cut of a longitude out of the way.
*/
fn crossings(window: &mut spice::Cell<f64>) -> Vec<f64> {
    let mut found = Vec::new();
    for index in 0..spice::wncard(window) {
        let (start, stop) = spice::wnfetd(window, index);
        for et in [start, stop] {
            if et > BEGIN + 60.0 && et < END - 60.0 {
                found.push(et);
            }
        }
    }
    found
}

/// The state of `target` relative to `observer`, split into position and velocity.
fn state(target: &str, et: f64, observer: &str) -> ([f64; 3], [f64; 3]) {
    let (state, _) = spice::spkezr(target, et, "J2000", "NONE", observer);
    (
        [state[0], state[1], state[2]],
        [state[3], state[4], state[5]],
    )
}

#[test]
#[serial]
fn distance_and_range_rate() {
    common::load();

    // The satellite's distance from the centre swings about the target's orbit radius, once per
    // synodic period, so asking when it is the nearer of the two cuts the span into intervals.
    let mut cnfine = confinement();
    let mut closer = spice::gfdist(
        "MOON",
        "NONE",
        "SUN",
        "<",
        common::ORBIT_RADIUS,
        0.0,
        common::STEP,
        &mut cnfine,
    );
    common::assert_ok("gfdist");
    assert!(spice::wncard(&mut closer) >= 1, "the search found nothing");

    for et in midpoints(&mut closer) {
        let (position, _) = state("MOON", et, "SUN");
        assert!(
            spice::vnorm(position) < common::ORBIT_RADIUS,
            "inside an interval the distance should be below the reference value"
        );
    }
    for et in crossings(&mut closer) {
        let (position, _) = state("MOON", et, "SUN");
        assert_relative_eq!(spice::vnorm(position), common::ORBIT_RADIUS, epsilon = 1.0);
    }

    // The extrema of that distance are where the range rate changes sign.
    let mut cnfine = confinement();
    let mut approaching = spice::gfrr(
        "MOON",
        "NONE",
        "SUN",
        "<",
        0.0,
        0.0,
        common::STEP,
        &mut cnfine,
    );
    common::assert_ok("gfrr");
    assert!(spice::wncard(&mut approaching) >= 1);

    for et in midpoints(&mut approaching) {
        let (position, velocity) = state("MOON", et, "SUN");
        assert!(
            spice::vdot(position, velocity) < 0.0,
            "the satellite should be approaching at {et}"
        );
    }
    for et in crossings(&mut approaching) {
        let (position, velocity) = state("MOON", et, "SUN");
        let rate = spice::vdot(position, velocity) / spice::vnorm(position);
        assert_relative_eq!(rate, 0.0, epsilon = 1e-6);
    }

    // The absolute minimum of the distance is a single instant, reported as an empty interval.
    // Half the span, so that the two new moons it holds do not tie.
    let mut cnfine = span(0.0, END);
    let mut nearest = spice::gfdist(
        "MOON",
        "NONE",
        "SUN",
        "ABSMIN",
        0.0,
        0.0,
        common::STEP,
        &mut cnfine,
    );
    common::assert_ok("gfdist for the absolute minimum");
    assert_eq!(spice::wncard(&mut nearest), 1);
    let (start, stop) = spice::wnfetd(&mut nearest, 0);
    assert_relative_eq!(start, stop, epsilon = 1e-6);
    let (position, _) = state("MOON", start, "SUN");
    assert_relative_eq!(
        spice::vnorm(position),
        common::ORBIT_RADIUS - common::SATELLITE_RADIUS,
        epsilon = 1.0
    );

    common::unload();
}

#[test]
#[serial]
fn position_coordinates() {
    common::load();

    // The satellite orbits in the x-y plane of `J2000`, so its x coordinate relative to the target
    // is positive over half of each revolution: one interval inside the span.
    let mut cnfine = confinement();
    let mut ahead = spice::gfposc(
        "MOON",
        "J2000",
        "NONE",
        "EARTH",
        "RECTANGULAR",
        "X",
        ">",
        0.0,
        0.0,
        common::STEP,
        &mut cnfine,
    );
    common::assert_ok("gfposc");
    assert_eq!(spice::wncard(&mut ahead), 1);

    for et in midpoints(&mut ahead) {
        let (position, _) = state("MOON", et, "EARTH");
        assert!(position[0] > 0.0);
    }
    for et in crossings(&mut ahead) {
        let (position, _) = state("MOON", et, "EARTH");
        assert_relative_eq!(position[0], 0.0, epsilon = 1e-3);
    }

    common::unload();
}

#[test]
#[serial]
fn angles() {
    common::load();

    // The three bodies are coplanar, so the phase angle at the satellite runs from zero, when it
    // is behind the target, to pi when it passes between the target and the centre.
    let mut cnfine = confinement();
    let mut lit = spice::gfpa(
        "MOON",
        "SUN",
        "NONE",
        "EARTH",
        ">",
        std::f64::consts::FRAC_PI_2,
        0.0,
        common::STEP,
        &mut cnfine,
    );
    common::assert_ok("gfpa");
    assert!(spice::wncard(&mut lit) >= 1);

    for et in midpoints(&mut lit) {
        assert!(spice::phaseq(et, "MOON", "SUN", "EARTH", "NONE") > std::f64::consts::FRAC_PI_2);
    }
    for et in crossings(&mut lit) {
        assert_relative_eq!(
            spice::phaseq(et, "MOON", "SUN", "EARTH", "NONE"),
            std::f64::consts::FRAC_PI_2,
            epsilon = 1e-6
        );
    }

    // Seen from the centre, the two bodies are never further apart than the ratio of their orbit
    // radii, and they line up twice per synodic period.
    let widest = (common::SATELLITE_RADIUS / common::ORBIT_RADIUS).atan();
    let mut cnfine = confinement();
    let mut apart = spice::gfsep(
        "EARTH",
        "POINT",
        "J2000",
        "MOON",
        "POINT",
        "J2000",
        "NONE",
        "SUN",
        ">",
        0.75 * widest,
        0.0,
        common::STEP,
        &mut cnfine,
    );
    common::assert_ok("gfsep");
    assert!(spice::wncard(&mut apart) >= 1);

    let separation = |et: f64| {
        let (target, _) = state("EARTH", et, "SUN");
        let (satellite, _) = state("MOON", et, "SUN");
        spice::vsep(target, satellite)
    };
    for et in midpoints(&mut apart) {
        assert!(separation(et) > 0.75 * widest);
    }
    for et in crossings(&mut apart) {
        assert_relative_eq!(separation(et), 0.75 * widest, epsilon = 1e-9);
    }

    // The incidence angle at a point on the equator drops to zero once per rotation of the target,
    // which the kernels make just short of a day, so the span holds one window per day.
    let spoint = [common::SHAPE_RADIUS, 0.0, 0.0];
    let mut cnfine = confinement();
    let mut noon = spice::gfilum(
        "Ellipsoid",
        "INCIDENCE",
        "EARTH",
        "SUN",
        "IAU_EARTH",
        "NONE",
        "MOON",
        spoint,
        "<",
        0.3,
        0.0,
        common::STEP,
        &mut cnfine,
    );
    common::assert_ok("gfilum");
    assert!(
        (35..45).contains(&spice::wncard(&mut noon)),
        "one window per rotation over about forty days, got {}",
        spice::wncard(&mut noon)
    );

    let incidence = |et: f64| {
        let (_, _, _, incidence, _) = spice::ilumin(
            "Ellipsoid",
            "EARTH",
            et,
            "IAU_EARTH",
            "NONE",
            "MOON",
            spoint,
        );
        incidence
    };
    for et in midpoints(&mut noon) {
        assert!(incidence(et) < 0.3);
    }
    for et in crossings(&mut noon) {
        assert_relative_eq!(incidence(et), 0.3, epsilon = 1e-6);
    }

    common::unload();
}

#[test]
#[serial]
fn surface_points() {
    common::load();

    // The sub-observer point sweeps the whole range of longitudes once per rotation of the target.
    let mut cnfine = confinement();
    let mut eastern = spice::gfsubc(
        "EARTH",
        "IAU_EARTH",
        "Near point: ellipsoid",
        "NONE",
        "SUN",
        "LATITUDINAL",
        "LONGITUDE",
        ">",
        0.0,
        0.0,
        common::STEP,
        &mut cnfine,
    );
    common::assert_ok("gfsubc");
    assert!(
        (35..45).contains(&spice::wncard(&mut eastern)),
        "one window per rotation, got {}",
        spice::wncard(&mut eastern)
    );

    let sub_longitude = |et: f64| {
        let (spoint, _, _) = spice::subpnt(
            "Near point: ellipsoid",
            "EARTH",
            et,
            "IAU_EARTH",
            "NONE",
            "SUN",
        );
        spice::reclat(spoint).1
    };
    for et in midpoints(&mut eastern) {
        assert!(sub_longitude(et) > 0.0);
    }
    for et in crossings(&mut eastern) {
        assert_relative_eq!(sub_longitude(et).sin(), 0.0, epsilon = 1e-6);
    }

    // A ray cast at the centre from the target, in a frame that keeps pointing at it, always hits.
    // Its intercept walks round the centre once per rotation of the centre.
    let dvec = [1.0, 0.0, 0.0];
    let mut cnfine = confinement();
    let mut eastern = spice::gfsntc(
        "SUN",
        "IAU_SUN",
        "Ellipsoid",
        "NONE",
        "EARTH",
        "TEST_SUNWARD",
        dvec,
        "LATITUDINAL",
        "LONGITUDE",
        ">",
        0.0,
        0.0,
        common::STEP,
        &mut cnfine,
    );
    common::assert_ok("gfsntc");
    assert!(spice::wncard(&mut eastern) >= 1);

    let intercept_longitude = |et: f64| {
        let (spoint, _, _, found) = spice::sincpt(
            "Ellipsoid",
            "SUN",
            et,
            "IAU_SUN",
            "NONE",
            "EARTH",
            "TEST_SUNWARD",
            dvec,
        );
        assert!(found, "the ray should always reach the centre");
        spice::reclat(spoint).1
    };
    for et in midpoints(&mut eastern) {
        assert!(intercept_longitude(et) > 0.0);
    }
    // An interval ends either where the longitude passes zero or where it wraps at the anti
    // meridian, which the sine of it treats alike.
    for et in crossings(&mut eastern) {
        assert_relative_eq!(intercept_longitude(et).sin(), 0.0, epsilon = 1e-6);
    }

    common::unload();
}

#[test]
#[serial]
fn fields_of_view() {
    common::load();

    // The instrument frame is the identity, so a ray along +z in `J2000` is on the boresight for
    // the whole span and one along -z is never in view.
    let mut cnfine = confinement();
    let always = spice::gfrfov(
        "TEST_INSTRUMENT",
        [0.0, 0.0, 1.0],
        "J2000",
        "NONE",
        "TEST_SPACECRAFT",
        common::STEP,
        &mut cnfine,
    );
    common::assert_ok("gfrfov");
    assert_eq!(always.to_vec(), vec![BEGIN, END]);

    let mut cnfine = confinement();
    let mut never = spice::gfrfov(
        "TEST_INSTRUMENT",
        [0.0, 0.0, -1.0],
        "J2000",
        "NONE",
        "TEST_SPACECRAFT",
        common::STEP,
        &mut cnfine,
    );
    common::assert_ok("gfrfov looking the other way");
    assert_eq!(spice::wncard(&mut never), 0);

    // The telescope stares along the prime meridian of the target, so the satellite crosses its
    // field of view once per rotation. Its half angle is five degrees, which the target turns
    // through in a little under forty minutes, so the step has to be shorter than that.
    let mut cnfine = confinement();
    let mut rises = spice::gftfov(
        "TEST_TELESCOPE",
        "MOON",
        "POINT",
        "",
        "NONE",
        "EARTH",
        600.0,
        &mut cnfine,
    );
    common::assert_ok("gftfov");
    assert!(
        (35..45).contains(&spice::wncard(&mut rises)),
        "one pass per rotation over about forty days, got {}",
        spice::wncard(&mut rises)
    );

    for et in midpoints(&mut rises) {
        assert!(
            spice::fovtrg("TEST_TELESCOPE", "MOON", "POINT", "", "NONE", "EARTH", et),
            "the satellite should be in view at {et}"
        );
    }
    for et in crossings(&mut rises) {
        // A boundary is the moment the satellite is exactly on the edge, so a second either side
        // of it is out of view on one side and in view on the other.
        let before = spice::fovtrg(
            "TEST_TELESCOPE",
            "MOON",
            "POINT",
            "",
            "NONE",
            "EARTH",
            et - 1.0,
        );
        let after = spice::fovtrg(
            "TEST_TELESCOPE",
            "MOON",
            "POINT",
            "",
            "NONE",
            "EARTH",
            et + 1.0,
        );
        assert_ne!(before, after, "the field of view should change at {et}");
    }

    common::unload();
}

/// The x coordinate of the satellite relative to the target, as a scalar function of time.
unsafe extern "C" fn coordinate(et: f64, value: *mut f64) {
    let (position, _) = spice::spkpos("MOON", et, "J2000", "NONE", "EARTH");
    unsafe { *value = position[0] };
}

/// Whether [`coordinate`] is decreasing, worked out by differencing it.
unsafe extern "C" fn decreasing(
    udfuns: Option<spice::UdFuns>,
    et: f64,
    isdecr: *mut spice::c::SpiceBoolean,
) {
    let derivative = spice::uddc(
        udfuns.expect("the search passes its own function"),
        et,
        60.0,
    );
    unsafe { *isdecr = derivative as spice::c::SpiceBoolean };
}

/// Whether the satellite is on the +x side of the target.
unsafe extern "C" fn ahead(
    _udfuns: Option<spice::UdFuns>,
    et: f64,
    xbool: *mut spice::c::SpiceBoolean,
) {
    let (position, _) = spice::spkpos("MOON", et, "J2000", "NONE", "EARTH");
    unsafe { *xbool = (position[0] > 0.0) as spice::c::SpiceBoolean };
}

#[test]
#[serial]
fn user_defined_searches() {
    common::load();

    // The placeholder function is there to be passed where one is wanted and not used.
    assert_eq!(spice::udf(1.0), 0.0);

    // A day in, the satellite is a quarter of the way round, so its x coordinate is falling.
    let et = common::EPOCH;
    let (_, velocity) = state("MOON", et, "EARTH");
    assert!(velocity[0] < 0.0);
    assert!(spice::uddc(coordinate, et, 60.0));
    assert_relative_eq!(
        spice::uddf(coordinate, et, 60.0),
        velocity[0],
        epsilon = 1e-6
    );

    // The same search written three ways: as a coordinate of the position, as a scalar function
    // of time, and as a boolean function of time.
    let mut cnfine = confinement();
    let built_in = spice::gfposc(
        "MOON",
        "J2000",
        "NONE",
        "EARTH",
        "RECTANGULAR",
        "X",
        ">",
        0.0,
        0.0,
        common::STEP,
        &mut cnfine,
    )
    .to_vec();
    common::assert_ok("gfposc");

    let mut cnfine = confinement();
    let scalar = spice::gfuds(
        coordinate,
        decreasing,
        ">",
        0.0,
        0.0,
        common::STEP,
        &mut cnfine,
    )
    .to_vec();
    common::assert_ok("gfuds");

    let mut cnfine = confinement();
    let boolean = spice::gfudb(coordinate, ahead, common::STEP, &mut cnfine).to_vec();
    common::assert_ok("gfudb");

    assert_eq!(built_in.len(), 2, "one interval, so two endpoints");
    assert_slice_eq(&scalar, &built_in, 1e-6);
    assert_slice_eq(&boolean, &built_in, 1e-6);

    common::unload();
}

/// The ANSI C interrupt signal, which is 2 wherever CSPICE builds.
const SIGINT: i32 = 2;

/// The default disposition of a signal.
const SIG_DFL: usize = 0;

extern "C" {
    /// Used only to undo what [`spice::gfinth`] does to the process.
    fn signal(sig: i32, handler: usize) -> usize;
}

#[test]
#[serial]
fn stepping_and_interrupts() {
    common::load();

    // The step the default stepping routine reports is the one last set, whatever the epoch.
    spice::gfsstp(120.0);
    assert_eq!(spice::gfstep(0.0), 120.0);
    assert_eq!(spice::gfstep(common::EPOCH), 120.0);
    common::assert_ok("gfsstp and gfstep");

    // The default refinement halves the bracket, whichever way round it is given.
    assert_eq!(spice::gfrefn(10.0, 20.0, true, false), 15.0);
    assert_eq!(spice::gfrefn(20.0, 10.0, false, true), 15.0);
    common::assert_ok("gfrefn");

    // The progress reporter writes to the screen, so all there is to check is that the three
    // stages accept the window and the messages without complaint.
    let mut window = span(0.0, 100.0);
    spice::gfrepi(&mut window, "Working ", " done");
    spice::gfrepu(0.0, 100.0, 50.0);
    spice::gfrepf();
    common::assert_ok("the progress reporter");

    // Nothing has interrupted a search, and handing the handler the interrupt signal is what the
    // C signal handler would do.
    assert!(!spice::gfbail());
    spice::gfinth(SIGINT);
    common::assert_ok("gfinth");
    assert!(spice::gfbail());
    spice::gfclrh();
    assert!(!spice::gfbail());
    // `gfinth` made itself the handler for the signal; give the process its own back.
    unsafe { signal(SIGINT, SIG_DFL) };

    // The convergence tolerance is global, and it bounds how far a reported crossing can be from
    // the one a fine search reports.
    let crossing = |tolerance: f64| {
        spice::gfstol(tolerance);
        let mut cnfine = confinement();
        let mut window = spice::gfposc(
            "MOON",
            "J2000",
            "NONE",
            "EARTH",
            "RECTANGULAR",
            "X",
            ">",
            0.0,
            0.0,
            common::STEP,
            &mut cnfine,
        );
        common::assert_ok("gfposc under a set tolerance");
        spice::wnfetd(&mut window, 0).0
    };
    let coarse = crossing(600.0);
    let fine = crossing(1e-6);
    assert_relative_eq!(coarse, fine, epsilon = 600.0);

    // The widest integers CSPICE will work with.
    assert_eq!(spice::intmax(), i32::MAX);
    assert_eq!(spice::intmin(), i32::MIN);

    common::unload();
}

#[test]
#[serial]
fn searches_driven_by_the_caller() {
    common::load();

    // The stepping and refinement the searches below are handed are the ones the simpler forms
    // use internally, so each has to report what its simpler form reports.
    spice::gfsstp(common::STEP);
    spice::gfstol(1e-6);

    let mut cnfine = span(0.0, END);
    let occultations = spice::gfoclt(
        "ANY",
        "MOON",
        "ELLIPSOID",
        "IAU_MOON",
        "SUN",
        "ELLIPSOID",
        "IAU_SUN",
        "NONE",
        "EARTH",
        common::STEP,
        &mut cnfine,
    )
    .to_vec();
    common::assert_ok("gfoclt");
    assert!(!occultations.is_empty());

    let mut cnfine = span(0.0, END);
    let mut result = spice::Cell::<f64>::new(64);
    spice::raw::gfocce(
        "ANY",
        "MOON",
        "ELLIPSOID",
        "IAU_MOON",
        "SUN",
        "ELLIPSOID",
        "IAU_SUN",
        "NONE",
        "EARTH",
        1e-6,
        spice::c::gfstep_c,
        spice::c::gfrefn_c,
        false,
        spice::c::gfrepi_c,
        spice::c::gfrepu_c,
        spice::c::gfrepf_c,
        false,
        spice::c::gfbail_c,
        &mut cnfine,
        &mut result,
    );
    common::assert_ok("gfocce");
    assert_slice_eq(&result.to_vec(), &occultations, 1e-3);

    // The same, for the field of view of the telescope.
    spice::gfsstp(600.0);
    let mut cnfine = confinement();
    let passes = spice::gftfov(
        "TEST_TELESCOPE",
        "MOON",
        "POINT",
        "",
        "NONE",
        "EARTH",
        600.0,
        &mut cnfine,
    )
    .to_vec();
    common::assert_ok("gftfov");

    let mut cnfine = confinement();
    let mut result = spice::Cell::<f64>::new(256);
    spice::raw::gffove(
        "TEST_TELESCOPE",
        "POINT",
        [0.0, 0.0, 1.0],
        "MOON",
        "",
        "NONE",
        "EARTH",
        1e-6,
        spice::c::gfstep_c,
        spice::c::gfrefn_c,
        false,
        spice::c::gfrepi_c,
        spice::c::gfrepu_c,
        spice::c::gfrepf_c,
        false,
        spice::c::gfbail_c,
        &mut cnfine,
        &mut result,
    );
    common::assert_ok("gffove");
    assert_slice_eq(&result.to_vec(), &passes, 1e-3);

    // The general search names its quantity and its parameters as strings, and has to agree with
    // the one written for that quantity.
    spice::gfsstp(common::STEP);
    let mut cnfine = confinement();
    let distances = spice::gfdist(
        "MOON",
        "NONE",
        "SUN",
        "<",
        common::ORBIT_RADIUS,
        0.0,
        common::STEP,
        &mut cnfine,
    )
    .to_vec();
    common::assert_ok("gfdist");

    let mut cnfine = confinement();
    let mut result = spice::Cell::<f64>::new(64);
    spice::raw::gfevnt(
        spice::c::gfstep_c,
        spice::c::gfrefn_c,
        "DISTANCE",
        &["TARGET", "OBSERVER", "ABCORR"],
        &["MOON", "SUN", "NONE"],
        &[],
        &[],
        &[],
        "<",
        common::ORBIT_RADIUS,
        1e-6,
        0.0,
        false,
        spice::c::gfrepi_c,
        spice::c::gfrepu_c,
        spice::c::gfrepf_c,
        32,
        false,
        spice::c::gfbail_c,
        &mut cnfine,
        &mut result,
    );
    common::assert_ok("gfevnt");
    assert_slice_eq(&result.to_vec(), &distances, 1e-3);

    common::unload();
}

#[test]
#[serial]
fn validating_a_window() {
    common::reset();

    // A cell holding unordered, overlapping endpoints is not a window until it is validated.
    let mut window = spice::Cell::<f64>::new(16);
    for endpoint in [10.0, 12.0, 2.0, 7.0, 1.0, 5.0, 23.0, 29.0] {
        spice::appndd(endpoint, &mut window);
    }
    common::assert_ok("appndd");
    assert_eq!(window.len(), 8);

    spice::wnvald(16, 8, &mut window);
    common::assert_ok("wnvald");
    assert_eq!(window.to_vec(), vec![1.0, 7.0, 10.0, 12.0, 23.0, 29.0]);
    assert_eq!(spice::wncard(&mut window), 3);
}
