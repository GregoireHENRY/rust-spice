use itertools::multizip;
use spice;
use tool;

fn main() {
    let mut system = spice::SystemBuilder::default()
        .kernel("rsc/data/hera_PO_EMA_2024.tm")
        .unwrap()
        .frame("J2000")
        .observer("HERA")
        .target("DIMORPHOS")
        .start_date("2027-MAR-23 16:00:00")
        .duration(129.0 * spice::DAY)
        .aberration_correction("NONE")
        .build()
        .unwrap();

    let time_step = 1.0 * spice::DAY;

    let times = system.times_formatted(time_step);
    let positions = system.positions(time_step);
    let distances = tool::distances(positions);

    for (time, distance) in multizip((times.iter(), distances.iter())) {
        println!("{} -> {:.2} km", time, distance);
    }

    system.unload().unwrap();
}
