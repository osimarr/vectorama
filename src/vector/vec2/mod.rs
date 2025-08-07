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

/// A 2D vector with `f32` components.
///
/// `Vec2` provides convenient methods for 2D vector arithmetic, normalization, and conversion.
///
/// # Example
/// ```
/// use vectorama::Vec2;
/// let v = Vec2::new(1.0, 2.0);
/// ```
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
    /// Creates a new 2D vector from `x` and `y` components.
    ///
    /// # Parameters
    /// - `x`: The x component.
    /// - `y`: The y component.
    ///
    /// # Returns
    /// A new `Vec2` with the specified components.
    pub const fn new(x: f32, y: f32) -> Self {
        let vector2 = Vector2 { x, y };
        Self {
            data: Vector2Union { vector: vector2 },
        }
    }

    /// Returns a vector with both components set to zero.
    ///
    /// # Returns
    /// A zero vector.
    pub fn zeros() -> Self {
        Self::from(Vector::zeros())
    }

    /// Returns a vector with both components set to one.
    ///
    /// # Returns
    /// A vector with all components equal to 1.0.
    pub fn ones() -> Self {
        Self::from(Vector::ones())
    }

    /// Creates a 3D vector from this 2D vector and a given z component.
    ///
    /// # Parameters
    /// - `z`: The z component for the resulting `Vec3`.
    ///
    /// # Returns
    /// A `Vec3` with x, y from this vector and the specified z.
    pub fn xyz(&self, z: f32) -> Vec3 {
        Vec3::new(self.x, self.y, z)
    }

    /// Computes the dot product with another 2D vector.
    ///
    /// # Parameters
    /// - `other`: The other vector.
    ///
    /// # Returns
    /// The dot product as a `f32`.
    pub fn dot(&self, other: &Vec2) -> f32 {
        unsafe { self.data.matrix.dot(&other.data.matrix) }
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
