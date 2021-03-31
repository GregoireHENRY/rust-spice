// TODO: redo these tests with kernels that won't change

use itertools::multizip;
use na::{Matrix1xX, Matrix3x1, Matrix3xX};

#[test]
#[serial]
fn system() {
    let mut system = spice::SystemBuilder::default()
        .kernel("rsc/data/hera_PO_EMA_2024.tm")
        .unwrap()
        .frame("J2000")
        .observer("HERA")
        .target("DIMORPHOS")
        .start_date("2027-MAR-23 16:00:00")
        .duration(129.0 * spice::DAY)
        .aberration_correction("NONE")
        .build()
        .unwrap();
    system.unload().unwrap();
}

#[test]
#[serial]
fn test_fields() {
    let kernel = "rsc/data/hera_PO_EMA_2024.tm";
    let frame = "J2000";
    let observer = "HERA";
    let target = "DIMORPHOS";
    let start_date = "2027-MAR-23 16:00:00";
    let duration = 129.0 * spice::DAY;
    let aberration_correction = "NONE";
    let mut system = spice::SystemBuilder::default()
        .kernel(kernel)
        .unwrap()
        .frame(frame)
        .observer(observer)
        .target(target)
        .start_date(start_date)
        .duration(duration)
        .aberration_correction(aberration_correction)
        .build()
        .unwrap();
    assert_eq!(system.kernel().name(), kernel);
    assert_eq!(system.frame(), frame);
    assert_eq!(system.observer(), observer);
    assert_eq!(system.target(), target);
    assert_eq!(system.start_date(), start_date);
    assert_eq!(system.duration(), duration);
    assert_eq!(system.aberration_correction(), aberration_correction);
    system.unload().unwrap();
}

#[test]
#[serial]
fn time_start() {
    let mut system = spice::SystemBuilder::default()
        .kernel("rsc/data/hera_PO_EMA_2024.tm")
        .unwrap()
        .frame("J2000")
        .observer("HERA")
        .target("DIMORPHOS")
        .start_date("2027-MAR-23 16:00:00")
        .duration(129.0 * spice::DAY)
        .aberration_correction("NONE")
        .build()
        .unwrap();
    let expected_time = 859089667.1856234;

    let time = system.time_start();

    assert_eq!(time, expected_time);
    system.unload().unwrap();
}

#[test]
#[serial]
fn time_end() {
    let mut system = spice::SystemBuilder::default()
        .kernel("rsc/data/hera_PO_EMA_2024.tm")
        .unwrap()
        .frame("J2000")
        .observer("HERA")
        .target("DIMORPHOS")
        .start_date("2027-MAR-23 16:00:00")
        .duration(129.0 * spice::DAY)
        .aberration_correction("NONE")
        .build()
        .unwrap();
    let expected_time = 870235267.1856234;

    let time = system.time_end();

    assert_eq!(time, expected_time);
    system.unload();
}

#[test]
#[serial]
fn position_start() {
    let mut system = spice::SystemBuilder::default()
        .kernel("rsc/data/hera_PO_EMA_2024.tm")
        .unwrap()
        .frame("J2000")
        .observer("HERA")
        .target("DIMORPHOS")
        .start_date("2027-MAR-23 16:00:00")
        .duration(129.0 * spice::DAY)
        .aberration_correction("NONE")
        .build()
        .unwrap();
    let expected_position =
        Matrix3x1::new(18.62639796759623, 21.05444863563425, -7.136416860555217);

    let position = system.position_start();

    for (component, expected_component) in multizip((position.iter(), expected_position.iter())) {
        // println!("{} {}", component, expected_component);
        assert!(relative_eq!(
            component,
            expected_component,
            epsilon = f64::EPSILON
        ));
    }

    system.unload();
}

