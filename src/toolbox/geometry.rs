use itertools::multizip;
use na::{Matrix1xX, Matrix3xX, RealField};

/// Distance between two list of vectors.
pub fn distances<T>(vectors_1: Matrix3xX<T>, vectors_2: Matrix3xX<T>) -> Matrix1xX<T>
where
    T: RealField,
{
    let number_vectors = vectors_1.ncols();
    let mut distances = Matrix1xX::zeros(number_vectors);
    for (distance, vector_1, vector_2) in multizip((
        distances.iter_mut(),
        vectors_1.column_iter(),
        vectors_2.column_iter(),
    )) {
        *distance = (vector_2 - vector_1).norm();
    }
    distances
}

/// Directions from a list of points to one other.
pub fn directions<T>(vectors_1: Matrix3xX<T>, vectors_2: Matrix3xX<T>) -> Matrix3xX<T>
where
    T: RealField,
{
    let number_vectors = vectors_1.ncols();
    let mut directions = Matrix3xX::zeros(number_vectors);
    for (mut direction, vector_1, vector_2) in multizip((
        directions.column_iter_mut(),
        vectors_1.column_iter(),
        vectors_2.column_iter(),
    )) {
        direction.copy_from(&(vector_2 - vector_1).normalize());
    }
    directions
}
