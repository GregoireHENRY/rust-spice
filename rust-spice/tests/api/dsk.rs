//! Digital shape kernels, and the DAS/DLA layers underneath them.

use crate::assert_slice_eq;
use crate::common;

/// Open the test DSK and position a search on its only segment.
fn segment() -> (i32, spice::DLADSC) {
    let handle = spice::dasopr(&common::kernel("test.bds"));
    common::assert_ok("dasopr");
    let (dladsc, found) = spice::dlabfs(handle);
    assert!(found, "the test DSK should hold one segment");
    (handle, dladsc)
}

#[test]
#[serial]
fn das_and_dla() {
    common::load();

    let (handle, first) = segment();
    assert!(handle != 0);

    // One segment only, so the forward and backward searches agree...
    let (last, found) = spice::dlabbs(handle);
    assert!(found);
    assert_eq!(first.ibase, last.ibase);
    assert_eq!(first.dbase, last.dbase);

    // ... and there is nothing after it.
    assert!(!spice::dlafns(handle, first).1);

    spice::dascls(handle);
    common::assert_ok("dascls");
    common::unload();
}

#[test]
#[serial]
fn dskgd() {
    common::load();

    let (handle, dladsc) = segment();
    let descriptor = spice::dskgd(handle, dladsc);

    assert_eq!(descriptor.surfce, common::SURFACE);
    assert_eq!(descriptor.center, common::TARGET);
    assert_eq!(descriptor.dtype, 2, "a type 2, plate model, segment");
    assert_eq!(descriptor.dclass, 2, "a general surface");
    assert_eq!(descriptor.corsys, 1, "latitudinal coordinates");
    assert_relative_eq!(descriptor.co3max, common::SHAPE_RADIUS, epsilon = 1e-9);
    assert_relative_eq!(descriptor.start, common::FIRST, epsilon = 1e-9);
    assert_relative_eq!(descriptor.stop, common::LAST, epsilon = 1e-9);

    spice::dascls(handle);
    common::unload();
}

#[test]
#[serial]
fn dskz02() {
    common::load();

    let (handle, dladsc) = segment();
    assert_eq!(spice::dskz02(handle, dladsc), (6, 8), "an octahedron");

    spice::dascls(handle);
    common::unload();
}

#[test]
#[serial]
fn dskv02() {
    common::load();

    let (handle, dladsc) = segment();

    let vertices = spice::dskv02(handle, dladsc);
    assert_eq!(vertices, common::octahedron_vertices());

    // The raw form takes a one-based start and a room, and truncates to what was read.
    assert_eq!(
        spice::raw::dskv02(handle, dladsc, 2, 2),
        vertices[1..3].to_vec()
    );
    assert_eq!(spice::raw::dskv02(handle, dladsc, 1, 100).len(), 6);

    spice::dascls(handle);
    common::unload();
}

#[test]
#[serial]
fn dskp02() {
    common::load();

    let (handle, dladsc) = segment();

    let plates = spice::dskp02(handle, dladsc);
    assert_eq!(plates, common::octahedron_plates());
    assert_eq!(
        spice::raw::dskp02(handle, dladsc, 1, 3),
        plates[..3].to_vec()
    );

    spice::dascls(handle);
    common::unload();
}

#[test]
#[serial]
fn dskn02() {
    common::load();

    let (handle, dladsc) = segment();

    // Plate 1 is the face of the octahedron facing the (+,+,+) octant.
    let normal = spice::dskn02(handle, dladsc, 1);
    let third = 1.0 / 3f64.sqrt();
    assert_slice_eq(&normal, &[third, third, third], 1e-12);

    // Every normal is a unit vector pointing away from the centre.
    for plate in 1..=8 {
        let normal = spice::dskn02(handle, dladsc, plate);
        assert_relative_eq!(spice::vnorm(normal), 1.0, epsilon = 1e-12);
    }

    spice::dascls(handle);
    common::unload();
}

