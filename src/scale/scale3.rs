use std::ops::{Deref, DerefMut, Mul, MulAssign};

use crate::{
    matrix::Matrix,
    scale::{from_homogeneous_matrix, scale_vector, to_homogeneous_matrix},
    vector::{Vector, vec3::Vec3},
};

#[derive(Debug, Clone, Copy)]
pub struct Scale3 {
    vector: Vec3,
}

impl Default for Scale3 {
    fn default() -> Self {
        Scale3 {
            vector: Vec3::ones(),
        }
    }
}

impl Scale3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Scale3 {
            vector: Vec3::new(x, y, z),
        }
    }

    pub fn homogeneous_matrix(&self) -> Matrix<4, 4> {
        to_homogeneous_matrix(self.vector)
    }

    pub fn scale(&self, vector: Vec3) -> Vec3 {
        scale_vector(vector, self.vector).into()
    }
}

// From -------------------------------------------------------------------------------------------
impl From<Vec3> for Scale3 {
    fn from(vector: Vec3) -> Self {
        Scale3 { vector }
    }
}

impl From<[f32; 3]> for Scale3 {
    fn from(array: [f32; 3]) -> Self {
        Scale3::from(Vec3::from(array))
    }
}

impl From<Vector<3>> for Scale3 {
    fn from(vector: Vector<3>) -> Self {
        Scale3::from(Vec3::from(vector))
    }
}

impl From<&Matrix<4, 4>> for Scale3 {
    fn from(matrix: &Matrix<4, 4>) -> Self {
        from_homogeneous_matrix(matrix).into()
    }
}

impl From<Matrix<4, 4>> for Scale3 {
    fn from(matrix: Matrix<4, 4>) -> Self {
        Scale3::from(&matrix)
    }
}
// From -------------------------------------------------------------------------------------------

// Deref and DerefMut -----------------------------------------------------------------------------
impl Deref for Scale3 {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.vector
    }
}

impl DerefMut for Scale3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vector
    }
}
// Deref and DerefMut -----------------------------------------------------------------------------

// Mul Vector -------------------------------------------------------------------------------------
impl Mul<&Vec3> for &Scale3 {
    type Output = Vec3;

    fn mul(self, vector: &Vec3) -> Self::Output {
        self.scale(*vector)
    }
}

#[allow(clippy::op_ref)]
impl Mul<Vec3> for &Scale3 {
    type Output = Vec3;

    fn mul(self, vector: Vec3) -> Self::Output {
        self * &vector
    }
}

#[allow(clippy::op_ref)]
impl Mul<&Vec3> for Scale3 {
    type Output = Vec3;
    fn mul(self, vector: &Vec3) -> Self::Output {
        &self * vector
    }
}

impl Mul<Vec3> for Scale3 {
    type Output = Vec3;

    fn mul(self, vector: Vec3) -> Self::Output {
        &self * &vector
    }
}
// Mul Vector -------------------------------------------------------------------------------------

// Mul Scalar -------------------------------------------------------------------------------------
impl Mul<f32> for &Scale3 {
    type Output = Scale3;

    fn mul(self, scalar: f32) -> Self::Output {
        Scale3 {
            vector: self.vector * scalar,
        }
    }
}

#[allow(clippy::op_ref)]
impl Mul<f32> for Scale3 {
    type Output = Scale3;

    fn mul(self, scalar: f32) -> Self::Output {
        &self * scalar
    }
}

impl MulAssign<f32> for Scale3 {
    fn mul_assign(&mut self, scalar: f32) {
        self.vector *= scalar;
    }
}
// Mul Scalar -------------------------------------------------------------------------------------
