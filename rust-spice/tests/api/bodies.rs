//! Body and surface name/code translation, and body constants from the pool.

use crate::assert_slice_eq;
use crate::common;

#[test]
#[serial]
fn bodc2n() {
    common::load();

    // Built-in identifiers need no kernel.
    assert_eq!(spice::bodc2n(399), ("EARTH".to_string(), true));
    assert_eq!(spice::bodc2n(10), ("SUN".to_string(), true));

    // The frame kernel adds our own.
    assert_eq!(
        spice::bodc2n(common::SPACECRAFT),
        ("TEST_SPACECRAFT".to_string(), true)
    );

    // An unknown code is reported as not found, with an empty name.
    let (name, found) = spice::bodc2n(-123_456_789);
    assert!(!found);
    assert!(name.is_empty());

    // The raw form honours the buffer size it is given.
    assert_eq!(spice::raw::bodc2n(399, 4), ("EAR".to_string(), true));

    common::unload();
}

#[test]
#[serial]
fn bodc2s() {
    common::load();

    assert_eq!(spice::bodc2s(399), "EARTH");
    // Without a name, the code itself is returned as a string.
    assert_eq!(spice::bodc2s(-123_456_789), "-123456789");

    common::unload();
}

#[test]
#[serial]
fn bodn2c() {
    common::load();

    assert_eq!(spice::bodn2c("EARTH"), (399, true));
    assert_eq!(spice::bodn2c("TEST_INSTRUMENT"), (common::INSTRUMENT, true));

    let (code, found) = spice::bodn2c("NOT A BODY");
    assert!(!found);
    assert_eq!(code, 0);

    common::unload();
}

#[test]
#[serial]
fn bods2c() {
    common::load();

    assert_eq!(spice::bods2c("EARTH"), (399, true));
    // Unlike `bodn2c`, a numeric string is accepted.
    assert_eq!(spice::bods2c("399"), (399, true));
    assert!(!spice::bods2c("NOT A BODY").1);

    common::unload();
}

#[test]
#[serial]
fn bodfnd() {
    common::load();

    let (target, found) = spice::bodn2c("EARTH");
    assert!(found);
    assert!(spice::bodfnd(target, "RADII"));
    assert!(spice::bodfnd(target, "GM"));
    assert!(!spice::bodfnd(target, "NO_SUCH_ITEM"));

    common::unload();
}

#[test]
#[serial]
fn bodvrd() {
    common::load();

    let radii = spice::bodvrd("EARTH", "RADII", 3);
    assert_slice_eq(&radii, &[6378.1366, 6378.1366, 6356.7519], 1e-9);

    // The vector is truncated to what CSPICE actually wrote.
    assert_eq!(spice::bodvrd("EARTH", "RADII", 10).len(), 3);
    // And it is empty when nothing is in the pool; CSPICE signals that as an error, which the
    // wrapper leaves in place for `errors::check` to pick up.
    assert!(spice::bodvrd("EARTH", "NO_SUCH_ITEM", 3).is_empty());
    assert!(spice::errors::check().is_err());

    common::unload();
}

#[test]
#[serial]
fn bodvcd() {
    common::load();

    // Same values, reached through the ID code instead of the name.
    assert_eq!(
        spice::bodvcd(399, "RADII", 3),
        spice::bodvrd("EARTH", "RADII", 3)
    );
    assert_slice_eq(&spice::bodvcd(10, "GM", 1), &[common::GM_CENTER], 1e-3);

    common::unload();
}

#[test]
#[serial]
fn surfaces() {
    common::load();

    // The text PCK associates surface 1 of body 399 with a name.
    assert_eq!(
        spice::srfs2c("TEST OCTAHEDRON", "EARTH"),
        (common::SURFACE, true)
    );
    assert_eq!(
        spice::srfscc("TEST OCTAHEDRON", 399),
        (common::SURFACE, true)
    );
    assert_eq!(
        spice::srfc2s(common::SURFACE, 399),
        ("TEST OCTAHEDRON".to_string(), true)
    );
    assert_eq!(
        spice::srfcss(common::SURFACE, "EARTH"),
        ("TEST OCTAHEDRON".to_string(), true)
    );

    // An unknown surface comes back as the code, flagged as not being a name.
    let (name, is_name) = spice::srfc2s(99, 399);
    assert!(!is_name);
    assert_eq!(name, "99");
    assert!(!spice::srfs2c("NO SUCH SURFACE", "EARTH").1);
    assert!(!spice::srfscc("NO SUCH SURFACE", 399).1);

    common::unload();
}
