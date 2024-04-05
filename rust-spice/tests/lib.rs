#[macro_use]
extern crate approx;
extern crate nalgebra as na;
extern crate spice;
#[macro_use]
extern crate serial_test;

#[cfg(not(feature = "lock"))]
mod core;

use std::ffi::CString;

#[test]
#[serial]
fn test_c() {
    unsafe {
        let kernel = CString::new(
            "/Users/gregoireh/data/spice-kernels/hera/kernels/mk/hera_study_PO_EMA_2024.tm",
        )
        .unwrap()
        .into_raw();
        spice::c::furnsh_c(kernel);
        spice::c::unload_c(kernel);
    }
}

#[cfg(feature = "lock")]
pub mod lock_tests {
    #[test]
    #[serial]
    fn acquire_fail() {
        let _sl = spice::SpiceLock::try_acquire().unwrap();
        assert!(spice::SpiceLock::try_acquire().is_err());
    }
    #[test]
    #[serial]
    fn reacquire() {
        let sl = spice::SpiceLock::try_acquire().unwrap();
        drop(sl);

        let try_new_sl = spice::SpiceLock::try_acquire();
        assert!(try_new_sl.is_ok());
    }
    #[test]
    #[serial]
    fn str2et() {
        let sl = spice::SpiceLock::try_acquire().unwrap();
        sl.furnsh("/Users/gregoireh/data/spice-kernels/hera/kernels/mk/hera_study_PO_EMA_2024.tm");

        let et = sl.str2et("2027-MAR-23 16:00:00");

        assert_relative_eq!(et, 859089669.1856234, epsilon = f64::EPSILON);

        sl.unload("/Users/gregoireh/data/spice-kernels/hera/kernels/mk/hera_study_PO_EMA_2024.tm");
    }
    #[test]
    #[serial]
    fn spkezr() {
        let sl = spice::SpiceLock::try_acquire().unwrap();

        sl.furnsh("/Users/gregoireh/data/spice-kernels/hera/kernels/mk/hera_study_PO_EMA_2024.tm");

        // an arbitrary time
        let et = sl.str2et("2021-01-06 09:36:09.1825432 TDB");

        // sun in relation to ssb
        let (sun_ssb_posvec, _sun_ssb_lt) = sl.spkezr("sun", et, "j2000", "none", "ssb");
        // earth in relation to ssb
        let (earth_ssb_posvec, _earth_ssb_lt) = sl.spkezr("earth", et, "j2000", "none", "ssb");
        // earth in relation to sun
        let (earth_sun_posvec, _earth_sun_ly) = sl.spkezr("earth", et, "j2000", "none", "sun");

        // Quick check that the (Sun relative) earth velocity vectors are the same regardless of whether we
        // calculate them indirectly from SB or directly compared to  the Sun
        assert_eq!(earth_ssb_posvec[3] - sun_ssb_posvec[3], earth_sun_posvec[3]);
        assert_eq!(earth_ssb_posvec[4] - sun_ssb_posvec[4], earth_sun_posvec[4]);
        assert_eq!(earth_ssb_posvec[5] - sun_ssb_posvec[5], earth_sun_posvec[5]);

        sl.unload("/Users/gregoireh/data/spice-kernels/hera/kernels/mk/hera_study_PO_EMA_2024.tm");
    }
    #[test]
    #[serial]
    fn cell() {
        let sl = spice::SpiceLock::try_acquire().unwrap();

        sl.furnsh("/Users/gregoireh/data/spice-kernels/hera/kernels/mk/hera_study_PO_EMA_2024.tm");

        let (file, _, _, _, found) = sl.kdata(1, "dsk");
        assert!(found);

        let cell = sl.dskobj(&file);

        assert_eq!(cell.card, 1);
        assert_eq!(cell.get_data_int(0), -658031);

        assert_eq!(sl.bodc2n(cell.get_data_int(0)).0, "DIMORPHOS");

        sl.kclear();
    }
    #[test]
    #[serial]
    fn multiple_threads() {
        use std::sync::{Arc, Mutex};
        use std::thread;

        let sl = spice::SpiceLock::try_acquire().unwrap();
        sl.furnsh("/Users/gregoireh/data/spice-kernels/hera/kernels/mk/hera_study_PO_EMA_2024.tm");

        let sl = Arc::new(Mutex::new(sl));

        let n_children = 5;
        let mut children = Vec::with_capacity(n_children);

        for _ in 0..n_children {
            let sl = Arc::clone(&sl);
            children.push(thread::spawn(move || {
                for _ in 0..10 {
                    // If these calls were not guarded by the lock, they could lead to data races and UB
                    sl.lock().unwrap().str2et("2027-MAR-23 16:00:00");
                }
            }));
        }

        for c in children {
            c.join().unwrap();
        }

        sl.lock().unwrap().unload(
            "/Users/gregoireh/data/spice-kernels/hera/kernels/mk/hera_study_PO_EMA_2024.tm",
        );
    }
}
