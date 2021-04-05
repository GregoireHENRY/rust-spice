use itertools::multizip;
use na::Matrix3x1;

#[test]
#[serial]
fn load() {
    let mut kernel = spice::Kernel::new("rsc/data/hera_PO_EMA_2024.tm").unwrap();
    kernel.unload().unwrap();
}

#[test]
#[serial]
fn position() {
    let mut kernel = spice::Kernel::new("rsc/data/hera_PO_EMA_2024.tm").unwrap();
    let expected_position =
        Matrix3x1::new(18.62639796759623, 21.05444863563425, -7.136416860555217);
    let expected_light_time = 0.00009674284381011395;

    let time = spice::ephemeris_from_date("2027-MAR-23 16:00:00");
    let (position, light_time) = spice::position("DIMORPHOS", time, "J2000", "NONE", "HERA");

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
    let mut kernel = spice::Kernel::new("rsc/data/hera_PO_EMA_2024.tm")?;

    assert_eq!(
        kernel.load(),
        Err(spice::KernelError {
            kind: spice::KernelErrorKind::AlreadyLoaded
        })
    );

    kernel.unload()?;
    Ok(())
}

#[test]
#[serial]
fn display() -> Result<(), spice::KernelError> {
    let mut kernel = spice::Kernel::new("rsc/data/hera_PO_EMA_2024.tm")?;

    match kernel.load() {
        Ok(_) => (),
        Err(e) => assert_eq!(e.to_string(), "the kernel is already loaded"),
    };

    kernel.unload()?;
    Ok(())
}

#[test]
#[serial]
fn debug() -> Result<(), spice::KernelError> {
    let mut kernel = spice::Kernel::new("rsc/data/hera_PO_EMA_2024.tm")?;

    match kernel.load() {
        Ok(_) => (),
        Err(e) => assert_eq!(format!("{:?}", e), r#""the kernel is already loaded""#),
    };

    kernel.unload()?;
    Ok(())
}
