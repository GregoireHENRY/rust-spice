use itertools::multizip;
use na::Matrix3x1;

#[test]
#[serial]
fn load() {
    let mut kernel = spice::Kernel::new("rsc/krn/hera_study_PO_EMA_2024.tm").unwrap();
    kernel.unload().unwrap();
}

#[test]
#[serial]
fn position() {
    let mut kernel = spice::Kernel::new("rsc/krn/hera_study_PO_EMA_2024.tm").unwrap();
    let expected_position =
        Matrix3x1::new(18.62640405424448, 21.054373008357004, -7.136291402940499);
    let expected_light_time = 0.00009674257074746383;

    let time = spice::str2et("2027-MAR-23 16:00:00");
    let (position, light_time) = spice::spkpos("DIMORPHOS", time, "J2000", "NONE", "HERA");

    for (component, expected_component) in multizip((position.iter(), expected_position.iter())) {
        assert!(relative_eq!(
            component,
            expected_component,
            epsilon = f64::EPSILON
        ));
    }

    assert!(relative_eq!(
        light_time,
        expected_light_time,
        epsilon = f64::EPSILON
    ));

    kernel.unload().unwrap();
}

#[test]
#[serial]
fn already_loaded() -> Result<(), spice::KernelError> {
    let mut kernel = spice::Kernel::new("rsc/krn/hera_study_PO_EMA_2024.tm")?;

    assert!(kernel.load().is_err());

    kernel.unload()?;
    Ok(())
}

#[test]
#[serial]
fn already_unloaded() -> Result<(), spice::KernelError> {
    let mut kernel = spice::Kernel::new("rsc/krn/hera_study_PO_EMA_2024.tm")?;

    kernel.unload()?;

    assert!(kernel.unload().is_err());
    Ok(())
}

#[test]
#[serial]
fn unload_and_load() -> Result<(), spice::KernelError> {
    let mut kernel = spice::Kernel::new("rsc/krn/hera_study_PO_EMA_2024.tm")?;
    kernel.unload()?;
    kernel.load()?;
    kernel.unload()?;
    Ok(())
}
