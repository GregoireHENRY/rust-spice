use spice;
use std::ffi::CString;

fn main() {
    unsafe {
        let kernel = CString::new("/path/to/metakernel.mk").unwrap().into_raw();
        spice::c::furnsh_c(kernel);

        let mut ephemeris_time = 0.0;
        let date = CString::new("2027-MAR-23 16:00:00").unwrap().into_raw();
        spice::c::str2et_c(date, &mut ephemeris_time);

        let target_c = CString::new("TARGET_NAME").unwrap().into_raw();
        let frame_c = CString::new("FRAME_NAME").unwrap().into_raw();
        let abcorr_c = CString::new("NONE").unwrap().into_raw();
        let observer_c = CString::new("SUN").unwrap().into_raw();
        let mut light_time = 0.0;
        let mut position = [0.0, 0.0, 0.0];
        spice::c::spkpos_c(
            target_c,
            ephemeris_time,
            frame_c,
            abcorr_c,
            observer_c,
            &mut position[0],
            &mut light_time,
        );

        spice::c::unload_c(kernel);
    }
}
