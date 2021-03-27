use itertools::multizip;
use na::{Matrix1xX, Matrix3xX};
use spice::{directions, distances};

#[test]
fn compute_distances() {
    let vectors = Matrix3xX::from_column_slice(&[1.0, 0.0, 0.0, 0.0, 1.0, 1.0]);
    let expected_distances = Matrix1xX::from_column_slice(&[1.0, 2.0f64.sqrt()]);

    let distances = distances(vectors);

    for (distance, expected_distance) in multizip((distances.iter(), expected_distances.iter())) {
        assert!(relative_eq!(
            distance,
            expected_distance,
            epsilon = f64::EPSILON
        ));
    }
}

#[test]
fn compute_directions() {
    let vectors = Matrix3xX::from_column_slice(&[1.0, 0.0, 0.0, 0.0, 1.0, 1.0]);
    let expected_directions = Matrix1xX::from_column_slice(&[
        1.0,
        0.0,
        0.0,
        0.0,
        1.0 / 2.0f64.sqrt(),
        1.0 / 2.0f64.sqrt(),
    ]);

    let directions = directions(vectors);

    for (direction_projection, expected_direction_projection) in
        multizip((directions.iter(), expected_directions.iter()))
    {
        assert!(relative_eq!(
            direction_projection,
            expected_direction_projection,
            epsilon = f64::EPSILON
        ));
    }
}
