use itertools::multizip;
use nalgebra::Rotation3;
use serde_json::json;
use tool::{Vector, Vectors};

fn main() -> Result<(), spice::KernelError> {
    tool::log_init!();

    // Define names.
    let mut kernel = spice::Kernel::new("rsc/krn/hera_study_PO_EA_2026.tm")?;
    let reference_frame_name = "J2000";
    let primary_name = "DIDYMOS";
    let secondary_name = "DIMORPHOS";
    let primary_frame_name = "DIDYMOS_FIXED";
    let secondary_frame_name = "DIMORPHOS_FIXED";
    let date = "2027-MAR-23 16:00:00";
    let abcorr = "NONE";
    let time_step = 1.0 * tool::MINUTE;
    let duration = 11.92 * tool::HOUR;

    // Create context.
    let et_start = spice::str2et(date);
    let ephemeris_times = tool::linspace(et_start, et_start + duration, time_step);
    let size = ephemeris_times.len();

    // Allocate list of results.
    let mut secondary_positions = Vectors::<f64>::zeros(size);
    let mut primary_frames = vec![Rotation3::new(Vector::zeros()); size];
    let mut secondary_frames = vec![Rotation3::new(Vector::zeros()); size];

    for (et, mut secondary_pos, primary_frame, secondary_frame) in multizip((
        ephemeris_times.iter().cloned(),
        secondary_positions.column_iter_mut(),
        primary_frames.iter_mut(),
        secondary_frames.iter_mut(),
    )) {
        secondary_pos.copy_from(
            &spice::spkpos(
                secondary_name,
                et,
                reference_frame_name,
                abcorr,
                primary_name,
            )
            .0,
        );
        *primary_frame = spice::pxform(reference_frame_name, primary_frame_name, et);
        *secondary_frame = spice::pxform(reference_frame_name, secondary_frame_name, et);
    }

    // Export.
    let json = json!({
        "context": {
            "reference_frame_name": reference_frame_name,
            "primary_name": primary_name,
            "secondary_name": secondary_name,
            "primary_frame_name": primary_frame_name,
            "secondary_frame_name": secondary_frame_name,
            "abcorr": abcorr,
            "date": date,
            "duration": duration,
            "time_step": time_step,
        },
        "secondary_positions": secondary_positions,
        "primary_frames": primary_frames,
        "secondary_frames": secondary_frames,
    });
    tool::writejs!("rsc/data/tmp/binary.json", json);

    kernel.unload()?;
    Ok(())
}
