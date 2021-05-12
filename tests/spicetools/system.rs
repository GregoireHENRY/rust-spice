// TODO: redo these tests with kernels that won't change

use itertools::multizip;
use na::{Matrix1xX, Matrix3x1, Matrix3xX};
use spice::{SystemBuilder, SystemError};

#[test]
#[serial]
fn system() {
    let mut system = SystemBuilder::default()
        .kernel("rsc/krn/hera_study_PO_EMA_2024.tm")
        .unwrap()
        .frame("J2000")
        .observer("HERA")
        .target("DIMORPHOS")
        .start_date("2027-MAR-23 16:00:00")
        .duration(100.0 * tool::DAY)
        .aberration_correction("NONE")
        .build()
        .unwrap();
    system.unload().unwrap();
}

#[test]
#[serial]
fn test_fields() {
    let kernel = "rsc/krn/hera_study_PO_EMA_2024.tm";
    let frame = "J2000";
    let observer = "HERA";
    let target = "DIMORPHOS";
    let start_date = "2027-MAR-23 16:00:00";
    let duration = 100.0 * tool::DAY;
    let aberration_correction = "NONE";
    let mut system = SystemBuilder::default()
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
    assert!(relative_eq!(
        system.duration(),
        duration,
        epsilon = f64::EPSILON
    ));
    assert_eq!(system.aberration_correction(), aberration_correction);
    system.unload().unwrap();
}

#[test]
#[serial]
fn time_start() {
    let mut system = SystemBuilder::default()
        .kernel("rsc/krn/hera_study_PO_EMA_2024.tm")
        .unwrap()
        .frame("J2000")
        .observer("HERA")
        .target("DIMORPHOS")
        .start_date("2027-MAR-23 16:00:00")
        .duration(100.0 * tool::DAY)
        .aberration_correction("NONE")
        .build()
        .unwrap();
    let expected_time = 859089667.1856234;

    let time = system.time_start();

    assert!(relative_eq!(time, expected_time, epsilon = f64::EPSILON));
    system.unload().unwrap();
}

#[test]
#[serial]
fn time_end() {
    let mut system = SystemBuilder::default()
        .kernel("rsc/krn/hera_study_PO_EMA_2024.tm")
        .unwrap()
        .frame("J2000")
        .observer("HERA")
        .target("DIMORPHOS")
        .start_date("2027-MAR-23 16:00:00")
        .duration(100.0 * tool::DAY)
        .aberration_correction("NONE")
        .build()
        .unwrap();
    let expected_time = 867729667.1856234;

    let time = system.time_end();

    assert!(relative_eq!(time, expected_time, epsilon = f64::EPSILON));
    system.unload().unwrap();
}

#[test]
#[serial]
fn position_start() {
    let mut system = SystemBuilder::default()
        .kernel("rsc/krn/hera_study_PO_EMA_2024.tm")
        .unwrap()
        .frame("J2000")
        .observer("HERA")
        .target("DIMORPHOS")
        .start_date("2027-MAR-23 16:00:00")
        .duration(100.0 * tool::DAY)
        .aberration_correction("NONE")
        .build()
        .unwrap();
    let expected_position =
        Matrix3x1::new(18.62640405424448, 21.054373008357004, -7.136291402940499);

    let position = system.position_start();

    for (component, expected_component) in multizip((position.iter(), expected_position.iter())) {
        // println!("{} {}", component, expected_component);
        assert!(relative_eq!(
            component,
            expected_component,
            epsilon = f64::EPSILON
        ));
    }

    system.unload().unwrap();
}

#[test]
#[serial]
fn position_end() {
    let mut system = SystemBuilder::default()
        .kernel("rsc/krn/hera_study_PO_EMA_2024.tm")
        .unwrap()
        .frame("J2000")
        .observer("HERA")
        .target("DIMORPHOS")
        .start_date("2027-MAR-23 16:00:00")
        .duration(100.0 * tool::DAY)
        .aberration_correction("NONE")
        .build()
        .unwrap();
    let expected_position =
        Matrix3x1::new(-15.755418936009995, -6.983783596736234, -0.9102520894410653);

    let position = system.position_end();

    for (component, expected_component) in multizip((position.iter(), expected_position.iter())) {
        assert!(relative_eq!(
            component,
            expected_component,
            epsilon = f64::EPSILON
        ));
    }

    system.unload().unwrap();
}

#[test]
#[serial]
fn number_points() {
    let mut system = SystemBuilder::default()
        .kernel("rsc/krn/hera_study_PO_EMA_2024.tm")
        .unwrap()
        .frame("J2000")
        .observer("HERA")
        .target("DIMORPHOS")
        .start_date("2027-MAR-23 16:00:00")
        .duration(100.0 * tool::DAY)
        .aberration_correction("NONE")
        .build()
        .unwrap();
    let time_step = 1.0 * tool::DAY;
    let expected_number = 101;

    let number = system.number_points(time_step);

    assert_eq!(number, expected_number);

    system.unload().unwrap();
}

