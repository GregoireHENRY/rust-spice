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

#[test]
#[serial]
fn packing() {
    assert_eq!(spice::vpack(1.0, 2.0, 3.0), A);
    assert_eq!(spice::vupack(A), (1.0, 2.0, 3.0));

    assert!(spice::vzero([0.0; 3]));
    assert!(!spice::vzero(A));
}

#[test]
#[serial]
fn projections() {
    // Projecting onto an axis keeps only that component; the perpendicular part keeps the rest.
    assert_slice_eq(&spice::vproj(A, [1.0, 0.0, 0.0]), &[1.0, 0.0, 0.0], 1e-15);
    assert_slice_eq(&spice::vperp(A, [1.0, 0.0, 0.0]), &[0.0, 2.0, 3.0], 1e-15);

    // The two parts add back up, and are orthogonal.
    let (along, across) = (spice::vproj(A, B), spice::vperp(A, B));
    assert_slice_eq(&spice::vadd(along, across), &A, 1e-14);
    assert_relative_eq!(spice::vdot(along, across), 0.0, epsilon = 1e-14);
}

#[test]
#[serial]
fn combinations() {
    assert_slice_eq(&spice::vlcom(2.0, A, 3.0, B), &[14.0, -11.0, 24.0], 1e-14);
    assert_slice_eq(
        &spice::vlcom3(1.0, A, 1.0, B, 1.0, [1.0, 1.0, 1.0]),
        &[6.0, -2.0, 10.0],
        1e-14,
    );

    // `vlcom` agrees with scaling and adding by hand.
    assert_slice_eq(
        &spice::vlcom(2.0, A, 3.0, B),
        &spice::vadd(spice::vscl(2.0, A), spice::vscl(3.0, B)),
        1e-14,
    );
}

#[test]
#[serial]
fn rotating_a_vector() {
    let quarter = std::f64::consts::FRAC_PI_2;

    // A quarter turn about +z takes +x to +y.
    assert_slice_eq(
        &spice::vrotv([1.0, 0.0, 0.0], [0.0, 0.0, 1.0], quarter),
        &[0.0, 1.0, 0.0],
        1e-15,
    );
    // A vector along the axis is unchanged.
    assert_slice_eq(
        &spice::vrotv([0.0, 0.0, 5.0], [0.0, 0.0, 1.0], quarter),
        &[0.0, 0.0, 5.0],
        1e-15,
    );

    // It agrees with building the matrix and applying it.
    assert_slice_eq(
        &spice::vrotv(A, [1.0, 1.0, 1.0], 0.7),
        &spice::mxv(spice::axisar([1.0, 1.0, 1.0], 0.7), A),
        1e-14,
    );
}

