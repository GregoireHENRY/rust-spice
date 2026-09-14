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

    // `spkaps` is `spkltc` with stellar aberration, so it needs the observer's acceleration as
    // well as its state. With no correction asked for neither is used, and the state is the
    // geometric one, though the light time is still reported.
    let (state, lt, dlt) = spice::spkaps(common::TARGET, et, "J2000", "NONE", observer, [0.0; 3]);
    common::assert_ok("spkaps");
    assert_slice_eq(&state, &geometric, 1e-9);
    let (_, geometric_lt) = spice::spkgeo(common::TARGET, et, "J2000", common::CENTER);
    assert_relative_eq!(lt, geometric_lt, epsilon = 1e-12);
    assert!(dlt.abs() < 1.0);

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

    // The records of the segment can be read back one at a time, through the descriptor the DAF
    // search reports.
    let descriptor = first_segment(&path);
    let reader = spice::dafopr(&path);
    spice::dafbfs(reader);
    assert!(spice::daffna());
    assert_eq!(spice::cknr02(reader, &descriptor), 2);
    common::assert_ok("cknr02");
    let record = spice::ckgr02(reader, &descriptor, 1, 10);
    common::assert_ok("ckgr02");
    assert_eq!(record.len(), 10);
    assert_slice_eq(&record[..3], &[starts[0], stops[0], rates[0]], 1e-9);
    assert_slice_eq(&record[3..7], &quats[0], 1e-15);
    spice::dafcls(reader);
    spice::unload(&path);
    let _ = std::fs::remove_file(&path);

    // Type 3 holds discrete pointing with linear interpolation inside each interval.
    let path = common::kernel("written03.bc");
    let _ = std::fs::remove_file(&path);
    let ticks = vec![0.0, 1000.0, 2000.0];
    let quats = vec![[1.0, 0.0, 0.0, 0.0]; 3];
    let avvs = vec![[0.0; 3]; 3];

    let handle = spice::ckopn(&path, "type 3", 0);
    spice::ckw03(
        handle,
        ticks[0],
        ticks[2],
        instrument,
        "J2000",
        true,
        "TYPE 3",
        3,
        &ticks,
        &quats,
        &avvs,
        1,
        &[ticks[0]],
    );
    spice::ckcls(handle);
    common::assert_ok("ckw03");

    let descriptor = first_segment(&path);
    let reader = spice::dafopr(&path);
    spice::dafbfs(reader);
    assert!(spice::daffna());
    assert_eq!(spice::cknr03(reader, &descriptor), 3);
    common::assert_ok("cknr03");
    let record = spice::ckgr03(reader, &descriptor, 2, 8);
    common::assert_ok("ckgr03");
    assert_relative_eq!(record[0], ticks[1], epsilon = 1e-9);
    assert_slice_eq(&record[1..5], &quats[1], 1e-15);
    spice::dafcls(reader);
    let _ = std::fs::remove_file(&path);

    // Type 5 interpolates the quaternions themselves; subtype 1 carries nothing else.
    let path = common::kernel("written05.bc");
    let _ = std::fs::remove_file(&path);
    let packets = [1.0, 0.0, 0.0, 0.0].repeat(2);

    let handle = spice::ckopn(&path, "type 5", 0);
    spice::ckw05(
        handle,
        1,
        1,
        ticks[0],
        ticks[2],
        instrument,
        "J2000",
        true,
        "TYPE 5",
        2,
        &[ticks[0], ticks[2]],
        &packets,
        1.0,
        1,
        &[ticks[0]],
    );
    spice::ckcls(handle);
    common::assert_ok("ckw05");

    spice::furnsh(&path);
    let (matrix, _, found) = spice::ckgp(instrument, ticks[1], 1000.0, "J2000");
    common::assert_ok("reading the type 5 CK");
    assert!(found);
    crate::assert_matrix_eq(&matrix, &spice::ident(), 1e-12);

    common::unload();
    let _ = std::fs::remove_file(&path);
}

