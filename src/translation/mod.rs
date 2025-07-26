use crate::{matrix::Matrix, vector::Vector};

pub mod translation2;
pub mod translation3;

pub(super) fn to_homogeneous_matrix<const M: usize, const M1: usize>(
    vector: impl Into<Vector<M>>,
) -> Matrix<M1, M1> {
    assert!(M1 == M + 1, "Matrix size must be M+1 for translation");
    let mut matrix = Matrix::identity();
    let vector = vector.into();
    for m in 0..M {
        matrix[(m, M)] = vector[m];
    }
    matrix
}

pub(super) fn from_homogeneous_matrix<const M: usize, const M1: usize>(
    matrix: &Matrix<M1, M1>,
) -> Vector<M> {
    assert!(M1 == M + 1, "Matrix size must be M+1 for translation");
    let mut vector = Vector::zeros();
    for m in 0..M {
        vector[m] = matrix[(m, M)];
    }
    vector
}
