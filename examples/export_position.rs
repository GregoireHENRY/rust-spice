use itertools::multizip;
use serde_json::json;
use spice;
use tool::Vectors;

fn main() -> Result<(), spice::KernelError> {
    let mut kernel = spice::Kernel::new("rsc/krn/hera_study_PO_EMA_2024.tm")?;
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
    let json = json!({
        "context": {
            "frame": frame,
            "observer": observer,
            "target": target,
            "abcorr": abcorr,
            "date": date,
            "duration": duration,
            "time_step": time_step,
        },
        "positions": positions,
    });
    tool::writejs!("rsc/data/tmp/position.json", json);

    kernel.unload()?;
    Ok(())
}
