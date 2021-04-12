use spice;

fn main() -> Result<(), spice::KernelError> {
    let mut kernel = spice::Kernel::new("rsc/data/hera_PO_EMA_2024.tm")?;
    let inertial_frame = "J2000";
    let system_frame = "DIDYMOS_FIXED";
    let body_frame = "DIMORPHOS_FIXED";
    let observer = "SUN";
    let target = "DIMORPHOS";
    let date = "2027-MAR-23 16:00:00";
    let abcorr = "NONE";
    let time_step = 3600.0;

    let et = spice::str2et(date);

    let (position, _) = spice::spkpos(target, et, inertial_frame, abcorr, observer);

    let system_rotate = spice::pxform(inertial_frame, system_frame, et);
    let body_rotate = spice::pxform(inertial_frame, body_frame, et);
    let rotate_step = spice::pxfrm2(body_frame, body_frame, et, et + time_step);

    println!(
        "{} position in {} wrt {} (km): {:.2}",
        target, inertial_frame, observer, position
    );
    println!(
        "rotation {} -> {}: {:.2}",
        inertial_frame,
        system_frame,
        system_rotate.matrix()
    );
    println!(
        "rotation {} -> {}: {:.2}",
        inertial_frame,
        body_frame,
        body_rotate.matrix()
    );
    println!(
        "rotation of {} itself after {:0} seconds: {:.2}",
        body_frame,
        time_step,
        rotate_step.matrix()
    );

    kernel.unload()?;
    Ok(())
}
