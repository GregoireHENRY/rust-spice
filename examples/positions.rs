use itertools::multizip;
use spice;

fn main() {
    let mut system = spice::System::new(
        "rsc/data/hera_PO_EMA_2024.tm", // kernel
        "J2000",                        // frame
        "HERA",                         // observer
        "DIMORPHOS",                    // target
        "2027-MAR-23 16:00:00",         // start date
        129.0 * spice::DAY(),           // duration
        "NONE",                         // aberration correction
    );
    let time_step = 1.0 * spice::DAY();

    let times = system.times_formatted(time_step);
    let mut positions = system.positions(time_step);
    let distances = spice::compute_distances(&mut positions);

    for (time, distance) in multizip((times.iter(), distances.iter())) {
        println!("{} -> {:.2} km", time, distance);
    }

    system.unload();
}
