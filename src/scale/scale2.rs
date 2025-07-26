use std::ops::{Deref, DerefMut, Mul, MulAssign};

use crate::{
    matrix::Matrix,
    scale::{from_homogeneous_matrix, scale_vector, to_homogeneous_matrix},
    vector::{Vector, vec2::Vec2},
};

#[derive(Debug, Clone, Copy)]
pub struct Scale2 {
    vector: Vec2,
}

impl Default for Scale2 {
    fn default() -> Self {
        Scale2 {
            vector: Vec2::ones(),
        }
    }
}

impl Scale2 {
    pub fn new(x: f32, y: f32) -> Self {
        Scale2 {
            vector: Vec2::new(x, y),
        }
    }

    pub fn homogeneous_matrix(&self) -> Matrix<3, 3> {
        to_homogeneous_matrix(self.vector)
    }

    pub fn scale(&self, vector: Vec2) -> Vec2 {
        scale_vector(vector, self.vector).into()
    }
}

// From -------------------------------------------------------------------------------------------
impl From<Vec2> for Scale2 {
    fn from(vector: Vec2) -> Self {
        Scale2 { vector }
    }
}

impl From<[f32; 2]> for Scale2 {
    fn from(array: [f32; 2]) -> Self {
        Scale2::from(Vec2::from(array))
    }
}

impl From<Vector<2>> for Scale2 {
    fn from(vector: Vector<2>) -> Self {
        Scale2::from(Vec2::from(vector))
    }
}

impl From<&Matrix<3, 3>> for Scale2 {
    fn from(matrix: &Matrix<3, 3>) -> Self {
        from_homogeneous_matrix(matrix).into()
    }
}

impl From<Matrix<3, 3>> for Scale2 {
    fn from(matrix: Matrix<3, 3>) -> Self {
        Scale2::from(&matrix)
    }
}
// From -------------------------------------------------------------------------------------------

// Deref and DerefMut -----------------------------------------------------------------------------
impl Deref for Scale2 {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.vector
    }
}

impl DerefMut for Scale2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vector
    }
}
// Deref and DerefMut -----------------------------------------------------------------------------

// Mul Vector -------------------------------------------------------------------------------------
impl Mul<&Vec2> for &Scale2 {
    type Output = Vec2;

    fn mul(self, vector: &Vec2) -> Self::Output {
        self.scale(*vector)
    }
}

#[allow(clippy::op_ref)]
impl Mul<Vec2> for &Scale2 {
    type Output = Vec2;

    fn mul(self, vector: Vec2) -> Self::Output {
        self * &vector
    }
}

#[allow(clippy::op_ref)]
impl Mul<&Vec2> for Scale2 {
    type Output = Vec2;
    fn mul(self, vector: &Vec2) -> Self::Output {
        &self * vector
    }
}

impl Mul<Vec2> for Scale2 {
    type Output = Vec2;

    fn mul(self, vector: Vec2) -> Self::Output {
        &self * &vector
    }
}
// Mul Vector -------------------------------------------------------------------------------------

// Mul Scalar -------------------------------------------------------------------------------------
impl Mul<f32> for &Scale2 {
    type Output = Scale2;

    fn mul(self, scalar: f32) -> Self::Output {
        Scale2 {
            vector: self.vector * scalar,
        }
    }
}

#[allow(clippy::op_ref)]
impl Mul<f32> for Scale2 {
    type Output = Scale2;

    fn mul(self, scalar: f32) -> Self::Output {
        &self * scalar
    }
}

impl MulAssign<f32> for Scale2 {
    fn mul_assign(&mut self, scalar: f32) {
        self.vector *= scalar;
    }
}
// Mul Scalar -------------------------------------------------------------------------------------
