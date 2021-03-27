use itertools::multizip;
use na::{convert, Matrix1xX, Matrix3xX, RealField};

/// Compute the required size for vector to go from start to end with step.
pub fn size_range_with_step(start: f64, end: f64, step: f64) -> usize {
    let mut size = ((end - start) / step) as usize;
    if start + size as f64 * step < end {
        size += 1;
    }
    size
}

/// Create a vector from `start` to `end` with `step`. The last step might be smaller
/// than `step` just to include `end` in the vector.
pub fn linspace<T>(start: f64, end: f64, step: f64) -> Matrix1xX<T>
where
    T: RealField,
{
    let size = size_range_with_step(start, end, step);
    let mut vector = Matrix1xX::from_fn(size, |i, _| convert(start + step * i as f64));
    if vector[size - 1] > convert(end) {
        vector[size - 1] = convert(end);
    }
    vector
}

/// Dot product vectorized between two list of vectors.
pub fn dot_vectors<T>(vector_list_1: &Matrix3xX<T>, vector_list_2: &Matrix3xX<T>) -> Matrix1xX<T>
where
    T: RealField,
{
    let mut dot_products = Matrix1xX::zeros(vector_list_1.ncols());
    for (dot_product, vector_1, vector_2) in multizip((
        dot_products.iter_mut(),
        vector_list_1.column_iter(),
        vector_list_2.column_iter(),
    )) {
        *dot_product = vector_1.dot(&vector_2);
    }
    dot_products
}
