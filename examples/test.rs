use spice;

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
    let json = serde_json::json!(
        { "context": {
            "abcorr": abcorr,
            "date": date,
            "date_1": date_1,
            "date_2": date_2,
            "duration_1": duration_1,
            "duration_2": duration_2,
            "frame": frame,
            "origin": origin,
            "secondary_frame": secondary_frame,
        },
        "secondary_frame_1": secondary_frame_matrix_1,
        "secondary_frame_2": secondary_frame_matrix_2,
        }
    );
    tool::writejs!("rsc/data/tmp.json", json);

    kernel.unload()?;
    Ok(())
}
