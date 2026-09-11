/*!
CSPICE error handling.

## Description

CSPICE keeps its own error state. By default, a routine that fails prints a long report to the
screen and **terminates the process** — fine for a Fortran program, fatal for a library. Calling
[`quiet`] once, at start-up, switches the toolkit to the `"RETURN"` action with no output device:
failing routines then simply return, and [`check`] turns that state into a Rust [`Result`].

```no_run
# #[cfg(not(feature = "lock"))]
# {
spice::errors::quiet();

spice::furnsh("does-not-exist.tm");
if let Err(error) = spice::errors::check() {
    eprintln!("{error}");
}
# }
```

[`check`] resets the error state, so the next call starts from a clean slate.

See the [C documentation](https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/req/error.html).
*/

use crate::c::{SpiceChar, SpiceInt};
use crate::core::ffi::{from_cbuf, to_cstring};
use crate::MAX_LEN_OUT;
use std::fmt;

#[cfg(any(feature = "lock", doc))]
use {crate::core::lock::SpiceLock, spice_derive::impl_for};

/**
An error reported by CSPICE.
*/
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    /// Short, machine readable name, such as `SPICE(NOSUCHFILE)`.
    pub short: String,
    /// Explanation of the short message.
    pub explain: String,
    /// Long message, describing what went wrong in this particular call.
    pub long: String,
    /// The chain of routines that were active when the error was signalled.
    pub traceback: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.short)?;
        for detail in [&self.explain, &self.long] {
            if !detail.is_empty() {
                write!(f, ": {detail}")?;
            }
        }
        if !self.traceback.is_empty() {
            write!(f, " [{}]", self.traceback)?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {}

/**
Whether CSPICE is in an error state.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn failed() -> bool {
    unsafe { crate::c::failed_c() != 0 }
}

/**
Clear the CSPICE error state.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn reset() {
    unsafe { crate::c::reset_c() }
}

/**
Retrieve one of the error messages: `"SHORT"`, `"EXPLAIN"` or `"LONG"`.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn getmsg(option: &str) -> String {
    let option = to_cstring(option);
    let mut message = vec![0 as SpiceChar; MAX_LEN_OUT];
    unsafe {
        crate::c::getmsg_c(
            option.as_ptr() as *mut SpiceChar,
            message.len() as SpiceInt,
            message.as_mut_ptr(),
        )
    };
    from_cbuf(&message)
}

/**
Return the traceback of the routines that were active when the error was signalled.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn qcktrc() -> String {
    let mut trace = vec![0 as SpiceChar; MAX_LEN_OUT];
    unsafe { crate::c::qcktrc_c(trace.len() as SpiceInt, trace.as_mut_ptr()) };
    from_cbuf(&trace)
}

/**
Turn the current CSPICE error state into a [`Result`], clearing it on the way.

`Ok(())` when no routine has failed since the last reset.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn check() -> Result<(), Error> {
    if !failed() {
        return Ok(());
    }

    let error = Error {
        short: getmsg("SHORT"),
        explain: getmsg("EXPLAIN"),
        long: getmsg("LONG"),
        traceback: qcktrc(),
    };
    reset();
    Err(error)
}

/// Build the in/out buffer the `err*_c` routines use for both `"SET"` and `"GET"`.
fn inout(value: &str) -> Vec<SpiceChar> {
    let mut buffer = vec![0 as SpiceChar; MAX_LEN_OUT.max(value.len() + 1)];
    for (target, byte) in buffer.iter_mut().zip(value.as_bytes()) {
        *target = *byte as SpiceChar;
    }
    buffer
}

/// Generate the wrappers of the three `operation`/`value` error routines.
macro_rules! inout_routine {
    ($($name:ident => $cname:ident, $doc:expr);* $(;)?) => {$(
        #[doc = $doc]
        ///
        /// `op` is `"SET"` to install `value`, or `"GET"` to read the current one back; the current
        /// value is returned either way.
        #[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
        pub fn $name(op: &str, value: &str) -> String {
            let op = to_cstring(op);
            let mut buffer = inout(value);
            unsafe {
                crate::c::$cname(
                    op.as_ptr() as *mut SpiceChar,
                    buffer.len() as SpiceInt,
                    buffer.as_mut_ptr(),
                )
            };
            from_cbuf(&buffer)
        }
    )*};
}

inout_routine! {
    erract => erract_c, "Retrieve or set the default error action.";
    errdev => errdev_c, "Retrieve or set the name of the current output device for error messages.";
    errprt => errprt_c, "Retrieve or set the list of error message items to be output.";
}

/**
Stop CSPICE from writing to the screen and from terminating the process on error.

Sets the error action to `"RETURN"` and the error device to `"NULL"`; pair it with [`check`].
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn quiet() {
    erract("SET", "RETURN");
    errdev("SET", "NULL");
}

/**
Restore the CSPICE defaults: report to the screen, then abort the process.
*/
#[cfg_attr(any(feature = "lock", doc), impl_for(SpiceLock))]
pub fn loud() {
    erract("SET", "ABORT");
    errdev("SET", "SCREEN");
}
