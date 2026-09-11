/*!
Integration tests for the guarded API of the `lock` feature.

With `lock` enabled the free functions are hidden, and the whole API is reached through
[`spice::SpiceLock`] instead. These tests check that the guard behaves like a singleton, that the
methods it exposes agree with the free functions they wrap, and that it makes the toolkit usable
from several threads.
*/

#![cfg(feature = "lock")]

#[macro_use]
extern crate approx;
#[macro_use]
extern crate serial_test;

#[path = "../common/mod.rs"]
mod common;

use spice::SpiceLock;

/// Acquire the lock and load the test kernels through it.
fn acquire() -> SpiceLock {
    common::reset();
    let lock = SpiceLock::try_acquire().expect("no other lock should be held");
    lock.quiet();
    lock.furnsh(&common::meta());
    common::assert_ok("loading the test meta-kernel");
    lock
}

#[test]
#[serial]
fn only_one_lock_at_a_time() {
    let lock = SpiceLock::try_acquire().unwrap();
    assert!(SpiceLock::try_acquire().is_err());
    drop(lock);

    // Dropping it releases the guard.
    let lock = SpiceLock::try_acquire();
    assert!(lock.is_ok());
    drop(lock);
    assert!(SpiceLock::try_acquire().is_ok());
}

#[test]
#[serial]
fn raw_functions_are_exposed() {
    let sl = acquire();

    let et = sl.str2et("2000-JAN-02 00:00:00 TDB");
    assert_relative_eq!(et, 43200.0, epsilon = 1e-9);

    let (position, light_time) = sl.spkpos("EARTH", et, "J2000", "NONE", "SUN");
    common::assert_ok("spkpos");
    for (component, expected) in position.iter().zip(common::analytic_state(et).iter()) {
        assert_relative_eq!(component, expected, epsilon = 1e-3);
    }
    assert_relative_eq!(
        light_time,
        sl.vnorm(position) / sl.clight(),
        epsilon = 1e-12
    );

    assert_eq!(sl.bodn2c("EARTH"), (399, true));
    assert!(sl.bodfnd(399, "RADII"));
    assert_relative_eq!(
        sl.vdot([1.0, 2.0, 3.0], [1.0, 2.0, 3.0]),
        14.0,
        epsilon = f64::EPSILON
    );

    sl.kclear();
}

#[test]
#[serial]
fn neat_functions_are_preferred() {
    let sl = acquire();

    // `bodc2n` has a neat version, so the lock exposes the one that sizes the buffer itself.
    assert_eq!(sl.bodc2n(399), ("EARTH".to_string(), true));
    // The default picture reports UTC, which trails the TDB epoch by the leap seconds.
    assert_eq!(sl.timout(0.0, spice::TIME_FORMAT), "2000-JAN-01 11:58:56");

    // And the cell returning ones hand back an owned cell.
    let bodies = sl.dskobj(&common::kernel("test.bds"));
    common::assert_ok("dskobj");
    assert_eq!(bodies.to_vec(), vec![common::TARGET]);

    let coverage = sl.spkcov(&common::kernel("test.bsp"), common::TARGET);
    assert_eq!(coverage.len(), 2);
    assert_relative_eq!(coverage.get(0).unwrap(), common::FIRST, epsilon = 1e-6);

    sl.kclear();
}

#[test]
#[serial]
fn errors_are_reachable() {
    let sl = acquire();

    assert!(!sl.failed());
    assert!(sl.check().is_ok());

    sl.furnsh("no-such-kernel.tm");
    assert!(sl.failed());
    assert_eq!(sl.getmsg("SHORT"), "SPICE(NOSUCHFILE)");

    let error = sl.check().expect_err("furnsh should have failed");
    assert_eq!(error.short, "SPICE(NOSUCHFILE)");
    assert!(!sl.failed(), "`check` resets the state");

    assert_eq!(sl.erract("GET", ""), "RETURN");
    sl.reset();

    sl.kclear();
}

#[test]
#[serial]
fn the_lock_can_be_shared_between_threads() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let sl = Arc::new(Mutex::new(acquire()));

    let children = (0..5)
        .map(|_| {
            let sl = Arc::clone(&sl);
            thread::spawn(move || {
                (0..10)
                    .map(|_| {
                        // Unguarded, these calls would race inside CSPICE's shared mutable state.
                        let sl = sl.lock().unwrap();
                        sl.str2et("2000-JAN-02 00:00:00 TDB")
                    })
                    .collect::<Vec<_>>()
            })
        })
        .collect::<Vec<_>>();

    for child in children {
        for value in child.join().unwrap() {
            assert_relative_eq!(value, 43200.0, epsilon = 1e-9);
        }
    }

    sl.lock().unwrap().kclear();
}