#[test]
#[serial]
fn position_end() {
    let mut system = spice::SystemBuilder::default()
        .kernel("rsc/data/hera_PO_EMA_2024.tm")
        .unwrap()
        .frame("J2000")
        .observer("HERA")
        .target("DIMORPHOS")
        .start_date("2027-MAR-23 16:00:00")
        .duration(129.0 * spice::DAY)
        .aberration_correction("NONE")
        .build()
        .unwrap();
    let expected_position =
        Matrix3x1::new(-6.978302254901042, -13.20486626395836, -3.9423172996249947);

    let position = system.position_end();

    for (component, expected_component) in multizip((position.iter(), expected_position.iter())) {
        assert!(relative_eq!(
            component,
            expected_component,
            epsilon = f64::EPSILON
        ));
    }

    system.unload();
}

#[test]
#[serial]
fn number_points() {
    let mut system = spice::SystemBuilder::default()
        .kernel("rsc/data/hera_PO_EMA_2024.tm")
        .unwrap()
        .frame("J2000")
        .observer("HERA")
        .target("DIMORPHOS")
        .start_date("2027-MAR-23 16:00:00")
        .duration(129.0 * spice::DAY)
        .aberration_correction("NONE")
        .build()
        .unwrap();
    let time_step = 1.0 * spice::DAY;
    let expected_number = 130;

    let number = system.number_points(time_step);

    assert_eq!(number, expected_number);

    system.unload();
}

#[test]
#[serial]
fn times() {
    let mut system = spice::SystemBuilder::default()
        .kernel("rsc/data/hera_PO_EMA_2024.tm")
        .unwrap()
        .frame("J2000")
        .observer("HERA")
        .target("DIMORPHOS")
        .start_date("2027-MAR-23 16:00:00")
        .duration(3.0 * spice::DAY)
        .aberration_correction("NONE")
        .build()
        .unwrap();
    let time_step = 1.0 * spice::DAY;
    let expected_times =
        Matrix1xX::from_column_slice(&[859089667.1856234, 859176067.1856234, 859262467.1856234]);

    let times = system.times(time_step);

    for (time, expected_time) in multizip((times.iter(), expected_times.iter())) {
        assert_eq!(time, expected_time);
    }

    system.unload();
}

#[test]
#[serial]
fn times_formatted() {
    let mut system = spice::SystemBuilder::default()
        .kernel("rsc/data/hera_PO_EMA_2024.tm")
        .unwrap()
        .frame("J2000")
        .observer("HERA")
        .target("DIMORPHOS")
        .start_date("2027-MAR-23 16:00:00")
        .duration(3.0 * spice::DAY)
        .aberration_correction("NONE")
        .build()
        .unwrap();
    let time_step = 1.0 * spice::DAY;
    let expected_times = Matrix1xX::from_column_slice(&[
        "2027-MAR-23 16:00:00",
        "2027-MAR-24 16:00:00",
        "2027-MAR-25 16:00:00",
        "2027-MAR-26 16:00:00",
    ]);

    let times = system.times_formatted(time_step);

    for (time, expected_time) in multizip((times.iter(), expected_times.iter())) {
        assert_eq!(time, expected_time);
    }

    system.unload();
}

#[test]
#[serial]
fn positions() {
    let mut system = spice::SystemBuilder::default()
        .kernel("rsc/data/hera_PO_EMA_2024.tm")
        .unwrap()
        .frame("J2000")
        .observer("HERA")
        .target("DIMORPHOS")
        .start_date("2027-MAR-23 16:00:00")
        .duration(3.0 * spice::DAY)
        .aberration_correction("NONE")
        .build()
        .unwrap();
    let time_step = 1.0 * spice::DAY;
    let expected_positions = Matrix3xX::from_column_slice(&[
        18.62639796759623,
        21.05444863563425,
        -7.136416860555217,
        18.685576648253853,
        17.77035176622136,
        2.315428951220107,
        18.63486050866997,
        13.806780236054001,
        11.523204797636854,
    ]);

    let positions = system.positions(time_step);

    for (component, expected_component) in multizip((positions.iter(), expected_positions.iter())) {
        assert!(relative_eq!(
            component,
            expected_component,
            epsilon = f64::EPSILON
        ));
    }

    system.unload();
}
