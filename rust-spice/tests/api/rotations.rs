//! Rotation matrices, quaternions, Euler angles and state transformations.

use crate::common;
use crate::{assert_matrix_eq, assert_slice_eq};

const IDENTITY: [[f64; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

/// An arbitrary rotation, built so the tests do not lean on a special case.
fn rotation() -> [[f64; 3]; 3] {
    spice::eul2m(0.3, -0.7, 1.1, 3, 2, 1)
}

#[test]
#[serial]
fn quaternions() {
    common::reset();

    // The identity rotation is the quaternion (1, 0, 0, 0).
    assert_slice_eq(&spice::m2q(IDENTITY), &[1.0, 0.0, 0.0, 0.0], 1e-15);
    assert_matrix_eq(&spice::q2m([1.0, 0.0, 0.0, 0.0]), &IDENTITY, 1e-15);

    // A quaternion and its matrix describe the same rotation, whichever way round you go.
    let matrix = rotation();
    assert_matrix_eq(&spice::q2m(spice::m2q(matrix)), &matrix, 1e-14);

    // Every unit quaternion stays one.
    let q = spice::m2q(matrix);
    assert_relative_eq!(q.iter().map(|c| c * c).sum::<f64>(), 1.0, epsilon = 1e-14);

    // Composing quaternions composes the rotations they stand for.
    let other = spice::axisar([0.0, 0.0, 1.0], std::f64::consts::FRAC_PI_3);
    let composed = spice::qxq(spice::m2q(matrix), spice::m2q(other));
    assert_matrix_eq(&spice::q2m(composed), &spice::mxm(matrix, other), 1e-13);
}

#[test]
#[serial]
fn axis_and_angle() {
    common::reset();

    // A quarter turn about the third axis, as `rotate` would build it but the other way round:
    // `axisar` rotates vectors, `rotate` rotates the frame.
    let quarter = std::f64::consts::FRAC_PI_2;
    let matrix = spice::axisar([0.0, 0.0, 1.0], quarter);
    assert_matrix_eq(&matrix, &spice::xpose(spice::rotate(quarter, 3)), 1e-15);

    let (axis, angle) = spice::raxisa(matrix);
    assert_slice_eq(&axis, &[0.0, 0.0, 1.0], 1e-15);
    assert_relative_eq!(angle, quarter, epsilon = 1e-15);

    // And the round trip for an arbitrary rotation.
    let matrix = rotation();
    let (axis, angle) = spice::raxisa(matrix);
    assert_matrix_eq(&spice::axisar(axis, angle), &matrix, 1e-14);
}

#[test]
#[serial]
fn euler_angles() {
    common::reset();

    // A single rotation about one axis is what `rotate` gives.
    assert_matrix_eq(
        &spice::eul2m(0.0, 0.0, 0.4, 3, 2, 1),
        &spice::rotate(0.4, 1),
        1e-15,
    );

    // Factoring a matrix returns the angles it was built from.
    let (a3, a2, a1) = (0.3, -0.7, 1.1);
    let matrix = spice::eul2m(a3, a2, a1, 3, 2, 1);
    let factored = spice::m2eul(matrix, 3, 2, 1);
    assert_slice_eq(&[factored.0, factored.1, factored.2], &[a3, a2, a1], 1e-14);
}

#[test]
#[serial]
fn state_transformations() {
    common::reset();

    // A rotation with no angular velocity is block diagonal, repeating the rotation twice.
    let matrix = rotation();
    let xform = spice::rav2xf(matrix, [0.0; 3]);
    for (row, expected) in xform.iter().take(3).zip(matrix.iter()) {
        assert_slice_eq(&row[..3], expected, 1e-15);
        assert_slice_eq(&row[3..], &[0.0; 3], 1e-15);
    }

    // Splitting it gives back what it was built from.
    let (rot, av) = spice::xf2rav(xform);
    assert_matrix_eq(&rot, &matrix, 1e-15);
    assert_slice_eq(&av, &[0.0; 3], 1e-15);

    // With an angular velocity, the round trip still holds.
    let spin = [0.01, -0.02, 0.03];
    let (rot, av) = spice::xf2rav(spice::rav2xf(matrix, spin));
    assert_matrix_eq(&rot, &matrix, 1e-15);
    assert_slice_eq(&av, &spin, 1e-15);

    // Euler angles and their derivatives round trip through a state transformation too.
    let eulang = [0.3, -0.7, 1.1, 0.01, 0.02, -0.03];
    let (recovered, unique) = spice::xf2eul(spice::eul2xf(eulang, 3, 2, 1), 3, 2, 1);
    assert!(unique, "the factorisation should not be degenerate here");
    assert_slice_eq(&recovered, &eulang, 1e-13);

    // And the rotation block matches what `eul2m` builds from the same angles.
    let xform = spice::eul2xf(eulang, 3, 2, 1);
    assert_matrix_eq(
        &spice::xf2rav(xform).0,
        &spice::eul2m(0.3, -0.7, 1.1, 3, 2, 1),
        1e-14,
    );
}

#[test]
#[serial]
fn orthogonality() {
    common::reset();

    let matrix = rotation();
    assert!(spice::isrot(matrix, 1e-12, 1e-12));
    assert!(spice::isrot(IDENTITY, 1e-12, 1e-12));
    // A matrix that scales is not a rotation.
    assert!(!spice::isrot(
        [[2.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        1e-12,
        1e-12
    ));

    // For a rotation, `invort` and `xpose` agree.
    assert_matrix_eq(&spice::invort(matrix), &spice::xpose(matrix), 1e-14);

    // For rows that are orthogonal but not unit length, it inverts the scaling as well.
    let scaled = [[2.0, 0.0, 0.0], [0.0, 4.0, 0.0], [0.0, 0.0, 8.0]];
    assert_matrix_eq(
        &spice::invort(scaled),
        &[[0.5, 0.0, 0.0], [0.0, 0.25, 0.0], [0.0, 0.0, 0.125]],
        1e-15,
    );
}
