use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use crate::vector::{Vector, vec2::Vec2, vec4::Vec4};

pub mod add;
pub mod div;
pub mod mul;
pub mod neg;
pub mod sub;

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy)]
pub union Vector3Union {
    matrix: Vector<3>,
    vector: Vector3,
}

impl Default for Vector3Union {
    fn default() -> Self {
        Self {
            vector: Default::default(),
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Vec3 {
    data: Vector3Union,
}

impl Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vec3")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        let vector3 = Vector3 { x, y, z };
        Self {
            data: Vector3Union { vector: vector3 },
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

    pub fn yz(&self) -> Vec2 {
        Vec2::new(self.y, self.z)
    }

    pub fn xz(&self) -> Vec2 {
        Vec2::new(self.x, self.z)
    }

    pub fn xyzw(&self, w: f32) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, w)
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        let matrix = unsafe { self.data.matrix.cross(&other.data.matrix) };
        Vec3::from(matrix)
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        unsafe { self.data.matrix.dot(&other.data.matrix) }
    }

    pub fn magnitude(&self) -> f32 {
        unsafe { self.data.matrix.magnitude() }
    }

    pub fn normalize(&self) -> Vec3 {
        let matrix = unsafe { self.data.matrix.normalize() };
        Vec3::from(matrix)
    }
}

// Deref and DerefMut -----------------------------------------------------------------------------
impl Deref for Vec3 {
    type Target = Vector3;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.data.vector }
    }
}

impl DerefMut for Vec3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.data.vector }
    }
}
// Deref and DerefMut -----------------------------------------------------------------------------

// From -------------------------------------------------------------------------------------------
impl From<Vector<3>> for Vec3 {
    fn from(value: Vector<3>) -> Self {
        Self {
            data: Vector3Union { matrix: value },
        }
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(value: [f32; 3]) -> Self {
        Self::from(Vector::from(value))
    }
}

impl From<Vec3> for Vector<3> {
    fn from(value: Vec3) -> Self {
        unsafe { value.data.matrix }
    }
}
// From -------------------------------------------------------------------------------------------

// AsRef ------------------------------------------------------------------------------------------
impl AsRef<[f32]> for Vec3 {
    fn as_ref(&self) -> &[f32] {
        unsafe { self.data.matrix.as_ref() }
    }
}
// AsRef ------------------------------------------------------------------------------------------
