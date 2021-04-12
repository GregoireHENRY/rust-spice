use itertools::multizip;
use serde_json::{from_str, json, to_string_pretty, Value};
use spice;
use std::collections::BTreeMap;
use tool::Vectors;

fn main() -> Result<(), spice::KernelError> {
    let mut kernel = spice::Kernel::new("rsc/data/hera_PO_EMA_2024.tm")?;
    let frame = "J2000";
    let barycenter = "DIDYMOS_BARYCENTER";
    let primary = "DIDYMOS";
    let secondary = "DIMORPHOS";
    let primary_frame = "DIDYMOS_FIXED";
    let secondary_frame = "DIMORPHOS_FIXED";
    let date = "2027-MAR-23 16:00:00";
    let abcorr = "NONE";
    let time_step = 1.0 * tool::DAY;
    let duration = 192.0 * tool::DAY;

    let et_start = spice::str2et(date);
    let ephemeris_times = tool::linspace(et_start, et_start + duration, time_step);
    let size = ephemeris_times.len();

    let mut positions_primary = Vectors::<f64>::zeros(size);
    let mut positions_secondary = Vectors::<f64>::zeros(size);
    let primary_frame_matrix = spice::pxform(frame, primary_frame, et_start + duration);
    let secondary_frame_matrix = spice::pxform(frame, secondary_frame, et_start + duration);

    for (et, mut primary_pos, mut secondary_pos) in multizip((
        ephemeris_times.iter().cloned(),
        positions_primary.column_iter_mut(),
        positions_secondary.column_iter_mut(),
    )) {
        primary_pos.copy_from(&spice::spkpos(primary, et, frame, abcorr, barycenter).0);
        secondary_pos.copy_from(&spice::spkpos(secondary, et, frame, abcorr, barycenter).0);
    }

    // Export.
    let path = "rsc/data/tmp.json";
    let mut context = BTreeMap::new();
    context.insert("frame".to_string(), json!(frame));
    context.insert("barycenter".to_string(), json!(barycenter));
    context.insert("primary".to_string(), json!(primary));
    context.insert("secondary".to_string(), json!(secondary));
    context.insert("primary_frame".to_string(), json!(primary_frame));
    context.insert("secondary_frame".to_string(), json!(secondary_frame));
    context.insert("abcorr".to_string(), json!(abcorr));
    context.insert("date".to_string(), json!(date));
    context.insert("duration".to_string(), json!(duration));
    context.insert("time_step".to_string(), json!(time_step));
    let j_pos_p: Value = from_str(&to_string_pretty(&positions_primary).unwrap()).unwrap();
    let j_pos_s: Value = from_str(&to_string_pretty(&positions_secondary).unwrap()).unwrap();
    let j_rot_p: Value = from_str(&to_string_pretty(&primary_frame_matrix).unwrap()).unwrap();
    let j_rot_s: Value = from_str(&to_string_pretty(&secondary_frame_matrix).unwrap()).unwrap();
    let json_struct = json!({"context": context, "positions_primary": j_pos_p, "positions_secondary": j_pos_s, "primary_rotation": j_rot_p, "secondary_rotation": j_rot_s});
    let json_string = to_string_pretty(&json_struct).unwrap();
    std::fs::write(path, json_string).unwrap();

    kernel.unload()?;
    Ok(())
}
