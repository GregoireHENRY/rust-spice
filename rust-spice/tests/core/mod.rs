use itertools::multizip;

#[test]
#[serial]
fn das() {
    spice::furnsh("rsc/krn/hera_study_PO_EMA_2024.tm");
    let handle = spice::dasopr("rsc/krn/g_08438mm_lgt_obj_didb_0000n00000_v002.bds");

    let (dladsc, found) = spice::dlabfs(handle);

    assert!(found);

    let dskdsc = spice::dskgd(handle, dladsc);
    let rmax = dskdsc.co3max;

    assert!(rmax > 0f64);

    spice::dascls(handle);
    spice::unload("rsc/krn/hera_study_PO_EMA_2024.tm");
}

#[test]
#[serial]
fn pxform() {
    spice::furnsh("rsc/krn/hera_study_PO_EMA_2024.tm");

    let et = spice::str2et("2027-MAR-23 16:00:00");
    let matrix = spice::pxform("J2000", "ECLIPJ2000", et);

    let expected_matrix = [
        [1.0, 0.0, 0.0],
        [0.0, 0.9174820620691818, 0.3977771559319137],
        [0.0, -0.3977771559319137, 0.9174820620691818],
    ];

    for (row, expected_row) in multizip((matrix.iter(), expected_matrix.iter())) {
        for (component, expected_component) in multizip((row.iter(), expected_row.iter())) {
            assert_relative_eq!(component, expected_component, epsilon = f64::EPSILON);
        }
    }

    spice::unload("rsc/krn/hera_study_PO_EMA_2024.tm");
}

#[test]
#[serial]
fn pxfrm2() {
    spice::furnsh("rsc/krn/hera_study_PO_EMA_2024.tm");

    let etfrom = spice::str2et("2027-MAR-23 16:00:00");
    let etto = etfrom + 30.0 * tool::MINUTE;
    let matrix = spice::pxfrm2("J2000", "J2000", etfrom, etto);

    let expected_matrix = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

    for (row, expected_row) in multizip((matrix.iter(), expected_matrix.iter())) {
        for (component, expected_component) in multizip((row.iter(), expected_row.iter())) {
            assert_relative_eq!(component, expected_component, epsilon = f64::EPSILON);
        }
    }

    spice::unload("rsc/krn/hera_study_PO_EMA_2024.tm");
}

#[test]
#[serial]
fn spkpos() {
    spice::furnsh("rsc/krn/hera_study_PO_EMA_2024.tm");

    let et = spice::str2et("2027-MAR-23 16:00:00");
    let (position, light_time) = spice::spkpos("DIMORPHOS", et, "J2000", "NONE", "HERA");

    let expected_position = [18.62640405424448, 21.054373008357004, -7.136291402940499];
    let expected_light_time = 0.00009674257074746383;

    for (component, expected_component) in multizip((position.iter(), expected_position.iter())) {
        assert_relative_eq!(component, expected_component, epsilon = f64::EPSILON);
    }

    assert_relative_eq!(light_time, expected_light_time, epsilon = f64::EPSILON);

    spice::unload("rsc/krn/hera_study_PO_EMA_2024.tm");
}

#[test]
#[serial]
fn str2et() {
    spice::furnsh("rsc/krn/hera_study_PO_EMA_2024.tm");

    let et = spice::str2et("2027-MAR-23 16:00:00");

    assert_relative_eq!(et, 859089667.1856234, epsilon = f64::EPSILON);

    spice::unload("rsc/krn/hera_study_PO_EMA_2024.tm");
}

#[test]
#[serial]
fn timout() {
    spice::furnsh("rsc/krn/hera_study_PO_EMA_2024.tm");

    let et = spice::str2et("2027-MAR-23 16:00:00");

    let date = spice::timout(et, spice::TIME_FORMAT);

    assert_eq!(date, "2027-MAR-23 16:00:00");

    spice::unload("rsc/krn/hera_study_PO_EMA_2024.tm");
}

#[test]
#[serial]
fn vsep() {
    let ang_1 = spice::vsep([1., 0., 0.], [1., 0., 0.]);
    let ang_2 = spice::vsep([1., 0., 0.], [0., 1., 0.]);

    assert_relative_eq!(ang_1, 0.0, epsilon = f64::EPSILON);
    assert_relative_eq!(ang_2, tool::TAU / 4., epsilon = f64::EPSILON);
}
