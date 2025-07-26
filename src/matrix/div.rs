use std::ops::{Div, DivAssign};

use crate::matrix::Matrix;

// Scalar -----------------------------------------------------------------------------------------
impl<const M: usize, const N: usize> Div<f32> for &Matrix<M, N> {
    type Output = Matrix<M, N>;

    fn div(self, rhs: f32) -> Self::Output {
        let mut result = Matrix::zeros();
        for m in 0..M {
            for n in 0..N {
                result[(m, n)] = self[(m, n)] / rhs;
            }
        }
        result
    }
}

#[allow(clippy::op_ref)]
impl<const M: usize, const N: usize> Div<f32> for Matrix<M, N> {
    type Output = Matrix<M, N>;

    fn div(self, rhs: f32) -> Self::Output {
        &self / rhs
    }
}

impl<const M: usize, const N: usize> DivAssign<f32> for Matrix<M, N> {
    fn div_assign(&mut self, rhs: f32) {
        for m in 0..M {
            for n in 0..N {
                self[(m, n)] = self[(m, n)] / rhs;
            }
        }
    }
}
// Scalar -----------------------------------------------------------------------------------------
