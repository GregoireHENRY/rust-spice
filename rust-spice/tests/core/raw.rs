use itertools::multizip;
use spice::{furnsh, pxform, pxfrm2, spkpos, str2et, timout, unload, TIME_FORMAT};

#[test]
#[serial]
fn test_str2et() {
    furnsh("rsc/krn/hera_study_PO_EMA_2024.tm");

    let et = str2et("2027-MAR-23 16:00:00");

    assert_relative_eq!(et, 859089667.1856234, epsilon = f64::EPSILON);

    unload("rsc/krn/hera_study_PO_EMA_2024.tm");
}

#[test]
#[serial]
fn test_timout() {
    furnsh("rsc/krn/hera_study_PO_EMA_2024.tm");

    let et = str2et("2027-MAR-23 16:00:00");

    let date = timout(et, TIME_FORMAT);

    assert_eq!(date, "2027-MAR-23 16:00:00");

    unload("rsc/krn/hera_study_PO_EMA_2024.tm");
}

#[test]
#[serial]
fn test_spkpos() {
    furnsh("rsc/krn/hera_study_PO_EMA_2024.tm");

    let et = str2et("2027-MAR-23 16:00:00");
    let (position, light_time) = spkpos("DIMORPHOS", et, "J2000", "NONE", "HERA");

    let expected_position = [18.62640405424448, 21.054373008357004, -7.136291402940499];
    let expected_light_time = 0.00009674257074746383;

    for (component, expected_component) in multizip((position.iter(), expected_position.iter())) {
        assert_relative_eq!(component, expected_component, epsilon = f64::EPSILON);
    }

    assert_relative_eq!(light_time, expected_light_time, epsilon = f64::EPSILON);

    unload("rsc/krn/hera_study_PO_EMA_2024.tm");
}

#[test]
#[serial]
fn test_pxform() {
    furnsh("rsc/krn/hera_study_PO_EMA_2024.tm");

    let et = str2et("2027-MAR-23 16:00:00");
    let matrix = pxform("J2000", "ECLIPJ2000", et);

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

    unload("rsc/krn/hera_study_PO_EMA_2024.tm");
}

#[test]
#[serial]
fn test_pxfrm2() {
    furnsh("rsc/krn/hera_study_PO_EMA_2024.tm");

    let etfrom = str2et("2027-MAR-23 16:00:00");
    let etto = etfrom + 30.0 * tool::MINUTE;
    let matrix = pxfrm2("J2000", "J2000", etfrom, etto);

    let expected_matrix = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

    for (row, expected_row) in multizip((matrix.iter(), expected_matrix.iter())) {
        for (component, expected_component) in multizip((row.iter(), expected_row.iter())) {
            assert_relative_eq!(component, expected_component, epsilon = f64::EPSILON);
        }
    }

    unload("rsc/krn/hera_study_PO_EMA_2024.tm");
}
