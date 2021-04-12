use serde_json::{from_str, json, to_string_pretty, Value};
use spice;
use std::collections::BTreeMap;

fn main() -> Result<(), spice::KernelError> {
    let mut kernel = spice::Kernel::new("rsc/data/hera_PO_EMA_2024.tm")?;
    let frame = "J2000";
    let origin = "SUN";
    let secondary_frame = "DIMORPHOS_FIXED";
    let date = "2027-MAR-23 16:00:00";
    let abcorr = "NONE";
    let duration_1 = 20.0 * tool::DAY;
    let duration_2 = 192.0 * tool::DAY;

    let et_start = spice::str2et(date);
    let date_1 = spice::timout(et_start + duration_1, spice::TIME_FORMAT);
    let date_2 = spice::timout(et_start + duration_2, spice::TIME_FORMAT);

    let secondary_frame_matrix_1 = spice::pxform(frame, secondary_frame, et_start + duration_1);
    let secondary_frame_matrix_2 = spice::pxform(frame, secondary_frame, et_start + duration_2);

    // Export.
    let path = "rsc/data/tmp.json";
    let mut context = BTreeMap::new();
    context.insert("frame".to_string(), json!(frame));
    context.insert("origin".to_string(), json!(origin));
    context.insert("secondary_frame".to_string(), json!(secondary_frame));
    context.insert("abcorr".to_string(), json!(abcorr));
    context.insert("date".to_string(), json!(date));
    context.insert("date_1".to_string(), json!(date_1));
    context.insert("date_2".to_string(), json!(date_2));
    context.insert("duration_1".to_string(), json!(duration_1));
    context.insert("duration_2".to_string(), json!(duration_2));
    let fs1: Value = from_str(&to_string_pretty(&secondary_frame_matrix_1).unwrap()).unwrap();
    let fs2: Value = from_str(&to_string_pretty(&secondary_frame_matrix_2).unwrap()).unwrap();
    let json_struct =
        json!({"context": context, "secondary_frame_1": fs1, "secondary_frame_2": fs2});
    let json_string = to_string_pretty(&json_struct).unwrap();
    std::fs::write(path, json_string).unwrap();

    kernel.unload()?;
    Ok(())
}
