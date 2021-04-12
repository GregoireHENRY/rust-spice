use nalgebra;
use spice;

fn main() -> Result<(), spice::KernelError> {
    let mut kernel = spice::Kernel::new("rsc/data/hera_PO_EMA_2024.tm")?;
    let date = "2027-MAR-23 16:00:00";
    let from = "J2000";
    let to = "ECLIPJ2000";

    let et = spice::str2et(date);

    let rotation_default = nalgebra::Rotation3::<f64>::identity();
    let rotate = spice::pxform("J2000", "ECLIPJ2000", et);

    let up = nalgebra::Unit::new_normalize(nalgebra::Vector3::new(0., 0., 1.));
    let up_tilted = rotate * up;
    let tilt_angle = up.dot(&up_tilted).acos() * tool::RAD2DEG;
    let angle_rotate = rotation_default.angle_to(&rotate) * tool::RAD2DEG;

    println!("rotation {} -> {}: {:.2}", from, to, rotate.matrix());
    println!("tilt angle: {:.2}", tilt_angle);
    println!("rotate angle: {:.2}", angle_rotate);

    kernel.unload()?;
    Ok(())
}
