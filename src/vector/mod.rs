use std::ops::{Index, IndexMut};

pub mod vec2;
pub mod vec3;
pub mod vec4;

use crate::matrix::Matrix;

pub type Vector<const M: usize> = Matrix<M, 1>;

// Index and IndexMut -----------------------------------------------------------------------------
impl<const M: usize> Index<usize> for Vector<M> {
    type Output = f32;

    fn index(&self, m: usize) -> &Self::Output {
        &self[(m, 0)]
    }
}

impl<const M: usize> IndexMut<usize> for Vector<M> {
    fn index_mut(&mut self, m: usize) -> &mut Self::Output {
        &mut self[(m, 0)]
    }
}
// Index and IndexMut -----------------------------------------------------------------------------

// From -------------------------------------------------------------------------------------------
impl<const M: usize> From<[f32; M]> for Vector<M> {
    fn from(array: [f32; M]) -> Self {
        Self::from([array])
    }
}

impl<const M: usize> From<Vector<M>> for [f32; M] {
    fn from(value: Vector<M>) -> Self {
        value.column(0)
    }
}
// From -------------------------------------------------------------------------------------------

impl<const M: usize> Vector<M> {
    pub fn dot(&self, other: &Vector<M>) -> f32 {
        let mut result = 0.0;
        for m in 0..M {
            result += self[m] * other[m];
        }
        result
    }

    pub fn magnitude(&self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag.abs() <= f32::EPSILON {
            return Self::zeros();
        }
        self / mag
    }
}

impl Vector<2> {
    pub fn cross(&self, other: &Vector<2>) -> f32 {
        self[0] * other[1] - self[1] * other[0]
    }
}

impl Vector<3> {
    pub fn cross(&self, other: &Vector<3>) -> Vector<3> {
        Vector::from([
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        ])
    }
}
