use std::ops::{Mul, MulAssign};

use crate::matrix::Matrix;

// Matrix -----------------------------------------------------------------------------------------
impl<const M: usize, const N: usize, const D: usize> Mul<&Matrix<D, N>> for &Matrix<M, D> {
    type Output = Matrix<M, N>;

    fn mul(self, rhs: &Matrix<D, N>) -> Self::Output {
        let mut result = Matrix::zeros();
        for m in 0..M {
            for n in 0..N {
                for d in 0..D {
                    result[(m, n)] += self[(m, d)] * rhs[(d, n)];
                }
            }
        }
        result
    }
}

#[allow(clippy::op_ref)]
impl<const M: usize, const N: usize, const D: usize> Mul<Matrix<D, N>> for &Matrix<M, D> {
    type Output = Matrix<M, N>;

    fn mul(self, rhs: Matrix<D, N>) -> Self::Output {
        self * &rhs
    }
}

#[allow(clippy::op_ref)]
impl<const M: usize, const N: usize, const D: usize> Mul<&Matrix<D, N>> for Matrix<M, D> {
    type Output = Matrix<M, N>;

    fn mul(self, rhs: &Matrix<D, N>) -> Self::Output {
        &self * rhs
    }
}

impl<const M: usize, const N: usize, const D: usize> Mul<Matrix<D, N>> for Matrix<M, D> {
    type Output = Matrix<M, N>;

    fn mul(self, rhs: Matrix<D, N>) -> Self::Output {
        &self * &rhs
    }
}
// Matrix -----------------------------------------------------------------------------------------

// Scalar -----------------------------------------------------------------------------------------
impl<const M: usize, const N: usize> Mul<&Matrix<M, N>> for f32 {
    type Output = Matrix<M, N>;

    fn mul(self, rhs: &Matrix<M, N>) -> Self::Output {
        let mut result = Matrix::zeros();
        for m in 0..M {
            for n in 0..N {
                result[(m, n)] = self * rhs[(m, n)];
            }
        }
        result
    }
}

#[allow(clippy::op_ref)]
impl<const M: usize, const N: usize> Mul<Matrix<M, N>> for f32 {
    type Output = Matrix<M, N>;

    fn mul(self, rhs: Matrix<M, N>) -> Self::Output {
        self * &rhs
    }
}

impl<const M: usize, const N: usize> Mul<f32> for &Matrix<M, N> {
    type Output = Matrix<M, N>;

    fn mul(self, rhs: f32) -> Self::Output {
        rhs * self
    }
}

#[allow(clippy::op_ref)]
impl<const M: usize, const N: usize> Mul<f32> for Matrix<M, N> {
    type Output = Matrix<M, N>;

    fn mul(self, rhs: f32) -> Self::Output {
        rhs * &self
    }
}

impl<const M: usize, const N: usize> MulAssign<f32> for Matrix<M, N> {
    fn mul_assign(&mut self, rhs: f32) {
        for m in 0..M {
            for n in 0..N {
                self[(m, n)] = rhs * self[(m, n)];
            }
        }
    }
}
// Scalar -----------------------------------------------------------------------------------------
