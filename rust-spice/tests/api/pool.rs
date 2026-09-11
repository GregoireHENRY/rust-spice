//! The kernel pool, and the KEEPER subsystem that feeds it.

use crate::assert_slice_eq;
use crate::common;

#[test]
#[serial]
fn furnsh_and_unload() {
    common::unload();
    assert_eq!(spice::ktotal("ALL"), 0);

    spice::furnsh(&common::meta());
    common::assert_ok("furnsh");
    let loaded = spice::ktotal("ALL");
    assert!(
        loaded > 1,
        "expected the meta-kernel and its contents, got {loaded}"
    );
    assert_eq!(spice::ktotal("SPK"), 1);
    assert_eq!(spice::ktotal("CK"), 1);
    assert_eq!(spice::ktotal("DSK"), 1);
    assert_eq!(spice::ktotal("META"), 1);

    spice::unload(&common::kernel("test.bsp"));
    assert_eq!(spice::ktotal("SPK"), 0);

    common::unload();
    assert_eq!(spice::ktotal("ALL"), 0);
}

#[test]
#[serial]
fn kclear() {
    common::load();
    assert!(spice::ktotal("ALL") > 0);

    spice::kclear();
    assert_eq!(spice::ktotal("ALL"), 0);
    // The pool goes with the kernels.
    assert!(!spice::expool("BODY399_RADII"));
}

#[test]
#[serial]
fn binary_pck() {
    common::load();

    let pck = common::kernel("test.bpc");

    assert_eq!(spice::pckfrm(&pck).to_vec(), vec![common::PCK_FRAME]);

    let coverage = spice::pckcov(&pck, common::PCK_FRAME);
    common::assert_ok("pckcov");
    assert_eq!(coverage.len(), 2);
    assert_relative_eq!(coverage.get(0).unwrap(), common::FIRST, epsilon = 1e-6);
    assert_relative_eq!(coverage.get(1).unwrap(), common::LAST, epsilon = 1e-6);

    // A frame the file says nothing about has no coverage.
    assert!(spice::pckcov(&pck, -1).is_empty());

    common::unload();
}

#[test]
#[serial]
fn kdata_and_kinfo() {
    common::load();

    let (file, kind, source, handle, found) = spice::kdata(0, "SPK");
    assert!(found);
    assert_eq!(file, common::kernel("test.bsp"));
    assert_eq!(kind, "SPK");
    assert_eq!(source, common::meta());
    assert!(handle > 0);

    // Past the end of the list, nothing is found.
    assert!(!spice::kdata(9, "SPK").4);

    let (kind, source, from_kinfo, found) = spice::kinfo(&file);
    assert!(found);
    assert_eq!(kind, "SPK");
    assert_eq!(source, common::meta());
    assert_eq!(from_kinfo, handle);

    assert!(!spice::kinfo("no-such-kernel.bsp").3);

    common::unload();
}

#[test]
#[serial]
fn doubles() {
    common::load();

    let radii = spice::gdpool("BODY399_RADII", 0, 3);
    assert_slice_eq(&radii, &[6378.1366, 6378.1366, 6356.7519], 1e-9);

    // `start` skips the leading components.
    assert_slice_eq(&spice::gdpool("BODY399_RADII", 2, 3), &[6356.7519], 1e-9);
    // An absent variable yields nothing.
    assert!(spice::gdpool("NO_SUCH_VARIABLE", 0, 3).is_empty());

    spice::pdpool("TEST_DOUBLES", &[1.5, -2.5, 3.5]);
    assert_slice_eq(
        &spice::gdpool("TEST_DOUBLES", 0, 5),
        &[1.5, -2.5, 3.5],
        1e-15,
    );

    common::unload();
}

#[test]
#[serial]
fn integers() {
    common::load();

    spice::pipool("TEST_INTEGERS", &[7, -8, 9]);
    assert_eq!(spice::gipool("TEST_INTEGERS", 0, 5), vec![7, -8, 9]);
    assert_eq!(spice::gipool("TEST_INTEGERS", 1, 5), vec![-8, 9]);
    assert!(spice::gipool("NO_SUCH_VARIABLE", 0, 3).is_empty());

    common::unload();
}

#[test]
#[serial]
fn characters() {
    common::load();

    spice::pcpool("TEST_STRINGS", &["alpha", "beta", "gamma"]);
    assert_eq!(
        spice::gcpool("TEST_STRINGS", 0, 5),
        vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()]
    );
    assert_eq!(
        spice::gcpool("TEST_STRINGS", 2, 5),
        vec!["gamma".to_string()]
    );
    assert!(spice::gcpool("NO_SUCH_VARIABLE", 0, 3).is_empty());

    common::unload();
}

#[test]
#[serial]
fn expool_and_dtpool() {
    common::load();

    assert!(spice::expool("BODY399_RADII"));
    assert!(!spice::expool("NO_SUCH_VARIABLE"));

    let (found, count, kind) = spice::dtpool("BODY399_RADII");
    assert!(found);
    assert_eq!(count, 3);
    assert_eq!(kind, "N");

    spice::pcpool("TEST_STRINGS", &["alpha"]);
    let (found, count, kind) = spice::dtpool("TEST_STRINGS");
    assert!(found);
    assert_eq!(count, 1);
    assert_eq!(kind, "C");

    assert!(!spice::dtpool("NO_SUCH_VARIABLE").0);

    common::unload();
}

#[test]
#[serial]
fn ldpool_and_clpool() {
    common::unload();

    // `ldpool` fills the pool without going through the KEEPER subsystem.
    spice::ldpool(&common::kernel("test.tpc"));
    common::assert_ok("ldpool");
    assert!(spice::expool("BODY399_RADII"));
    assert_eq!(spice::ktotal("ALL"), 0);

    spice::clpool();
    assert!(!spice::expool("BODY399_RADII"));
}
