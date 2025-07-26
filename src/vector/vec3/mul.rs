use std::ops::{Mul, MulAssign};

use crate::{matrix::Matrix, vector::vec3::Vec3};

// Matrix -----------------------------------------------------------------------------------------
impl Mul<&Vec3> for &Matrix<3, 3> {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        (self * unsafe { rhs.data.matrix }).into()
    }
}

#[allow(clippy::op_ref)]
impl Mul<&Vec3> for Matrix<3, 3> {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        (self * unsafe { rhs.data.matrix }).into()
    }
}

#[allow(clippy::op_ref)]
impl Mul<Vec3> for &Matrix<3, 3> {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self * &rhs
    }
}

impl Mul<Vec3> for Matrix<3, 3> {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        &self * &rhs
    }
}
// Matrix -----------------------------------------------------------------------------------------

// Scalar -----------------------------------------------------------------------------------------
impl Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        let matrix = unsafe { self.data.matrix * rhs };
        Vec3::from(matrix)
    }
}

#[allow(clippy::op_ref)]
impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        &self * rhs
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        unsafe { self.data.matrix *= rhs }
    }
}

impl Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}

#[allow(clippy::op_ref)]
impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        &rhs * self
    }
}
// Scalar -----------------------------------------------------------------------------------------