#[test]
#[serial]
fn quadratic_form_and_identity() {
    let identity = spice::ident();
    assert_matrix_eq(
        &identity,
        &[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        0.0,
    );
    assert_matrix_eq(&spice::mequ(M), &M, 0.0);
    assert_matrix_eq(&spice::mxm(M, identity), &M, 0.0);

    // With the identity in the middle, the quadratic form is just the dot product.
    assert_relative_eq!(
        spice::vtmv(A, identity, B),
        spice::vdot(A, B),
        epsilon = 1e-14
    );
    // The quadratic form is the dot product of `A` with `M * B`.
    assert_relative_eq!(
        spice::vtmv(A, M, B),
        spice::vdot(A, spice::mxv(M, B)),
        epsilon = 1e-12
    );
    assert_relative_eq!(spice::vtmv(A, M, B), 210.0, epsilon = 1e-12);

    // Transposing a 6x6 twice is the identity on it.
    let mut big = [[0.0; 6]; 6];
    for (row, line) in big.iter_mut().enumerate() {
        for (col, cell) in line.iter_mut().enumerate() {
            *cell = (row * 6 + col) as f64;
        }
    }
    assert_eq!(spice::xpose6(spice::xpose6(big)), big);
    assert_eq!(spice::xpose6(big)[1][4], big[4][1]);
}

/// A four dimensional pair, so the general routines cannot silently be doing 3-vector work.
const G1: [f64; 4] = [1.0, 2.0, 3.0, 4.0];
const G2: [f64; 4] = [-2.0, 1.0, 0.0, 5.0];

#[test]
#[serial]
fn general_dimension_vectors() {
    crate::common::reset();

    assert_eq!(spice::vaddg(&G1, &G2), vec![-1.0, 3.0, 3.0, 9.0]);
    assert_eq!(spice::vsubg(&G1, &G2), vec![3.0, 1.0, 3.0, -1.0]);
    assert_eq!(spice::vsclg(2.0, &G1), vec![2.0, 4.0, 6.0, 8.0]);
    assert_eq!(spice::vequg(&G1), G1.to_vec());
    assert_eq!(spice::vminug(&G1), vec![-1.0, -2.0, -3.0, -4.0]);
    assert_eq!(spice::moved(&G1), G1.to_vec());

    assert_relative_eq!(spice::vdotg(&G1, &G2), 20.0, epsilon = 1e-14);
    assert_relative_eq!(spice::vnormg(&G1), 30f64.sqrt(), epsilon = 1e-14);
    assert_relative_eq!(
        spice::vdistg(&G1, &G2),
        spice::vnormg(&spice::vsubg(&G1, &G2)),
        epsilon = 1e-14
    );
    assert!(spice::vzerog(&[0.0; 4]));
    assert!(!spice::vzerog(&G1));

    // The unit vector and the magnitude, both ways of asking.
    let (unit, magnitude) = spice::unormg(&G1);
    assert_relative_eq!(magnitude, spice::vnormg(&G1), epsilon = 1e-14);
    assert_eq!(unit, spice::vhatg(&G1));
    assert_relative_eq!(spice::vnormg(&unit), 1.0, epsilon = 1e-14);

    // Separation and relative difference, checked against their definitions.
    let cosine = spice::vdotg(&G1, &G2) / (spice::vnormg(&G1) * spice::vnormg(&G2));
    assert_relative_eq!(spice::vsepg(&G1, &G2), cosine.acos(), epsilon = 1e-12);
    assert_relative_eq!(
        spice::vrelg(&G1, &G2),
        spice::vdistg(&G1, &G2) / spice::vnormg(&G1).max(spice::vnormg(&G2)),
        epsilon = 1e-14
    );

    // Projection, and the linear combination.
    let along = spice::vprojg(&G1, &G2);
    let across = spice::vsubg(&G1, &along);
    assert_relative_eq!(spice::vdotg(&along, &across), 0.0, epsilon = 1e-13);
    assert_eq!(
        spice::vlcomg(2.0, &G1, 3.0, &G2),
        spice::vaddg(&spice::vsclg(2.0, &G1), &spice::vsclg(3.0, &G2))
    );

    // In three dimensions they agree with the fixed size forms.
    assert_eq!(spice::vaddg(&A, &B), spice::vadd(A, B).to_vec());
    assert_relative_eq!(spice::vdotg(&A, &B), spice::vdot(A, B), epsilon = 0.0);
    assert_relative_eq!(spice::vsepg(&A, &B), spice::vsep(A, B), epsilon = 1e-15);

    crate::common::assert_ok("the general dimension vector routines");
}

#[test]
#[serial]
#[should_panic(expected = "must all have the same length")]
fn general_dimension_vectors_reject_a_length_mismatch() {
    crate::common::reset();
    spice::vaddg(&G1, &A);
}

#[test]
#[serial]
fn general_dimension_matrices() {
    crate::common::reset();

    // A 2x3 matrix and a 3x2 one, stored row by row.
    let a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let b = [7.0, 8.0, 9.0, 10.0, 11.0, 12.0];

    // (2x3)(3x2) = 2x2.
    assert_eq!(spice::mxmg(&a, &b, 2, 3, 2), vec![58.0, 64.0, 139.0, 154.0]);
    // The transpose of the 2x3, times the 2x2 identity, is 3x2. The dimensions are named after
    // what they mean, not after their position: `nc1`, then the shared row count, then `nc2`.
    let square = [1.0, 0.0, 0.0, 1.0];
    assert_eq!(
        spice::mtxmg(&a, &square, 3, 2, 2),
        vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0]
    );
    // (2x3)(2x3)^T = 2x2.
    assert_eq!(spice::mxmtg(&a, &a, 2, 3, 2), vec![14.0, 32.0, 32.0, 77.0]);

    // Matrix times vector, and the transpose form.
    assert_eq!(spice::mxvg(&a, &[1.0, 1.0, 1.0], 2, 3), vec![6.0, 15.0]);
    assert_eq!(spice::mtxvg(&a, &[1.0, 1.0], 3, 2), vec![5.0, 7.0, 9.0]);

    // Transposing and copying.
    assert_eq!(spice::xposeg(&a, 2, 3), vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
    assert_eq!(spice::xposeg(&spice::xposeg(&a, 2, 3), 3, 2), a.to_vec());
    assert_eq!(spice::mequg(&a, 2, 3), a.to_vec());

    // The quadratic form, against the same computation done in two steps.
    let (v1, v2) = ([1.0, 2.0], [3.0, 4.0, 5.0]);
    let expected = spice::vdotg(&v1, &spice::mxvg(&a, &v2, 2, 3));
    assert_relative_eq!(spice::vtmvg(&v1, &a, &v2, 2, 3), expected, epsilon = 1e-13);

    // In three dimensions they agree with the fixed size forms.
    let flat = M.iter().flatten().copied().collect::<Vec<f64>>();
    assert_eq!(spice::mxvg(&flat, &A, 3, 3), spice::mxv(M, A).to_vec());
    assert_eq!(
        spice::xposeg(&flat, 3, 3),
        spice::xpose(M)
            .iter()
            .flatten()
            .copied()
            .collect::<Vec<f64>>()
    );

    crate::common::assert_ok("the general dimension matrix routines");
}

#[test]
#[serial]
#[should_panic(expected = "needs 6 elements")]
fn general_dimension_matrices_reject_a_wrong_shape() {
    crate::common::reset();
    spice::xposeg(&[1.0, 2.0, 3.0], 2, 3);
}
