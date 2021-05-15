/*!
# SPICE functions for the DAS subsystem

*/

// spice_derive::create_function!(fn myfunc(n: usize) -> usize );
// spice_derive::cspice_proc!(fn myfunc(name: String, value: String) -> usize);

use crate::cstr;
use approx::assert_relative_eq;
use itertools::multizip;

spice_derive::cspice_proc!(fn furnsh<S: Into<String>>(name: S));
spice_derive::cspice_proc!(fn unload<S: Into<String>>(name: S));
spice_derive::cspice_proc!(fn str2et<S: Into<String>>(targ: S) -> f64);
spice_derive::cspice_proc!(fn spkpos<S: Into<String>>(targ: S, et: f64, frame: S, abcorr: S, obs: S) -> ([f64; 3], f64));

#[test]
fn test_derive() {
    furnsh("rsc/krn/hera_study_PO_EMA_2024.tm");

    let et = str2et("2027-MAR-23 16:00:00");
    let (position, light_time) = spkpos("DIMORPHOS", et, "J2000", "NONE", "HERA");

    let expected_position = [18.62640405424448, 21.054373008357004, -7.136291402940499];
    let expected_light_time = 0.00009674257074746383;

    for (component, expected_component) in multizip((position.iter(), expected_position.iter())) {
        assert_relative_eq!(component, expected_component, epsilon = f64::EPSILON);
    }

    assert_relative_eq!(light_time, expected_light_time, epsilon = f64::EPSILON);

    unload("rsc/krn/hera_study_PO_EMA_2024.tm");
}
