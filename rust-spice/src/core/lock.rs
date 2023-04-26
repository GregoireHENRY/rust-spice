use std::sync::{Mutex, MutexGuard, OnceLock, TryLockError};

// global lock to keep track of things.
static LOCK: OnceLock<Mutex<()>> = OnceLock::new();

/// A wrapper singleton struct around the API to prevent concurrent calls to SPICE functions from multiple threads.
/// Exposes all functions as methods with identical signatures besides the added `&self` argument.
/// Only available with the `lock` feature enabled.
pub struct SpiceLock {
    // the guard is a RAII filed, and since drop gets called automatically, it neatly handles unlocking for us.
    _guard: MutexGuard<'static, ()>,
}

impl SpiceLock {
    /// Attempt to create a `SpiceLock` instance.
    /// Will be `Err` if an instance already exists or the underlying mutex has been poisoned.
    pub fn try_acquire() -> Result<Self, &'static str> {
        // we try to lock the mutex
        match LOCK.get_or_init(|| Mutex::new(())).try_lock() {
            // The guard can be moved, so it now lives inside the newly created spice lock.
            Ok(guard) => {
                Ok(Self {
                    _guard: guard
                })
            }
            // we might already be locked though.
            Err(TryLockError::WouldBlock) => {
                Err("Cannot acquire SPICE lock: Already locked.")
            }
            // Or some thread panicked in the past.
            // Technically this is recoverable, but for for time being its not.
            Err(TryLockError::Poisoned(_)) => {
                Err("Cannot acquire SPICE lock: Mutex poisoned (a thread panicked while holding the lock)")
            }
        }
    }

    /// Attempt to create a `SpiceLock` instance.
    /// Will **block** if an instance already exists.
    /// Will be `Err` if the underlying mutex has been poisoned.
    pub fn acquire() -> Result<Self, &'static str> {
        // try to lock the mutex
        match LOCK.get_or_init(|| Mutex::new(())).lock() {
            // all is well
            Ok(guard) => {
                Ok(Self {
                    _guard: guard
                })
            }
            // Poisoned
            Err(_) => {
                Err("Cannot acquire SPICE lock: Mutex poisoned (a thread panicked while holding the lock)")
            }
        }
    }
}
