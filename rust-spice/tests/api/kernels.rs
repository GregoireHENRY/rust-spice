//! The kernel writers and the low level readers that go with them.

use crate::assert_slice_eq;
use crate::common;

/// Write an SPK through `build`, load it, and hand the loaded path to `check`.
fn round_trip(name: &str, build: impl FnOnce(i32), check: impl FnOnce(&str)) {
    let path = common::kernel(name);
    let _ = std::fs::remove_file(&path);

    let handle = spice::spkopn(&path, "rust-spice writer test", 0);
    common::assert_ok("spkopn");
    build(handle);
    spice::spkcls(handle);
    common::assert_ok("writing the segment");

    spice::furnsh(&path);
    common::assert_ok("loading the written SPK");
    check(&path);
    spice::unload(&path);
    let _ = std::fs::remove_file(&path);
}

#[test]
#[serial]
fn spk_state_writers() {
    common::load();

    // A straight line, so every interpolation type should reproduce it exactly.
    let epochs = (0..16).map(|i| i as f64 * 100.0).collect::<Vec<f64>>();
    let states = epochs
        .iter()
        .map(|&t| [t, 2.0 * t, 3.0 * t, 1.0, 2.0, 3.0])
        .collect::<Vec<[f64; 6]>>();
    let expected = [500.0, 1000.0, 1500.0, 1.0, 2.0, 3.0];

    // Type 5 propagates two body motion between the states it is given, so write it with states
    // that obey it and check at one of them.
    let orbit_epochs = common::epochs()[..8].to_vec();
    let orbit_states = orbit_epochs
        .iter()
        .map(|&et| common::analytic_state(et))
        .collect::<Vec<_>>();
    round_trip(
        "w05.bsp",
        |handle| {
            spice::spkw05(
                handle,
                common::SPACECRAFT,
                common::CENTER,
                "J2000",
                orbit_epochs[0],
                orbit_epochs[7],
                "TYPE 5",
                common::GM_CENTER,
                8,
                &orbit_states,
                &orbit_epochs,
            )
        },
        |_| {
            let (state, _) =
                spice::spkezr("TEST_SPACECRAFT", orbit_epochs[3], "J2000", "NONE", "SUN");
            assert_slice_eq(&state, &orbit_states[3], 1e-6);
        },
    );

    // Types 8 and 12 take evenly spaced states.
    for (name, write) in [("w08.bsp", 8), ("w12.bsp", 12)] {
        round_trip(
            name,
            |handle| {
                if write == 8 {
                    spice::spkw08(
                        handle,
                        common::SPACECRAFT,
                        common::TARGET,
                        "J2000",
                        epochs[0],
                        epochs[15],
                        "TYPE 8",
                        3,
                        16,
                        &states,
                        epochs[0],
                        100.0,
                    )
                } else {
                    spice::spkw12(
                        handle,
                        common::SPACECRAFT,
                        common::TARGET,
                        "J2000",
                        epochs[0],
                        epochs[15],
                        "TYPE 12",
                        3,
                        16,
                        &states,
                        epochs[0],
                        100.0,
                    )
                }
            },
            |_| {
                let (state, _) = spice::spkezr("TEST_SPACECRAFT", 500.0, "J2000", "NONE", "EARTH");
                assert_slice_eq(&state, &expected, 1e-6);
            },
        );
    }

    // Type 13 takes unevenly spaced states.
    round_trip(
        "w13.bsp",
        |handle| {
            spice::spkw13(
                handle,
                common::SPACECRAFT,
                common::TARGET,
                "J2000",
                epochs[0],
                epochs[15],
                "TYPE 13",
                3,
                16,
                &states,
                &epochs,
            )
        },
        |_| {
            let (state, _) = spice::spkezr("TEST_SPACECRAFT", 500.0, "J2000", "NONE", "EARTH");
            assert_slice_eq(&state, &expected, 1e-6);
        },
    );

    common::unload();
}

