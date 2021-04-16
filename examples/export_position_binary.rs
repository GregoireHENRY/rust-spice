use itertools::multizip;
use serde_json::json;
use spice;
use tool::Vectors;

fn main() -> Result<(), spice::KernelError> {
    let mut kernel = spice::Kernel::new("rsc/krn/hera_study_PO_EMA_2024.tm")?;
    let frame = "J2000";
    let barycenter = "DIDYMOS_BARYCENTER";
    let primary = "DIDYMOS";
    let secondary = "DIMORPHOS";
    let primary_frame_name = "DIDYMOS_FIXED";
    let secondary_frame_name = "DIMORPHOS_FIXED";
    let date = "2027-MAR-23 16:00:00";
    let abcorr = "NONE";
    let time_step = 10.0 * tool::MINUTE;
    let duration = 20.0 * tool::DAY;

    let et_start = spice::str2et(date);
    let ephemeris_times = tool::linspace(et_start, et_start + duration, time_step);
    let size = ephemeris_times.len();

    let mut primary_positions = Vectors::zeros(size);
    let mut secondary_positions = Vectors::zeros(size);
    let primary_frame = spice::pxform(frame, primary_frame_name, et_start + duration);
    let secondary_frame = spice::pxform(frame, secondary_frame_name, et_start + duration);
    let primary_frame_previous =
        spice::pxform(frame, primary_frame_name, et_start + duration - time_step);
    let secondary_frame_previous =
        spice::pxform(frame, secondary_frame_name, et_start + duration - time_step);

    for (et, mut primary_pos, mut secondary_pos) in multizip((
        ephemeris_times.iter().cloned(),
        primary_positions.column_iter_mut(),
        secondary_positions.column_iter_mut(),
    )) {
        primary_pos.copy_from(&spice::spkpos(primary, et, frame, abcorr, barycenter).0);
        secondary_pos.copy_from(&spice::spkpos(secondary, et, frame, abcorr, barycenter).0);
    }

    // Export.
    let json = json!({
        "context": {
            "frame": frame,
            "barycenter": barycenter,
            "primary": primary,
            "secondary": secondary,
            "primary_frame_name": primary_frame_name,
            "secondary_frame_name": secondary_frame_name,
            "abcorr": abcorr,
            "date": date,
            "duration": duration,
            "time_step": time_step,
        },
        "primary_positions": primary_positions,
        "secondary_positions": secondary_positions,
        "primary_frame": primary_frame,
        "secondary_frame": secondary_frame,
        "primary_frame_previous": primary_frame_previous,
        "secondary_frame_previous": secondary_frame_previous,
    });
    tool::writejs!("rsc/data/tmp/position_binary.json", json);

    kernel.unload()?;
    Ok(())
}
