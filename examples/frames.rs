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

    let rotate = spice::pxform(inertial_frame, system_frame, et);

    let up = tool::Vector::new(0., 0., 1.);
    let up_tilted = rotate * up;

    let obliquity = up.dot(&up_tilted).acos() * tool::RAD2DEG;

    let rotate_step = spice::pxfrm2(body_frame, body_frame, et, et + time_step);

    println!("position (km): {:.2}", position);
    println!("rotation of system frame: {:.2}", rotate.matrix());
    println!("obliquity: {:.2} deg", obliquity);
    println!(
        "rotation body frame after {:0} seconds: {:.2}",
        time_step,
        rotate_step.matrix()
    );

    kernel.unload()?;
    Ok(())
}