#[test]
#[serial]
fn times() {
    let mut system = SystemBuilder::default()
        .kernel("rsc/krn/hera_study_PO_EMA_2024.tm")
        .unwrap()
        .frame("J2000")
        .observer("HERA")
        .target("DIMORPHOS")
        .start_date("2027-MAR-23 16:00:00")
        .duration(3.0 * tool::DAY)
        .aberration_correction("NONE")
        .build()
        .unwrap();
    let time_step = 1.0 * tool::DAY;
    let expected_times =
        Matrix1xX::from_column_slice(&[859089667.1856234, 859176067.1856234, 859262467.1856234]);

    let times = system.times(time_step);

    for (time, expected_time) in multizip((times.iter(), expected_times.iter())) {
        assert!(relative_eq!(time, expected_time, epsilon = f64::EPSILON));
    }

    system.unload().unwrap();
}

#[test]
#[serial]
fn times_formatted() {
    let mut system = SystemBuilder::default()
        .kernel("rsc/krn/hera_study_PO_EMA_2024.tm")
        .unwrap()
        .frame("J2000")
        .observer("HERA")
        .target("DIMORPHOS")
        .start_date("2027-MAR-23 16:00:00")
        .duration(3.0 * tool::DAY)
        .aberration_correction("NONE")
        .build()
        .unwrap();
    let time_step = 1.0 * tool::DAY;
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

    system.unload().unwrap();
}

#[test]
#[serial]
fn test_positions() {
    let mut system = SystemBuilder::default()
        .kernel("rsc/krn/hera_study_PO_EMA_2024.tm")
        .unwrap()
        .frame("J2000")
        .observer("HERA")
        .target("DIMORPHOS")
        .start_date("2027-MAR-23 16:00:00")
        .duration(3.0 * tool::DAY)
        .aberration_correction("NONE")
        .build()
        .unwrap();
    let time_step = 1.0 * tool::DAY;
    let expected_positions = Matrix3xX::from_column_slice(&[
        18.62640405424448,
        21.054373008357004,
        -7.136291402940499,
        18.686088286014225,
        17.770768246121182,
        2.3155663881208772,
        18.63536857304623,
        13.807160769358065,
        11.523427696615501,
        18.47919249140907,
        9.220087797695399,
        20.37198559472944,
    ]);

    let positions = system.positions(time_step);

    for (component, expected_component) in multizip((positions.iter(), expected_positions.iter())) {
        assert!(relative_eq!(
            component,
            expected_component,
            epsilon = f64::EPSILON
        ));
    }

    system.unload().unwrap();
}

#[test]
#[serial]
fn builder_needs_kernel() -> Result<(), SystemError> {
    let e = SystemBuilder::default()
        .frame("J2000")
        .observer("HERA")
        .target("DIMORPHOS")
        .start_date("2027-MAR-23 16:00:00")
        .duration(129.0 * tool::DAY)
        .aberration_correction("NONE")
        .build();

    assert!(e.is_err());
    Ok(())
}

#[test]
#[serial]
fn builder_needs_frame() -> Result<(), SystemError> {
    let e = SystemBuilder::default()
        .kernel("rsc/krn/hera_study_PO_EMA_2024.tm")?
        .observer("HERA")
        .target("DIMORPHOS")
        .start_date("2027-MAR-23 16:00:00")
        .duration(129.0 * tool::DAY)
        .aberration_correction("NONE")
        .build();

    assert!(e.is_err());
    Ok(())
}

#[test]
#[serial]
fn builder_needs_observer() -> Result<(), SystemError> {
    let e = SystemBuilder::default()
        .kernel("rsc/krn/hera_study_PO_EMA_2024.tm")?
        .frame("J2000")
        .target("DIMORPHOS")
        .start_date("2027-MAR-23 16:00:00")
        .duration(129.0 * tool::DAY)
        .aberration_correction("NONE")
        .build();

    assert!(e.is_err());
    Ok(())
}

#[test]
#[serial]
fn builder_needs_target() -> Result<(), SystemError> {
    let e = SystemBuilder::default()
        .kernel("rsc/krn/hera_study_PO_EMA_2024.tm")?
        .frame("J2000")
        .observer("HERA")
        .start_date("2027-MAR-23 16:00:00")
        .duration(129.0 * tool::DAY)
        .aberration_correction("NONE")
        .build();

    assert!(e.is_err());
    Ok(())
}

#[test]
#[serial]
fn builder_needs_start_date() -> Result<(), SystemError> {
    let e = SystemBuilder::default()
        .kernel("rsc/krn/hera_study_PO_EMA_2024.tm")?
        .frame("J2000")
        .observer("HERA")
        .target("DIMORPHOS")
        .duration(129.0 * tool::DAY)
        .aberration_correction("NONE")
        .build();

    assert!(e.is_err());
    Ok(())
}

#[test]
#[serial]
fn builder_needs_duration() -> Result<(), SystemError> {
    let e = SystemBuilder::default()
        .kernel("rsc/krn/hera_study_PO_EMA_2024.tm")?
        .frame("J2000")
        .observer("HERA")
        .target("DIMORPHOS")
        .start_date("2027-MAR-23 16:00:00")
        .aberration_correction("NONE")
        .build();

    assert!(e.is_err());
    Ok(())
}

#[test]
#[serial]
fn builder_needs_aberration() -> Result<(), SystemError> {
    let e = SystemBuilder::default()
        .kernel("rsc/krn/hera_study_PO_EMA_2024.tm")?
        .frame("J2000")
        .observer("HERA")
        .target("DIMORPHOS")
        .start_date("2027-MAR-23 16:00:00")
        .duration(129.0 * tool::DAY)
        .build();

    assert!(e.is_err());
    Ok(())
}
