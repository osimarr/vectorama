use std::ops::{Add, AddAssign};

use crate::matrix::Matrix;

// Matrix -----------------------------------------------------------------------------------------
impl<const M: usize, const N: usize> Add<&Matrix<M, N>> for &Matrix<M, N> {
    type Output = Matrix<M, N>;

    fn add(self, rhs: &Matrix<M, N>) -> Self::Output {
        let mut result = Matrix::zeros();
        for m in 0..M {
            for n in 0..N {
                result[(m, n)] = self[(m, n)] + rhs[(m, n)];
            }
        }
        result
    }
}

#[allow(clippy::op_ref)]
impl<const M: usize, const N: usize> Add<Matrix<M, N>> for &Matrix<M, N> {
    type Output = Matrix<M, N>;

    fn add(self, rhs: Matrix<M, N>) -> Self::Output {
        self + &rhs
    }
}

#[allow(clippy::op_ref)]
impl<const M: usize, const N: usize> Add<&Matrix<M, N>> for Matrix<M, N> {
    type Output = Matrix<M, N>;

    fn add(self, rhs: &Matrix<M, N>) -> Self::Output {
        &self + rhs
    }
}

impl<const M: usize, const N: usize> Add<Matrix<M, N>> for Matrix<M, N> {
    type Output = Matrix<M, N>;

    fn add(self, rhs: Matrix<M, N>) -> Self::Output {
        &self + &rhs
    }
}

impl<const M: usize, const N: usize> AddAssign<&Matrix<M, N>> for Matrix<M, N> {
    fn add_assign(&mut self, rhs: &Self) {
        for m in 0..M {
            for n in 0..N {
                self[(m, n)] += rhs[(m, n)];
            }
        }
    }
}

impl<const M: usize, const N: usize> AddAssign<Matrix<M, N>> for Matrix<M, N> {
    fn add_assign(&mut self, rhs: Self) {
        *self += &rhs;
    }
}
// Matrix -----------------------------------------------------------------------------------------