#[test]
#[serial]
fn spk_chebyshev_writers() {
    common::load();

    // A degree zero expansion is a constant, one interval a second.
    // Type 2 stores position only, so three coefficients per interval.
    let position = [100.0, 200.0, 300.0];
    round_trip(
        "w02.bsp",
        |handle| {
            spice::spkw02(
                handle,
                common::SPACECRAFT,
                common::TARGET,
                "J2000",
                0.0,
                10.0,
                "TYPE 2",
                1.0,
                10,
                0,
                &position.repeat(10),
                0.0,
            )
        },
        |_| {
            let (got, _) = spice::spkpos("TEST_SPACECRAFT", 5.0, "J2000", "NONE", "EARTH");
            assert_slice_eq(&got, &position, 1e-9);
        },
    );

    // Type 3 stores position and velocity, so six coefficients per interval.
    let state = [100.0, 200.0, 300.0, 0.0, 0.0, 0.0];
    round_trip(
        "w03.bsp",
        |handle| {
            spice::spkw03(
                handle,
                common::SPACECRAFT,
                common::TARGET,
                "J2000",
                0.0,
                10.0,
                "TYPE 3",
                1.0,
                10,
                0,
                &state.repeat(10),
                0.0,
            )
        },
        |_| {
            let (got, _) = spice::spkezr("TEST_SPACECRAFT", 5.0, "J2000", "NONE", "EARTH");
            assert_slice_eq(&got, &state, 1e-9);
        },
    );

    common::unload();
}

#[test]
#[serial]
fn spk_segment_descriptors() {
    common::load();

    // Build a descriptor, then take it apart again.
    let descr = spice::spkpds(common::TARGET, common::CENTER, "J2000", 9, -100.0, 100.0);
    common::assert_ok("spkpds");
    assert_eq!(descr.len(), spice::raw::SPK_DSCSIZ);

    let (body, center, frame, kind, first, last, _, _) = spice::spkuds(&descr);
    common::assert_ok("spkuds");
    assert_eq!((body, center, kind), (common::TARGET, common::CENTER, 9));
    assert_eq!(frame, spice::namfrm("J2000"));
    assert_relative_eq!(first, -100.0, epsilon = 1e-9);
    assert_relative_eq!(last, 100.0, epsilon = 1e-9);

    // And find the segment of the loaded SPK that covers the target.
    let (handle, descr, ident, found) = spice::spksfs(common::TARGET, common::EPOCH);
    common::assert_ok("spksfs");
    assert!(found);
    assert!(handle != 0);
    assert_eq!(ident, "TEST ORBIT");
    let (body, center, _, kind, _, _, _, _) = spice::spkuds(&descr);
    assert_eq!((body, center, kind), (common::TARGET, common::CENTER, 9));

    // Evaluating the segment directly gives the same state the high level reader does.
    let (frame, state, center) = spice::spkpvn(handle, descr, common::EPOCH);
    common::assert_ok("spkpvn");
    assert_eq!(center, common::CENTER);
    assert_eq!(frame, spice::namfrm("J2000"));
    let (reference, _) = spice::spkezr("EARTH", common::EPOCH, "J2000", "NONE", "SUN");
    assert_slice_eq(&state, &reference, 1e-9);

    common::unload();
}

