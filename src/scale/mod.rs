use crate::{matrix::Matrix, vector::Vector};

pub mod scale2;
pub mod scale3;

pub(super) fn to_homogeneous_matrix<const M: usize, const M1: usize>(
    vector: impl Into<Vector<M>>,
) -> Matrix<M1, M1> {
    assert!(M1 == M + 1, "Matrix size must be M+1 for translation");
    let mut matrix = Matrix::identity();
    let vector = vector.into();
    for m in 0..M {
        matrix[(m, m)] = vector[m];
    }
    matrix
}

pub(super) fn from_homogeneous_matrix<const M: usize, const M1: usize>(
    matrix: &Matrix<M1, M1>,
) -> Vector<M> {
    assert!(M1 == M + 1, "Matrix size must be M+1 for translation");
    let matrix = matrix.view::<M, M>(0, 0);
    let mut vector = Vector::zeros();
    for m in 0..M {
        vector[m] = Vector::<M>::from(matrix.column(m)).magnitude();
    }
    vector
}

pub(super) fn scale_vector<const M: usize>(
    vector: impl Into<Vector<M>>,
    scale: impl Into<Vector<M>>,
) -> Vector<M> {
    let mut result = Vector::zeros();
    let vector = vector.into();
    let scale = scale.into();
    for m in 0..M {
        result[m] = vector[m] * scale[m];
    }
    result
}
