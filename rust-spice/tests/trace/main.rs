/*!
Turning the traceback off.

[`spice::trcoff`] cannot be undone: once tracing is off it stays off for the rest of the process,
and every later traceback comes back empty. So it gets a process of its own, rather than leaving
the rest of the suite unable to report where a failure came from.
*/

#![cfg(not(feature = "lock"))]

#[macro_use]
extern crate serial_test;

#[path = "../common/mod.rs"]
mod common;

#[test]
#[serial]
fn tracing_can_be_turned_off() {
    common::reset();
    spice::errors::quiet();

    // Out of the box, a failure says which routines it passed through.
    spice::furnsh("no-such-kernel.tm");
    let error = spice::errors::check().expect_err("furnsh should have failed");
    assert!(
        error.traceback.contains("furnsh_c"),
        "got {:?}",
        error.traceback
    );

    spice::trcoff();

    // From here on the failure is still reported, but without the trail.
    spice::furnsh("no-such-kernel.tm");
    let error = spice::errors::check().expect_err("furnsh should still fail");
    assert_eq!(error.short, "SPICE(NOSUCHFILE)");
    assert!(error.traceback.is_empty(), "got {:?}", error.traceback);

    spice::errors::loud();
}