#[test]
#[serial]
fn dskx02() {
    common::load();

    let (handle, dladsc) = segment();

    // A ray aimed at the centre from the (+,+,+) octant hits the plane x + y + z = r.
    let vertex = [3.0 * common::SHAPE_RADIUS; 3];
    let raydir = spice::vminus(spice::vhat(vertex));
    let (plate, point, found) = spice::dskx02(handle, dladsc, vertex, raydir);

    assert!(found);
    assert_eq!(plate, 1);
    assert_slice_eq(&point, &[common::SHAPE_RADIUS / 3.0; 3], 1e-9);

    // A ray pointing away from the body misses it.
    assert!(!spice::dskx02(handle, dladsc, vertex, spice::vhat(vertex)).2);

    spice::dascls(handle);
    common::unload();
}

#[test]
#[serial]
fn dskobj_and_dsksrf() {
    common::load();

    let dsk = common::kernel("test.bds");

    let bodies = spice::dskobj(&dsk);
    assert_eq!(bodies.to_vec(), vec![common::TARGET]);

    let surfaces = spice::dsksrf(&dsk, common::TARGET);
    assert_eq!(surfaces.to_vec(), vec![common::SURFACE]);

    // A body the file says nothing about has no surfaces.
    assert!(spice::dsksrf(&dsk, 499).is_empty());

    common::unload();
}

#[test]
#[serial]
fn tolerances() {
    common::load();

    let keyword = spice::raw::DSK_KEYXFR;
    let original = spice::dskgtl(keyword);
    common::assert_ok("dskgtl");
    assert!(
        original > 0.0,
        "the plate expansion fraction defaults to 1e-10"
    );

    spice::dskstl(keyword, original * 2.0);
    common::assert_ok("dskstl");
    assert_relative_eq!(spice::dskgtl(keyword), original * 2.0, epsilon = 1e-15);

    spice::dskstl(keyword, original);
    assert_relative_eq!(spice::dskgtl(keyword), original, epsilon = 1e-15);

    common::unload();
}

#[test]
#[serial]
fn write_and_read_back() {
    common::load();

    let path = common::kernel("written.bds");
    let _ = std::fs::remove_file(&path);

    // A single plate, so the expected contents are unambiguous.
    let vertices = vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    let plates = vec![[1, 2, 3]];

    let (spaixd, spaixi) = spice::dskmi2(
        &vertices, &plates, 0.1, 4, 100_000, 10_000, 10_000, true, 200_000,
    );
    common::assert_ok("dskmi2");
    assert_eq!(spaixd.len(), spice::raw::DSK02_SPADSZ);
    assert_eq!(spice::raw::DSK_NSYPAR, 10);

    let handle = spice::dskopn(&path, "rust-spice written DSK", 0);
    common::assert_ok("dskopn");
    spice::dskw02(
        handle,
        common::TARGET,
        2,
        2,
        "IAU_EARTH",
        1,
        &[0.0; 10],
        -std::f64::consts::PI,
        std::f64::consts::PI,
        -std::f64::consts::FRAC_PI_2,
        std::f64::consts::FRAC_PI_2,
        0.0,
        2.0,
        common::FIRST,
        common::LAST,
        &vertices,
        &plates,
        &spaixd,
        &spaixi,
    );
    spice::dskcls(handle, true);
    common::assert_ok("dskw02");

    let handle = spice::dasopr(&path);
    let (dladsc, found) = spice::dlabfs(handle);
    assert!(found);
    assert_eq!(spice::dskz02(handle, dladsc), (3, 1));
    assert_eq!(spice::dskv02(handle, dladsc), vertices);
    assert_eq!(spice::dskp02(handle, dladsc), plates);
    spice::dascls(handle);
    common::assert_ok("reading back the written DSK");

    common::unload();
    let _ = std::fs::remove_file(&path);
}

#[test]
#[serial]
#[should_panic(expected = "dskw02 needs")]
fn dskw02_rejects_too_few_coordinate_parameters() {
    common::load();

    spice::dskw02(
        1,
        common::TARGET,
        1,
        2,
        "IAU_EARTH",
        1,
        // CSPICE reads ten of these whatever the coordinate system.
        &[0.0; 3],
        0.0,
        1.0,
        0.0,
        1.0,
        0.0,
        1.0,
        common::FIRST,
        common::LAST,
        &[[0.0; 3]; 3],
        &[[1, 2, 3]],
        &[0.0; 10],
        &[0; 10],
    );
}
