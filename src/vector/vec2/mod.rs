use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::vector::{Vector, vec3::Vec3};

pub mod add;
pub mod div;
pub mod mul;
pub mod neg;
pub mod sub;

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy)]
pub union Vector2Union {
    matrix: Vector<2>,
    vector: Vector2,
}

impl Default for Vector2Union {
    fn default() -> Self {
        Self {
            vector: Default::default(),
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Vec2 {
    data: Vector2Union,
}

impl Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vec2")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        let vector2 = Vector2 { x, y };
        Self {
            data: Vector2Union { vector: vector2 },
        }
    }

    pub fn zeros() -> Self {
        Self::from(Vector::zeros())
    }

    pub fn ones() -> Self {
        Self::from(Vector::ones())
    }

    pub fn xyz(&self, z: f32) -> Vec3 {
        Vec3::new(self.x, self.y, z)
    }

    pub fn dot(&self, other: &Vec2) -> f32 {
        unsafe { self.data.matrix.dot(&other.data.matrix) }
    }

    pub fn magnitude(&self) -> f32 {
        unsafe { self.data.matrix.magnitude() }
    }

    pub fn normalize(&self) -> Vec2 {
        let matrix = unsafe { self.data.matrix.normalize() };
        Vec2::from(matrix)
    }
}

// Deref and DerefMut -----------------------------------------------------------------------------
impl Deref for Vec2 {
    type Target = Vector2;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.data.vector }
    }
}

impl DerefMut for Vec2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.data.vector }
    }
}
// Deref and DerefMut -----------------------------------------------------------------------------

// From -------------------------------------------------------------------------------------------
impl From<Vector<2>> for Vec2 {
    fn from(value: Vector<2>) -> Self {
        Self {
            data: Vector2Union { matrix: value },
        }
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from(value: [f32; 2]) -> Self {
        Self::from(Vector::from(value))
    }
}

impl From<Vec2> for Vector<2> {
    fn from(value: Vec2) -> Self {
        unsafe { value.data.matrix }
    }
}
// From -------------------------------------------------------------------------------------------

// AsRef ------------------------------------------------------------------------------------------
impl AsRef<[f32]> for Vec2 {
    fn as_ref(&self) -> &[f32] {
        unsafe { self.data.matrix.as_ref() }
    }
}
// AsRef ------------------------------------------------------------------------------------------
