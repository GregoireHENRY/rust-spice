//! Vector and matrix algebra.

use crate::{assert_matrix_eq, assert_slice_eq};

const A: [f64; 3] = [1.0, 2.0, 3.0];
const B: [f64; 3] = [4.0, -5.0, 6.0];
const M: [[f64; 3]; 3] = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 10.0]];
const N: [[f64; 3]; 3] = [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]];

#[test]
#[serial]
fn arithmetic() {
    assert_eq!(spice::vadd(A, B), [5.0, -3.0, 9.0]);
    assert_eq!(spice::vsub(A, B), [-3.0, 7.0, -3.0]);
    assert_eq!(spice::vscl(2.0, A), [2.0, 4.0, 6.0]);
    assert_eq!(spice::vequ(A), A);
    assert_eq!(spice::vminus(A), [-1.0, -2.0, -3.0]);
}

#[test]
#[serial]
fn norms() {
    assert_relative_eq!(spice::vnorm([3.0, 4.0, 0.0]), 5.0, epsilon = f64::EPSILON);
    assert_relative_eq!(spice::vnorm([0.0; 3]), 0.0, epsilon = f64::EPSILON);
    assert_relative_eq!(
        spice::vdist(A, B),
        spice::vnorm(spice::vsub(A, B)),
        epsilon = 1e-14
    );

    // `vrel` is the distance normalised by the larger of the two magnitudes.
    let expected = spice::vdist(A, B) / spice::vnorm(A).max(spice::vnorm(B));
    assert_relative_eq!(spice::vrel(A, B), expected, epsilon = 1e-14);

    assert_slice_eq(&spice::vhat([3.0, 4.0, 0.0]), &[0.6, 0.8, 0.0], 1e-15);
    let (unit, magnitude) = spice::unorm([3.0, 4.0, 0.0]);
    assert_slice_eq(&unit, &[0.6, 0.8, 0.0], 1e-15);
    assert_relative_eq!(magnitude, 5.0, epsilon = f64::EPSILON);
}

#[test]
#[serial]
fn products() {
    assert_relative_eq!(spice::vdot(A, A), 14.0, epsilon = f64::EPSILON);
    assert_relative_eq!(spice::vdot(A, B), 12.0, epsilon = f64::EPSILON);

    // Examples from the CSPICE documentation of vcrss_c.
    assert_eq!(
        spice::vcrss([0.0, 1.0, 0.0], [1.0, 0.0, 0.0]),
        [0.0, 0.0, -1.0]
    );
    assert_eq!(
        spice::vcrss([5.0, 5.0, 5.0], [-1.0, -1.0, -1.0]),
        [0.0, 0.0, 0.0]
    );
    assert_eq!(spice::vcrss(A, B), [27.0, 6.0, -13.0]);

    assert_relative_eq!(
        spice::vsep([1.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        0.0,
        epsilon = f64::EPSILON
    );
    assert_relative_eq!(
        spice::vsep([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
        std::f64::consts::FRAC_PI_2,
        epsilon = f64::EPSILON
    );
    // A zero vector gives a zero separation by definition.
    assert_relative_eq!(spice::vsep(A, [0.0; 3]), 0.0, epsilon = f64::EPSILON);
}

#[test]
#[serial]
fn matrix_vector() {
    // `N` is the rotation by 90 degrees about the third axis.
    assert_slice_eq(&spice::mxv(N, [1.0, 0.0, 0.0]), &[0.0, -1.0, 0.0], 1e-15);
    assert_slice_eq(&spice::mtxv(N, [1.0, 0.0, 0.0]), &[0.0, 1.0, 0.0], 1e-15);

    // The transpose of a rotation undoes it.
    assert_slice_eq(&spice::mtxv(N, spice::mxv(N, A)), &A, 1e-14);
}

#[test]
#[serial]
fn matrix_matrix() {
    let identity = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

    assert_matrix_eq(&spice::mxm(M, identity), &M, 1e-15);
    assert_matrix_eq(&spice::mxmt(N, N), &identity, 1e-15);
    assert_matrix_eq(&spice::mtxm(N, N), &identity, 1e-15);
    assert_matrix_eq(&spice::xpose(spice::xpose(M)), &M, 1e-15);
    assert_eq!(
        spice::xpose(M),
        [[1.0, 4.0, 7.0], [2.0, 5.0, 8.0], [3.0, 6.0, 10.0]]
    );

    assert_matrix_eq(&spice::mxm(M, spice::invert(M)), &identity, 1e-12);
    assert_relative_eq!(spice::det(M), -3.0, epsilon = 1e-12);
    assert_relative_eq!(spice::det(identity), 1.0, epsilon = f64::EPSILON);
    assert_relative_eq!(spice::trace(M), 16.0, epsilon = f64::EPSILON);
}

#[test]
#[serial]
fn rotations() {
    let quarter = std::f64::consts::FRAC_PI_2;

    // A coordinate rotation of 90 degrees about the third axis.
    assert_matrix_eq(&spice::rotate(quarter, 3), &N, 1e-15);

    // Rotating the identity is the same as generating the rotation.
    let identity = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    assert_matrix_eq(&spice::rotmat(identity, quarter, 3), &N, 1e-15);

    // Two quarter turns about the same axis make a half turn.
    assert_matrix_eq(
        &spice::rotmat(spice::rotate(quarter, 1), quarter, 1),
        &spice::rotate(std::f64::consts::PI, 1),
        1e-15,
    );
}

#[test]
#[serial]
fn twovec() {
    // First axis along x, second vector in the x-y plane: the identity.
    let matrix = spice::twovec([1.0, 0.0, 0.0], 1, [0.0, 1.0, 0.0], 2);
    assert_matrix_eq(
        &matrix,
        &[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        1e-15,
    );

    // The rows of the result are orthonormal whatever the inputs.
    let matrix = spice::twovec([1.0, 2.0, 3.0], 1, [0.0, 0.0, 1.0], 3);
    for (index, row) in matrix.iter().enumerate() {
        assert_relative_eq!(spice::vnorm(*row), 1.0, epsilon = 1e-14);
        for other in matrix.iter().skip(index + 1) {
            assert_relative_eq!(spice::vdot(*row, *other), 0.0, epsilon = 1e-14);
        }
    }
    assert_relative_eq!(spice::det(matrix), 1.0, epsilon = 1e-14);
}
