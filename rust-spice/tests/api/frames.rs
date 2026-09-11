//! Reference frames.

use crate::common;
use crate::{assert_matrix_eq, assert_slice_eq};

/// The rotation from `J2000` to `TEST_FIXED`; see the test frame kernel.
///
/// A `TKFRAME_*_ANGLES` assignment gives the angles that rotate the *frame*, which is the opposite
/// of the rotation applied to coordinates, so 90 degrees about the third axis produces the
/// transpose of `rotate(pi/2, 3)`.
const QUARTER_TURN: [[f64; 3]; 3] = [[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]];

#[test]
#[serial]
fn pxform() {
    common::load();

    let identity = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    assert_matrix_eq(
        &spice::pxform("J2000", "J2000", common::EPOCH),
        &identity,
        1e-15,
    );

    // A fixed offset frame gives the same matrix at any epoch.
    assert_matrix_eq(
        &spice::pxform("J2000", "TEST_FIXED", 0.0),
        &QUARTER_TURN,
        1e-15,
    );
    assert_matrix_eq(
        &spice::pxform("J2000", "TEST_FIXED", common::EPOCH),
        &QUARTER_TURN,
        1e-15,
    );

    // The CK frame holds the identity rotation over the whole coverage.
    assert_matrix_eq(
        &spice::pxform("J2000", "TEST_INSTRUMENT", common::EPOCH),
        &identity,
        1e-15,
    );

    common::unload();
}

#[test]
#[serial]
fn pxfrm2() {
    common::load();

    // Same frame, same epoch: the identity.
    let matrix = spice::pxfrm2("J2000", "J2000", common::EPOCH, common::EPOCH + 1800.0);
    assert_matrix_eq(
        &matrix,
        &[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        1e-15,
    );

    // A fixed frame does not move, so the two epochs make no difference.
    let matrix = spice::pxfrm2("J2000", "TEST_FIXED", 0.0, common::EPOCH);
    assert_matrix_eq(&matrix, &QUARTER_TURN, 1e-15);

    common::unload();
}

#[test]
#[serial]
fn sxform() {
    common::load();

    let state = spice::sxform("J2000", "TEST_FIXED", common::EPOCH);
    common::assert_ok("sxform");

    // The upper left block is the position transformation...
    for (row, expected) in state.iter().take(3).zip(QUARTER_TURN.iter()) {
        assert_slice_eq(&row[..3], expected, 1e-15);
        // ... and the upper right block is zero for a time independent rotation.
        assert_slice_eq(&row[3..], &[0.0; 3], 1e-15);
    }
    // The lower right block repeats the rotation, the lower left one its derivative.
    for (row, expected) in state.iter().skip(3).zip(QUARTER_TURN.iter()) {
        assert_slice_eq(&row[..3], &[0.0; 3], 1e-15);
        assert_slice_eq(&row[3..], expected, 1e-15);
    }

    // Applying it to a state rotates position and velocity alike.
    let (before, _) = spice::spkezr("MOON", common::EPOCH, "J2000", "NONE", "EARTH");
    let (after, _) = spice::spkezr("MOON", common::EPOCH, "TEST_FIXED", "NONE", "EARTH");
    let rotated = (0..6)
        .map(|row| (0..6).map(|col| state[row][col] * before[col]).sum::<f64>())
        .collect::<Vec<_>>();
    assert_slice_eq(&rotated, &after, 1e-9);

    common::unload();
}

#[test]
#[serial]
fn names_and_codes() {
    common::load();

    assert_eq!(spice::namfrm("J2000"), 1);
    assert_eq!(spice::frmnam(1), "J2000");

    assert_eq!(spice::namfrm("TEST_FIXED"), common::FIXED_FRAME);
    assert_eq!(spice::frmnam(common::FIXED_FRAME), "TEST_FIXED");

    // An unknown frame name maps to zero, and an unknown code to the empty string.
    assert_eq!(spice::namfrm("NO SUCH FRAME"), 0);
    assert!(spice::frmnam(-123_456_789).is_empty());

    common::unload();
}