/// The descriptor of the first segment of a DAF, which the low level readers are addressed by.
fn first_segment(path: &str) -> Vec<f64> {
    let handle = spice::dafopr(path);
    spice::dafbfs(handle);
    assert!(spice::daffna(), "the file should hold a segment");
    let summary = spice::dafgs();
    spice::dafcls(handle);
    summary
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

#[test]
#[serial]
fn spk_conic_and_equinoctial_writers() {
    common::load();

    // A circular, equatorial orbit of ten thousand kilometres about the target, which both of
    // these describe in their own terms.
    let radius: f64 = 10_000.0;
    let rate = (common::GM_TARGET / radius.powi(3)).sqrt();
    let expected = [radius, 0.0, 0.0, 0.0, radius * rate, 0.0];

    // Type 15 takes the periapsis, the poles and the shape of the conic. A flag of three asks for
    // no precession, so the constants that would drive it are not used.
    round_trip(
        "w15.bsp",
        |handle| {
            spice::spkw15(
                handle,
                common::SPACECRAFT,
                common::TARGET,
                "J2000",
                0.0,
                86400.0,
                "TYPE 15",
                0.0,
                [0.0, 0.0, 1.0],
                [1.0, 0.0, 0.0],
                radius,
                0.0,
                3.0,
                [0.0, 0.0, 1.0],
                common::GM_TARGET,
                0.0,
                0.0,
            )
        },
        |_| {
            let (state, _) = spice::spkezr("TEST_SPACECRAFT", 0.0, "J2000", "NONE", "EARTH");
            assert_slice_eq(&state, &expected, 1e-6);
        },
    );

    // Type 17 takes the same orbit as equinoctial elements: a circular orbit has no eccentricity
    // and no inclination, so only the semi-major axis and the mean longitude rate are not zero.
    let mut elements = [0.0; 9];
    elements[0] = radius;
    elements[7] = rate;
    round_trip(
        "w17.bsp",
        |handle| {
            spice::spkw17(
                handle,
                common::SPACECRAFT,
                common::TARGET,
                "J2000",
                0.0,
                86400.0,
                "TYPE 17",
                0.0,
                elements,
                0.0,
                std::f64::consts::FRAC_PI_2,
            )
        },
        |_| {
            // With the pole of the central body along +z, the equatorial frame the elements are
            // given in has no preferred x axis, so it is the shape of the orbit that is checked
            // rather than where on it the body starts.
            let (state, _) = spice::spkezr("TEST_SPACECRAFT", 0.0, "J2000", "NONE", "EARTH");
            let (position, velocity) = (
                [state[0], state[1], state[2]],
                [state[3], state[4], state[5]],
            );
            assert_relative_eq!(spice::vnorm(position), radius, epsilon = 1e-6);
            assert_relative_eq!(spice::vnorm(velocity), radius * rate, epsilon = 1e-9);
            assert_relative_eq!(spice::vdot(position, velocity), 0.0, epsilon = 1e-6);
            // An equatorial orbit, so nothing leaves the plane.
            assert_relative_eq!(position[2], 0.0, epsilon = 1e-9);
            assert_relative_eq!(velocity[2], 0.0, epsilon = 1e-12);
        },
    );

    common::unload();
}

#[test]
#[serial]
fn spk_chebyshev_and_hermite_writers() {
    common::load();

    // Type 14 is type 2 and 3 written a set at a time: each set opens with the midpoint and the
    // radius of the interval it covers, then the coefficients for each component. A degree zero
    // expansion is a constant.
    let position = [100.0, 200.0, 300.0];
    round_trip(
        "w14.bsp",
        |handle| {
            spice::spk14b(
                handle,
                "TYPE 14",
                common::SPACECRAFT,
                common::TARGET,
                "J2000",
                0.0,
                86400.0,
                0,
            );
            spice::spk14a(
                handle,
                1,
                &[43200.0, 43200.0, 100.0, 200.0, 300.0, 0.0, 0.0, 0.0],
                &[0.0],
            );
            spice::spk14e(handle);
        },
        |_| {
            let (state, _) = spice::spkezr("TEST_SPACECRAFT", 3600.0, "J2000", "NONE", "EARTH");
            assert_slice_eq(&state[..3], &position, 1e-9);
            assert_slice_eq(&state[3..], &[0.0; 3], 1e-12);
        },
    );

    // Type 18, subtype 1, is Lagrange interpolation of six element state packets. A straight line
    // is reproduced exactly by the first degree polynomial.
    let epochs = (0..8)
        .map(|index| index as f64 * 100.0)
        .collect::<Vec<f64>>();
    let packets = epochs
        .iter()
        .flat_map(|&t| [t, 2.0 * t, 3.0 * t, 1.0, 2.0, 3.0])
        .collect::<Vec<f64>>();
    round_trip(
        "w18.bsp",
        |handle| {
            spice::spkw18(
                handle,
                1,
                common::SPACECRAFT,
                common::TARGET,
                "J2000",
                epochs[0],
                epochs[7],
                "TYPE 18",
                1,
                epochs.len() as i32,
                &packets,
                &epochs,
            )
        },
        |_| {
            let (state, _) = spice::spkezr("TEST_SPACECRAFT", 250.0, "J2000", "NONE", "EARTH");
            assert_slice_eq(&state, &[250.0, 500.0, 750.0, 1.0, 2.0, 3.0], 1e-9);
        },
    );

    // Type 20 holds Chebyshev coefficients for the velocity and the position at each interval
    // midpoint, over intervals measured in Julian days from a Julian date. A velocity of zero
    // leaves the position at the midpoint value.
    round_trip(
        "w20.bsp",
        |handle| {
            spice::spkw20(
                handle,
                common::SPACECRAFT,
                common::TARGET,
                "J2000",
                0.0,
                86400.0,
                "TYPE 20",
                1.0,
                1,
                0,
                &[0.0, 100.0, 0.0, 200.0, 0.0, 300.0],
                1.0,
                1.0,
                spice::j2000(),
                0.0,
            )
        },
        |_| {
            let (state, _) = spice::spkezr("TEST_SPACECRAFT", 43200.0, "J2000", "NONE", "EARTH");
            assert_slice_eq(&state[..3], &position, 1e-9);
            assert_slice_eq(&state[3..], &[0.0; 3], 1e-12);
        },
    );

    common::unload();
}

/// A two-line element set, and the geophysical constants its propagator wants.
const TLE: [&str; 2] = [
    "1 43908U 18111AJ  20146.60805006  .00000806  00000-0  34965-4 0  9999",
    "2 43908  97.2676  47.2136 0020001 220.6050 139.3698 15.24999521 78544",
];
const GEOPHS: [f64; 8] = [
    1.082616e-3,
    -2.53881e-6,
    -1.65597e-6,
    7.43669161e-2,
    120.0,
    78.0,
    6378.135,
    1.0,
];

#[test]
#[serial]
fn spk_two_line_element_writer() {
    common::load();

    let (epoch, elements) = spice::getelm(1957, &TLE);
    common::assert_ok("getelm");
    let expected = spice::evsgp4(epoch, GEOPHS, elements);

    // Type 10 stores the element set itself, and evaluates it with the same propagator.
    round_trip(
        "w10.bsp",
        |handle| {
            spice::spkw10(
                handle,
                common::SPACECRAFT,
                common::TARGET,
                "J2000",
                epoch - 3600.0,
                epoch + 3600.0,
                "TYPE 10",
                &GEOPHS,
                1,
                &elements,
                &[epoch],
            )
        },
        |_| {
            // `evsgp4` reports the state in the true equator and equinox of date, which the type
            // 10 reader turns into the frame the segment declares. So the two differ by the
            // rotation between the two frames, and agree on everything that rotation preserves.
            let (state, _) = spice::spkezr("TEST_SPACECRAFT", epoch, "J2000", "NONE", "EARTH");
            let written = (
                [state[0], state[1], state[2]],
                [state[3], state[4], state[5]],
            );
            let propagated = (
                [expected[0], expected[1], expected[2]],
                [expected[3], expected[4], expected[5]],
            );
            assert_relative_eq!(
                spice::vnorm(written.0),
                spice::vnorm(propagated.0),
                epsilon = 1e-6
            );
            // The frame turns as well, so the speeds agree to a little less than the distances.
            assert_relative_eq!(
                spice::vnorm(written.1),
                spice::vnorm(propagated.1),
                epsilon = 1e-6
            );

            // The angle between the position and the velocity survives the change of frame, as
            // anything rigid must, which is what says the orbit itself is the same one.
            assert_relative_eq!(
                spice::vsep(written.0, written.1),
                spice::vsep(propagated.0, propagated.1),
                epsilon = 1e-6
            );

            // And the two frames are two decades of precession apart, no more.
            let turned = spice::vsep(written.0, propagated.0);
            assert!(
                (1e-4..1e-2).contains(&turned),
                "expected a small rotation, got {} degrees",
                turned.to_degrees()
            );
        },
    );

    common::unload();
}

#[test]
#[serial]
fn spk_segment_subset() {
    common::load();

    // The first segment of the test ephemeris, and its summary, which is what addresses it.
    let source = spice::dafopr(&common::kernel("test.bsp"));
    spice::dafbfs(source);
    assert!(spice::daffna());
    let mut summary = spice::dafgs();
    let ident = spice::dafgn(spice::MAX_LEN_OUT as i32);
    assert_eq!(ident, "TEST ORBIT");

    // Copy a day either side of the epoch into a file of its own.
    let path = common::kernel("subset.bsp");
    let _ = std::fs::remove_file(&path);
    let target = spice::spkopn(&path, "rust-spice subset", 0);
    spice::spksub(source, &mut summary, &ident, -86400.0, 86400.0, target);
    common::assert_ok("spksub");
    spice::spkcls(target);
    spice::dafcls(source);

    // It covers what it was asked for, and agrees with the file it came from.
    let coverage = spice::spkcov(&path, common::TARGET);
    assert_relative_eq!(coverage.get(0).unwrap(), -86400.0, epsilon = 1e-6);
    assert_relative_eq!(coverage.get(1).unwrap(), 86400.0, epsilon = 1e-6);

    let (whole, _) = spice::spkezr("EARTH", common::EPOCH, "J2000", "NONE", "SUN");
    spice::kclear();
    spice::furnsh(&path);
    let (part, _) = spice::spkezr("EARTH", common::EPOCH, "J2000", "NONE", "SUN");
    common::assert_ok("reading the subset back");
    assert_slice_eq(&part, &whole, 1e-9);

    common::unload();
    let _ = std::fs::remove_file(&path);
}