#[test]
#[serial]
fn spk_low_level_readers() {
    common::load();

    let et = common::EPOCH;

    // The geometric state relative to the solar system barycentre, and the position form.
    let ssb = spice::spkssb(common::TARGET, et, "J2000");
    common::assert_ok("spkssb");
    let (geometric, _) = spice::spkgeo(common::TARGET, et, "J2000", common::CENTER);
    let centre = spice::spkssb(common::CENTER, et, "J2000");
    let relative = (0..6).map(|i| ssb[i] - centre[i]).collect::<Vec<_>>();
    assert_slice_eq(&relative, &geometric, 1e-6);

    let (position, lt) = spice::spkgps(common::TARGET, et, "J2000", common::CENTER);
    assert_slice_eq(&position, &geometric[..3], 1e-9);
    assert!(lt > 0.0);

    // The corrected forms agree with `spkez` for the same correction.
    let (corrected, lt, dlt) = spice::spkacs(common::TARGET, et, "J2000", "LT", common::CENTER);
    common::assert_ok("spkacs");
    let (reference, reference_lt) = spice::spkez(common::TARGET, et, "J2000", "LT", common::CENTER);
    assert_slice_eq(&corrected, &reference, 1e-9);
    assert_relative_eq!(lt, reference_lt, epsilon = 1e-12);
    assert!(dlt.abs() < 1.0);

    // And the forms that take the observer's state rather than its ID.
    let observer = spice::spkssb(common::CENTER, et, "J2000");
    let (state, lt, _) = spice::spkltc(common::TARGET, et, "J2000", "LT", observer);
    common::assert_ok("spkltc");
    assert_slice_eq(&state, &corrected, 1e-9);
    assert!(lt > 0.0);

    let (apparent, _) = spice::spkapo(common::TARGET, et, "J2000", observer, "LT");
    assert_slice_eq(&apparent, &state[..3], 1e-9);
    let (apparent_state, _) = spice::spkapp(common::TARGET, et, "J2000", observer, "LT");
    assert_slice_eq(&apparent_state[..3], &apparent, 1e-9);

    common::unload();
}

#[test]
#[serial]
fn spk_load_and_unload() {
    common::reset();

    // `spklef` loads an SPK outside the KEEPER subsystem.
    let handle = spice::spklef(&common::kernel("test.bsp"));
    common::assert_ok("spklef");
    assert!(handle != 0);
    assert_eq!(spice::ktotal("SPK"), 0, "it does not go through KEEPER");

    let (_, _, _, found) = spice::spksfs(common::TARGET, common::EPOCH);
    assert!(found);

    spice::spkuef(handle);
    common::assert_ok("spkuef");
    common::unload();
}

#[test]
#[serial]
fn ck_writers() {
    common::load();

    let path = common::kernel("written01.bc");
    let _ = std::fs::remove_file(&path);
    let instrument = -999_200;

    // Type 1 is discrete pointing, one record per epoch.
    let ticks = (0..4).map(|i| i as f64 * 1000.0).collect::<Vec<f64>>();
    let quats = vec![[1.0, 0.0, 0.0, 0.0]; 4];
    let avvs = vec![[0.0; 3]; 4];

    let handle = spice::ckopn(&path, "type 1", 0);
    spice::ckw01(
        handle, ticks[0], ticks[3], instrument, "J2000", true, "TYPE 1", 4, &ticks, &quats, &avvs,
    );
    spice::ckcls(handle);
    common::assert_ok("ckw01");

    assert_eq!(spice::ckobj(&path).to_vec(), vec![instrument]);
    spice::furnsh(&path);
    let (matrix, _, found) = spice::ckgp(instrument, 1500.0, 1000.0, "J2000");
    common::assert_ok("reading the type 1 CK");
    assert!(found);
    crate::assert_matrix_eq(&matrix, &spice::ident(), 1e-14);
    spice::unload(&path);
    let _ = std::fs::remove_file(&path);

    // Type 2 holds a constant angular velocity over each interval.
    let path = common::kernel("written02.bc");
    let _ = std::fs::remove_file(&path);
    let starts = vec![0.0, 2000.0];
    let stops = vec![1000.0, 3000.0];
    let quats = vec![[1.0, 0.0, 0.0, 0.0]; 2];
    let avvs = vec![[0.0; 3]; 2];
    let rates = vec![1.0, 1.0];

    let handle = spice::ckopn(&path, "type 2", 0);
    spice::ckw02(
        handle, 0.0, 3000.0, instrument, "J2000", "TYPE 2", 2, &starts, &stops, &quats, &avvs,
        &rates,
    );
    spice::ckcls(handle);
    common::assert_ok("ckw02");
    assert_eq!(spice::ckobj(&path).to_vec(), vec![instrument]);

    common::unload();
    let _ = std::fs::remove_file(&path);
}

