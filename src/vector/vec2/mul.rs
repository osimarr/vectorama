use std::ops::{Mul, MulAssign};

use crate::{matrix::Matrix, vector::vec2::Vec2};

// Matrix -----------------------------------------------------------------------------------------
impl Mul<&Vec2> for &Matrix<2, 2> {
    type Output = Vec2;

    fn mul(self, rhs: &Vec2) -> Self::Output {
        (self * unsafe { rhs.data.matrix }).into()
    }
}

#[allow(clippy::op_ref)]
impl Mul<&Vec2> for Matrix<2, 2> {
    type Output = Vec2;

    fn mul(self, rhs: &Vec2) -> Self::Output {
        &self * rhs
    }
}

#[allow(clippy::op_ref)]
impl Mul<Vec2> for &Matrix<2, 2> {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        self * &rhs
    }
}

impl Mul<Vec2> for Matrix<2, 2> {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        self * &rhs
    }
}
// Matrix -----------------------------------------------------------------------------------------

// Scalar -----------------------------------------------------------------------------------------
impl Mul<f32> for &Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        let matrix = unsafe { self.data.matrix * rhs };
        Vec2::from(matrix)
    }
}

#[allow(clippy::op_ref)]
impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        &self * rhs
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        unsafe { self.data.matrix *= rhs }
    }
}

impl Mul<&Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: &Vec2) -> Self::Output {
        rhs * self
    }
}

#[allow(clippy::op_ref)]
impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        &rhs * self
    }
}
// Scalar -----------------------------------------------------------------------------------------
