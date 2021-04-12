use itertools::multizip;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use spice;
use std::collections::BTreeMap;
use tool::Vectors;

/// Struct to be converted into JSON.
#[derive(Serialize, Deserialize)]
pub struct JsonStruct {
    /// Some context.
    context: BTreeMap<String, Value>,
    /// Positions on the orbit.
    positions: Vectors<f64>,
}

/// Write JSON string to file.
pub fn write_str<S: AsRef<str>>(path: S, string: String) {
    std::fs::write(path.as_ref(), string).unwrap();
}

fn main() -> Result<(), spice::KernelError> {
    let mut kernel = spice::Kernel::new("rsc/data/hera_PO_EMA_2024.tm")?;
    let frame = "J2000";
    let target = "DIDYMOS_BARYCENTER";
    let observer = "SUN";
    let date = "2027-MAR-23 16:00:00";
    let abcorr = "NONE";
    let time_step = 1.0 * tool::DAY;
    let duration = 192.0 * tool::DAY;

    let et_start = spice::str2et(date);
    let ephemeris_times = tool::linspace(et_start, et_start + duration, time_step);
    let size = ephemeris_times.len();

    println!("{}", spice::timout(et_start + duration, spice::TIME_FORMAT));

    let mut positions = Vectors::<f64>::zeros(size);

    for (et, mut position) in
        multizip((ephemeris_times.iter().cloned(), positions.column_iter_mut()))
    {
        let (pos, _) = spice::spkpos(target, et, frame, abcorr, observer);
        position.copy_from(&pos);
    }

    // Export
    let path = "rsc/data/tmp.json";
    let mut context = BTreeMap::new();
    context.insert("frame".to_string(), json!(frame));
    context.insert("observer".to_string(), json!(observer));
    context.insert("target".to_string(), json!(target));
    context.insert("abcorr".to_string(), json!(abcorr));
    context.insert("date".to_string(), json!(date));
    context.insert("duration".to_string(), json!(duration));
    context.insert("time_step".to_string(), json!(time_step));
    let json_struct = JsonStruct { context, positions };
    let json_string = serde_json::to_string_pretty(&json_struct).unwrap();
    write_str(path, json_string);

    kernel.unload()?;
    Ok(())
}
