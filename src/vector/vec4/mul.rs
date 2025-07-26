use std::ops::{Mul, MulAssign};

use crate::{matrix::Matrix, vector::vec4::Vec4};

// Matrix -----------------------------------------------------------------------------------------
impl Mul<&Vec4> for &Matrix<4, 4> {
    type Output = Vec4;

    fn mul(self, rhs: &Vec4) -> Self::Output {
        (self * unsafe { rhs.data.matrix }).into()
    }
}

#[allow(clippy::op_ref)]
impl Mul<&Vec4> for Matrix<4, 4> {
    type Output = Vec4;

    fn mul(self, rhs: &Vec4) -> Self::Output {
        (self * unsafe { rhs.data.matrix }).into()
    }
}

#[allow(clippy::op_ref)]
impl Mul<Vec4> for &Matrix<4, 4> {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        self * &rhs
    }
}

impl Mul<Vec4> for Matrix<4, 4> {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        &self * &rhs
    }
}
// Matrix -----------------------------------------------------------------------------------------

// Scalar -----------------------------------------------------------------------------------------
impl Mul<f32> for &Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: f32) -> Self::Output {
        let matrix = unsafe { self.data.matrix * rhs };
        Vec4::from(matrix)
    }
}

#[allow(clippy::op_ref)]
impl Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: f32) -> Self::Output {
        &self * rhs
    }
}

impl MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, rhs: f32) {
        unsafe { self.data.matrix *= rhs }
    }
}

impl Mul<&Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, rhs: &Vec4) -> Self::Output {
        rhs * self
    }
}

#[allow(clippy::op_ref)]
impl Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        &rhs * self
    }
}
// Scalar -----------------------------------------------------------------------------------------
