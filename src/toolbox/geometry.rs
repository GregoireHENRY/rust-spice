use itertools::multizip;
use na::{Matrix1xX, Matrix3xX, RealField};

/// Distance between two list of vectors 3xX.
pub fn distances<T>(vectors: Matrix3xX<T>) -> Matrix1xX<T>
where
    T: RealField,
{
    let number_vectors = vectors.ncols();
    let mut distances = Matrix1xX::zeros(number_vectors);
    for (distance, vector) in multizip((distances.iter_mut(), vectors.column_iter())) {
        *distance = (vector).norm();
    }
    distances
}

/// Directions from the origin to a list of points 3xX.
pub fn directions<T>(vectors: Matrix3xX<T>) -> Matrix3xX<T>
where
    T: RealField,
{
    let number_vectors = vectors.ncols();
    let mut directions = Matrix3xX::zeros(number_vectors);
    for (mut direction, vector) in multizip((directions.column_iter_mut(), vectors.column_iter())) {
        direction.copy_from(&(vector).normalize());
    }
    directions
}
