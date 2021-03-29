// TODO: redo these tests with kernels that won't change

use itertools::multizip;
use na::{Matrix1xX, Matrix3x1};

#[test]
#[serial]
fn create_system() {
    let mut system = spice::System::new(
        "rsc/data/hera_PO_EMA_2024.tm", // kernel
        "J2000",                        // frame
        "HERA",                         // observer
        "DIMORPHOS",                    // target
        "2027-MAR-23 16:00:00",         // start date
        129.0 * spice::get_DAY(),       // duration
        "NONE",                         // aberration correction
    );
    system.unload();
}

#[test]
#[serial]
fn test_fields() {
    let kernel = "rsc/data/hera_PO_EMA_2024.tm";
    let frame = "J2000";
    let observer = "HERA";
    let target = "DIMORPHOS";
    let start_date = "2027-MAR-23 16:00:00";
    let duration = 129.0 * spice::get_DAY();
    let abcorr = "NONE";
    let mut system = spice::System::new(
        kernel, frame, observer, target, start_date, duration, abcorr,
    );

    assert_eq!(system.kernel(), kernel);
    assert_eq!(system.frame(), frame);
    assert_eq!(system.observer(), observer);
    assert_eq!(system.target(), target);
    assert_eq!(system.start_date(), start_date);
    assert_eq!(system.duration(), duration);
    assert_eq!(system.abcorr(), abcorr);

    system.unload();
}

#[test]
#[serial]
fn time_start() {
    let mut system = spice::System::new(
        "rsc/data/hera_PO_EMA_2024.tm", // kernel
        "J2000",                        // frame
        "HERA",                         // observer
        "DIMORPHOS",                    // target
        "2027-MAR-23 16:00:00",         // start date
        129.0 * spice::get_DAY(),       // duration
        "NONE",                         // aberration correction
    );
    let expected_time = 859089667.1856234;

    let time = system.time_start();

    assert_eq!(time, expected_time);
    system.unload();
}

#[test]
#[serial]
fn time_end() {
    let mut system = spice::System::new(
        "rsc/data/hera_PO_EMA_2024.tm", // kernel
        "J2000",                        // frame
        "HERA",                         // observer
        "DIMORPHOS",                    // target
        "2027-MAR-23 16:00:00",         // start date
        129.0 * spice::get_DAY(),       // duration
        "NONE",                         // aberration correction
    );
    let expected_time = 870235267.1856234;

    let time = system.time_end();

    assert_eq!(time, expected_time);
    system.unload();
}

#[test]
#[serial]
fn position_start() {
    let mut system = spice::System::new(
        "rsc/data/hera_PO_EMA_2024.tm", // kernel
        "J2000",                        // frame
        "HERA",                         // observer
        "DIMORPHOS",                    // target
        "2027-MAR-23 16:00:00",         // start date
        129.0 * spice::get_DAY(),       // duration
        "NONE",                         // aberration correction
    );
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
    let mut system = spice::System::new(
        "rsc/data/hera_PO_EMA_2024.tm", // kernel
        "J2000",                        // frame
        "HERA",                         // observer
        "DIMORPHOS",                    // target
        "2027-MAR-23 16:00:00",         // start date
        129.0 * spice::get_DAY(),       // duration
        "NONE",                         // aberration correction
    );
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
    let mut system = spice::System::new(
        "rsc/data/hera_PO_EMA_2024.tm", // kernel
        "J2000",                        // frame
        "HERA",                         // observer
        "DIMORPHOS",                    // target
        "2027-MAR-23 16:00:00",         // start date
        129.0 * spice::get_DAY(),       // duration
        "NONE",                         // aberration correction
    );
    let time_step = 1.0 * spice::get_DAY();
    let expected_number = 129;

    let number = system.number_points(time_step);

    assert_eq!(number, expected_number);

    system.unload();
}

#[test]
#[serial]
fn times_formatted() {
    let mut system = spice::System::new(
        "rsc/data/hera_PO_EMA_2024.tm", // kernel
        "J2000",                        // frame
        "HERA",                         // observer
        "DIMORPHOS",                    // target
        "2027-MAR-23 16:00:00",         // start date
        3.0 * spice::get_DAY(),         // duration
        "NONE",                         // aberration correction
    );
    let time_step = 1.0 * spice::get_DAY();
    let expected_times = Matrix1xX::from_column_slice(&[
        "2027-MAR-23 16:00:00",
        "2027-MAR-24 16:00:00",
        "2027-MAR-25 16:00:00",
    ]);

    let times = system.times_formatted(time_step);

    for (time, expected_time) in multizip((times.iter(), expected_times.iter())) {
        assert_eq!(time, expected_time);
    }

    system.unload();
}
