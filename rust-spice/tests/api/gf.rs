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
