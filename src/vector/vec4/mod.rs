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

/// A 4D vector with `f32` components.
///
/// `Vec4` provides convenient methods for 4D vector arithmetic, normalization, and conversion to and from
/// other vector types.
///
/// # Example
/// ```
/// use vectorama::vector::vec4::Vec4;
/// let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
/// ```
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
    /// Creates a new 4D vector from `x`, `y`, `z`, and `w` components.
    ///
    /// # Parameters
    /// - `x`: The x component.
    /// - `y`: The y component.
    /// - `z`: The z component.
    /// - `w`: The w component.
    ///
    /// # Returns
    /// A new `Vec4` with the specified components.
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        let vector4 = Vector4 { x, y, z, w };
        Self {
            data: Vector4Union { vector: vector4 },
        }
    }

    /// Returns a vector with all components set to zero.
    ///
    /// # Returns
    /// A zero vector.
    pub fn zeros() -> Self {
        Self::from(Vector::zeros())
    }

    /// Returns a vector with all components set to one.
    ///
    /// # Returns
    /// A vector with all components equal to 1.0.
    pub fn ones() -> Self {
        Self::from(Vector::ones())
    }

    /// Returns the `x` and `y` components as a `Vec2`.
    ///
    /// # Returns
    /// A `Vec2` containing the x and y components.
    pub fn xy(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    /// Returns the `x` and `z` components as a `Vec2`.
    ///
    /// # Returns
    /// A `Vec2` containing the x and z components.
    pub fn xz(&self) -> Vec2 {
        Vec2::new(self.x, self.z)
    }

    /// Returns the `x` and `w` components as a `Vec2`.
    ///
    /// # Returns
    /// A `Vec2` containing the x and w components.
    pub fn xw(&self) -> Vec2 {
        Vec2::new(self.x, self.w)
    }

    /// Returns the `y` and `z` components as a `Vec2`.
    ///
    /// # Returns
    /// A `Vec2` containing the y and z components.
    pub fn yz(&self) -> Vec2 {
        Vec2::new(self.y, self.z)
    }

    /// Returns the `y` and `w` components as a `Vec2`.
    ///
    /// # Returns
    /// A `Vec2` containing the y and w components.
    pub fn yw(&self) -> Vec2 {
        Vec2::new(self.y, self.w)
    }

    /// Returns the `z` and `w` components as a `Vec2`.
    ///
    /// # Returns
    /// A `Vec2` containing the z and w components.
    pub fn zw(&self) -> Vec2 {
        Vec2::new(self.z, self.w)
    }

    /// Returns the `x`, `y`, and `z` components as a `Vec3`.
    ///
    /// # Returns
    /// A `Vec3` containing the x, y, and z components.
    pub fn xyz(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    /// Returns the `x`, `y`, and `w` components as a `Vec3`.
    ///
    /// # Returns
    /// A `Vec3` containing the x, y, and w components.
    pub fn xyw(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.w)
    }

    /// Returns the `x`, `z`, and `w` components as a `Vec3`.
    ///
    /// # Returns
    /// A `Vec3` containing the x, z, and w components.
    pub fn xzw(&self) -> Vec3 {
        Vec3::new(self.x, self.z, self.w)
    }

    /// Returns the `y`, `z`, and `w` components as a `Vec3`.
    ///
    /// # Returns
    /// A `Vec3` containing the y, z, and w components.
    pub fn yzw(&self) -> Vec3 {
        Vec3::new(self.y, self.z, self.w)
    }

    /// Computes the magnitude (length) of the vector.
    ///
    /// # Returns
    /// The magnitude as a `f32`.
    pub fn magnitude(&self) -> f32 {
        unsafe { self.data.matrix.magnitude() }
    }

    /// Returns a normalized (unit length) version of this vector.
    ///
    /// # Returns
    /// The normalized vector.
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
