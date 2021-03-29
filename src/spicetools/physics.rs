use na::{Matrix1xX, Matrix3xX};
use std::vec::Vec;

pub fn compute_distances(positions: &mut Matrix3xX<f64>) -> Matrix1xX<f64> {
    let size = positions.ncols();
    let distances = unsafe {
        let ptr = crate::c::compute_distances(positions.as_mut_ptr(), size as _);
        Vec::from_raw_parts(ptr, size, size)
    };
    Matrix1xX::from_iterator(size, distances.iter().cloned())
}
