//! The CSPICE error state, and its translation into a Rust `Result`.

use crate::common;

#[test]
#[serial]
fn a_failing_call_is_reported() {
    common::load();
    spice::errors::quiet();

    assert!(!spice::errors::failed());
    assert!(spice::errors::check().is_ok());

    spice::furnsh("no-such-kernel.tm");
    assert!(spice::errors::failed());

    let error = spice::errors::check().expect_err("furnsh should have failed");
    assert_eq!(error.short, "SPICE(NOSUCHFILE)");
    assert!(
        error.long.contains("no-such-kernel.tm"),
        "got {:?}",
        error.long
    );
    assert!(!error.traceback.is_empty());
    assert!(error.to_string().contains("SPICE(NOSUCHFILE)"));

    // `check` resets the state on its way out.
    assert!(!spice::errors::failed());
    assert!(spice::errors::check().is_ok());

    common::unload();
}

#[test]
#[serial]
fn getmsg_and_reset() {
    common::load();
    spice::errors::quiet();

    spice::furnsh("no-such-kernel.tm");
    assert_eq!(spice::errors::getmsg("SHORT"), "SPICE(NOSUCHFILE)");
    assert!(spice::errors::getmsg("LONG").contains("no-such-kernel.tm"));

    spice::errors::reset();
    assert!(!spice::errors::failed());
    assert!(spice::errors::getmsg("SHORT").is_empty());

    common::unload();
}

#[test]
#[serial]
fn qcktrc() {
    common::load();
    spice::errors::quiet();

    // Nothing has failed, so the traceback holds no active routine.
    assert!(spice::errors::qcktrc().is_empty());

    spice::furnsh("no-such-kernel.tm");
    assert!(spice::errors::qcktrc().contains("FURNSH"));

    spice::errors::reset();
    common::unload();
}

#[test]
#[serial]
fn the_action_and_device_can_be_read_back() {
    common::load();

    spice::errors::quiet();
    assert_eq!(spice::errors::erract("GET", ""), "RETURN");
    assert_eq!(spice::errors::errdev("GET", ""), "NULL");

    spice::errors::erract("SET", "REPORT");
    assert_eq!(spice::errors::erract("GET", ""), "REPORT");

    // The list of reported items is settable too.
    spice::errors::errprt("SET", "SHORT");
    assert!(spice::errors::errprt("GET", "").contains("SHORT"));

    // `loud` restores the CSPICE defaults; put the guard back afterwards so that a later failure
    // does not take the test runner down with it.
    spice::errors::loud();
    assert_eq!(spice::errors::erract("GET", ""), "ABORT");
    assert_eq!(spice::errors::errdev("GET", ""), "SCREEN");
    spice::errors::quiet();

    common::unload();
}
