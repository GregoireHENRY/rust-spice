use std::cell::Cell;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, Ordering};

// Atomic bool to keep track of whether an instance exists
static mut IS_LOCKED: AtomicBool = AtomicBool::new(false);

/// A wrapper singleton struct around the API to prevent concurrent calls to SPICE functions from multiple threads.
/// Exposes all functions as methods with identical signatures besides the added `&self` argument.
/// Only available with the `lock` feature enabled.
pub struct SpiceLock {
    // Private dummy field. Prevents direct instantiation and makes type `!Sync` (because `Cell` is `!Sync`)
    _x: PhantomData<Cell<()>>,
}

impl SpiceLock {
    /// Attempt to create a `SpiceLock` instance.
    /// Will be `Err` if an instance already exists.
    pub fn try_acquire() -> Result<Self, &'static str> {
        // Sets value equal to `true` if it was `false` and
        // returns a result with the previous value (`Ok` if swapped, `Err` if not)
        let was_unlocked = unsafe {
            IS_LOCKED
                .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
        };
        // If the value was changed, it was atomically set to true and no instance exists
        if was_unlocked {
            // Safely return the only instance
            Ok(Self { _x: PhantomData })
        } else {
            // A lock already exists somewhere
            Err("Cannot acquire SPICE lock: Already locked.")
        }
    }
}

impl Drop for SpiceLock {
    fn drop(&mut self) {
        unsafe {
            IS_LOCKED.store(false, Ordering::Release);
        }
    }
}
