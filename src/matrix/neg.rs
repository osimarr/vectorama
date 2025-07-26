use std::ops::Neg;

use crate::matrix::Matrix;

// Matrix -----------------------------------------------------------------------------------------
impl<const M: usize, const N: usize> Neg for &Matrix<M, N> {
    type Output = Matrix<M, N>;

    fn neg(self) -> Self::Output {
        let mut result = Matrix::zeros();
        for m in 0..M {
            for n in 0..N {
                result[(m, n)] = -self[(m, n)]
            }
        }
        result
    }
}

impl<const M: usize, const N: usize> Neg for Matrix<M, N> {
    type Output = Matrix<M, N>;

    fn neg(self) -> Self::Output {
        -&self
    }
}
// Matrix -----------------------------------------------------------------------------------------
