use itertools::multizip;
use nalgebra::Rotation3;
use serde_json::json;
use tool::{Vector, Vectors};

fn main() -> Result<(), spice::KernelError> {
    let mut kernel = spice::Kernel::new("rsc/krn/hera_study_PO_EMA_2024.tm")?;
    let primary = "DIDYMOS";
    let secondary = "DIMORPHOS";
    let primary_frame_name = "DIDYMOS_FIXED";
    let secondary_frame_name = "DIMORPHOS_FIXED";
    let date = "2027-MAR-23 16:00:00";
    let abcorr = "NONE";
    let time_step = 1.0 * tool::MINUTE;
    let duration = 2.5 * tool::HOUR;

    let et_start = spice::str2et(date);
    let ephemeris_times = tool::linspace(et_start, et_start + duration, time_step);
    let size = ephemeris_times.len();

    let mut secondary_positions = Vectors::<f64>::zeros(size);
    let mut secondary_frames = vec![Rotation3::new(Vector::zeros()); size];

    for (et, mut secondary_pos, secondary_frame) in multizip((
        ephemeris_times.iter().cloned(),
        secondary_positions.column_iter_mut(),
        secondary_frames.iter_mut(),
    )) {
        secondary_pos
            .copy_from(&spice::spkpos(secondary, et, primary_frame_name, abcorr, primary).0);
        *secondary_frame = spice::pxform(primary_frame_name, secondary_frame_name, et);
    }

    // Export.
    let json = json!({
        "context": {
            "primary": primary,
            "secondary": secondary,
            "primary_frame_name": primary_frame_name,
            "secondary_frame_name": secondary_frame_name,
            "abcorr": abcorr,
            "date": date,
            "duration": duration,
            "time_step": time_step,
        },
        "secondary_positions": secondary_positions,
        "secondary_frames": secondary_frames
    });
    tool::writejs!("rsc/data/tmp/binary.json", json);

    kernel.unload()?;
    Ok(())
}
