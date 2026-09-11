/*!
A guard making the CSPICE API usable from several threads.

See the [multi-threaded usage][crate#multi-threaded-usage] section of the crate documentation.
*/

use std::cell::Cell;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, Ordering};

/// Whether an instance currently exists.
static IS_LOCKED: AtomicBool = AtomicBool::new(false);

/**
A wrapper singleton struct around the API to prevent concurrent calls to SPICE functions from
multiple threads.

Exposes all functions as methods with identical signatures besides the added `&self` argument.
Only available with the `lock` feature enabled.
*/
pub struct SpiceLock {
    // Private dummy field. Prevents direct instantiation and makes the type `!Sync`, because
    // `Cell` is `!Sync`.
    _x: PhantomData<Cell<()>>,
}

impl SpiceLock {
    /**
    Attempt to create a `SpiceLock` instance.

    Will be `Err` if an instance already exists.
    */
    pub fn try_acquire() -> Result<Self, &'static str> {
        // Sets the value to `true` if it was `false`, and reports whether the swap happened. If it
        // did, this is the only instance in the process.
        match IS_LOCKED.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) {
            Ok(_) => Ok(Self { _x: PhantomData }),
            Err(_) => Err("Cannot acquire SPICE lock: Already locked."),
        }
    }
}

impl Drop for SpiceLock {
    fn drop(&mut self) {
        IS_LOCKED.store(false, Ordering::Release);
    }
}