#[test]
#[serial]
fn dsk_low_level_reads() {
    common::load();

    let handle = spice::dasopr(&common::kernel("test.bds"));
    let (dladsc, _) = spice::dlabfs(handle);

    // The bookkeeping parameters agree with the octahedron the fixture wrote.
    let (nv, np, _, _, voxsiz, _, _, _, _, _, _) = spice::dskb02(handle, dladsc);
    common::assert_ok("dskb02");
    assert_eq!((nv, np), (6, 8));
    assert!(voxsiz > 0.0);

    // The plate array, three integers per plate. `start` counts from zero here.
    let plates = spice::dski02(handle, dladsc, spice::dsk02::KWPLAT, 0, 24);
    common::assert_ok("dski02");
    assert_eq!(plates.len(), 24);
    // Every index names one of the six vertices.
    assert!(plates.iter().all(|&i| (1..=6).contains(&i)));

    // Starting one in drops the first index.
    assert_eq!(
        spice::dski02(handle, dladsc, spice::dsk02::KWPLAT, 1, 24).len(),
        23
    );

    // The counts, as single valued items.
    assert_eq!(
        spice::dski02(handle, dladsc, spice::dsk02::KWNV, 0, 1),
        vec![6]
    );
    assert_eq!(
        spice::dski02(handle, dladsc, spice::dsk02::KWNP, 0, 1),
        vec![8]
    );
    common::assert_ok("the single valued items");

    // Each keyword belongs to one of the two readers; asking the wrong one is an error.
    assert!(spice::dskd02(handle, dladsc, spice::dsk02::KWNV, 0, 1).is_empty());
    assert!(spice::errors::check().is_err());

    // The vertices, three doubles each. CSPICE reorders them when it builds the spatial index,
    // so compare the set rather than the order.
    let mut got = spice::dskd02(handle, dladsc, spice::dsk02::KWVERT, 0, 18)
        .chunks(3)
        .map(|v| [v[0], v[1], v[2]])
        .collect::<Vec<_>>();
    common::assert_ok("dskd02");
    let mut expected = common::octahedron_vertices();
    let key = |v: &[f64; 3]| (v[0].to_bits(), v[1].to_bits(), v[2].to_bits());
    got.sort_by_key(key);
    expected.sort_by_key(key);
    assert_eq!(got, expected);

    spice::dascls(handle);

    // The vertical extent of the plate set, in latitudinal coordinates.
    let (mncor3, mxcor3) = spice::dskrb2(
        &common::octahedron_vertices(),
        &common::octahedron_plates(),
        1,
        &[0.0; 10],
    );
    common::assert_ok("dskrb2");
    assert!(mncor3 > 0.0 && mxcor3 > mncor3);
    assert_relative_eq!(mxcor3, common::SHAPE_RADIUS, max_relative = 1e-6);

    common::unload();
}

#[test]
#[serial]
fn spacecraft_clock_formatting() {
    common::load();

    // The string form of a tick count, and the partitions of the clock.
    let ticks = common::ticks(common::EPOCH);
    let formatted = spice::raw::scfmt(common::SPACECRAFT, ticks, spice::MAX_LEN_OUT as i32);
    common::assert_ok("scfmt");
    assert!(!formatted.is_empty());
    // `scdecd` adds the partition number, `scfmt` does not.
    assert!(spice::scdecd(common::SPACECRAFT, ticks).ends_with(&formatted));

    let (starts, stops) = spice::scpart(common::SPACECRAFT, 16);
    common::assert_ok("scpart");
    assert_eq!(starts.len(), 1, "the test clock has one partition");
    assert_eq!(stops.len(), 1);
    assert!(stops[0] > starts[0]);

    common::unload();
}
