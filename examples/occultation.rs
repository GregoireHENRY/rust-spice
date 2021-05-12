use serde_json::json;

fn main() -> Result<(), spice::KernelError> {
    tool::log_init!();

    let mut kernel =
        spice::Kernel::new("/home/greg/rob/hera/kernels/mk/hera_study_PO_EMA_2024.tm")?;
    let targ1 = "DIMORPHOS";
    let shape1 = "ELLIPSOID";
    let frame1 = "DIMORPHOS_FIXED";
    let targ2 = "DIDYMOS";
    let shape2 = "ELLIPSOID";
    let frame2 = "DIDYMOS_FIXED";
    let abcorr = "NONE";
    let observer = "SUN";
    let date = "2027-MAR-23 16:00:00";
    let time_step = 1.0 * tool::MINUTE;
    let duration = 2.0 * tool::DAY;

    let et_start = spice::str2et(date);
    let ephemeris_times = tool::linspace(et_start, et_start + duration, time_step);

    let mut occultations: Vec<spice::Occultation> = vec![];

    for et in ephemeris_times.iter() {
        let ocltid = spice::occult(
            targ1, shape1, frame1, targ2, shape2, frame2, abcorr, observer, *et,
        );
        occultations.push(ocltid);
    }

    // Export.
    let json = json!({
        "context": {
            "targ1": targ1,
            "shape1": shape1,
            "frame1": frame1,
            "targ2": targ2,
            "shape2": shape2,
            "frame2": frame2,
            "abcorr": abcorr,
            "observer": observer,
            "date": date,
            "duration": duration,
            "time_step": time_step,
        },
        "ephemeris_times": ephemeris_times,
        "occultations": occultations,
    });
    tool::writejs!("rsc/data/tmp/occultation.json", json);

    kernel.unload()?;
    Ok(())
}
