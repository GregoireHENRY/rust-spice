use itertools::multizip;
use na::{Matrix1xX, Matrix3xX};

#[test]
fn compute_distance() {
    let mut positions = Matrix3xX::from_column_slice(&[
        18.62639796759623,
        21.05444863563425,
        -7.136416860555217,
        18.685576648253853,
        17.77035176622136,
        2.315428951220107,
        18.63486050866997,
        13.806780236054001,
        11.523204797636854,
    ]);
    let size = positions.ncols();
    let expected_distances =
        Matrix1xX::from_column_slice(&[29.00277493974414, 25.89010212033094, 25.897286643055097]);

    let distances = spice::compute_distances(&mut positions);

    for (distance, expected_distance) in multizip((distances.iter(), expected_distances.iter())) {
        assert!(relative_eq!(
            distance,
            expected_distance,
            epsilon = f64::EPSILON
        ));
    }
}
