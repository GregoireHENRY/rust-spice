//! The geometry finder, and the windows it reports its answers in.

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
