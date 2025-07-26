use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::vector::{Vector, vec2::Vec2, vec3::Vec3};

pub mod add;
pub mod div;
pub mod mul;
pub mod neg;
pub mod sub;

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Clone, Copy)]
pub union Vector4Union {
    matrix: Vector<4>,
    vector: Vector4,
}

impl Default for Vector4Union {
    fn default() -> Self {
        Self {
            vector: Default::default(),
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Vec4 {
    data: Vector4Union,
}

impl Debug for Vec4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vec4")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .field("w", &self.w)
            .finish()
    }
}

impl Vec4 {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        let vector4 = Vector4 { x, y, z, w };
        Self {
            data: Vector4Union { vector: vector4 },
        }
    }

    pub fn zeros() -> Self {
        Self::from(Vector::zeros())
    }

    pub fn ones() -> Self {
        Self::from(Vector::ones())
    }

    pub fn xy(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn xz(&self) -> Vec2 {
        Vec2::new(self.x, self.z)
    }

    pub fn xw(&self) -> Vec2 {
        Vec2::new(self.x, self.w)
    }

    pub fn yz(&self) -> Vec2 {
        Vec2::new(self.y, self.z)
    }

    pub fn yw(&self) -> Vec2 {
        Vec2::new(self.y, self.w)
    }

    pub fn zw(&self) -> Vec2 {
        Vec2::new(self.z, self.w)
    }

    pub fn xyz(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    pub fn xyw(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.w)
    }

    pub fn xzw(&self) -> Vec3 {
        Vec3::new(self.x, self.z, self.w)
    }

    pub fn yzw(&self) -> Vec3 {
        Vec3::new(self.y, self.z, self.w)
    }

    pub fn magnitude(&self) -> f32 {
        unsafe { self.data.matrix.magnitude() }
    }

    pub fn normalize(&self) -> Vec4 {
        let matrix = unsafe { self.data.matrix.normalize() };
        Vec4::from(matrix)
    }
}

// Deref and DerefMut -----------------------------------------------------------------------------
impl Deref for Vec4 {
    type Target = Vector4;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.data.vector }
    }
}

impl DerefMut for Vec4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.data.vector }
    }
}
// Deref and DerefMut -----------------------------------------------------------------------------

// From -------------------------------------------------------------------------------------------
impl From<Vector<4>> for Vec4 {
    fn from(value: Vector<4>) -> Self {
        Self {
            data: Vector4Union { matrix: value },
        }
    }
}

impl From<[f32; 4]> for Vec4 {
    fn from(value: [f32; 4]) -> Self {
        Self::from(Vector::from(value))
    }
}

impl From<Vec4> for Vector<4> {
    fn from(value: Vec4) -> Self {
        unsafe { value.data.matrix }
    }
}
// From -------------------------------------------------------------------------------------------

// AsRef ------------------------------------------------------------------------------------------
impl AsRef<[f32]> for Vec4 {
    fn as_ref(&self) -> &[f32] {
        unsafe { self.data.matrix.as_ref() }
    }
}
// AsRef ------------------------------------------------------------------------------------------
