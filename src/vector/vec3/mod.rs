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

/// A 3D vector with `f32` components.
///
/// `Vec3` provides convenient methods for 3D vector arithmetic, normalization, cross and dot products,
/// and conversion to and from other vector types.
///
/// # Example
/// ```
/// use vectorama::vector::vec3::Vec3;
/// let v = Vec3::new(1.0, 2.0, 3.0);
/// ```
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
    /// Creates a new 3D vector from `x`, `y`, and `z` components.
    ///
    /// # Parameters
    /// - `x`: The x component.
    /// - `y`: The y component.
    /// - `z`: The z component.
    ///
    /// # Returns
    /// A new `Vec3` with the specified components.
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        let vector3 = Vector3 { x, y, z };
        Self {
            data: Vector3Union { vector: vector3 },
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

    /// Returns the `y` and `z` components as a `Vec2`.
    ///
    /// # Returns
    /// A `Vec2` containing the y and z components.
    pub fn yz(&self) -> Vec2 {
        Vec2::new(self.y, self.z)
    }

    /// Returns the `x` and `z` components as a `Vec2`.
    ///
    /// # Returns
    /// A `Vec2` containing the x and z components.
    pub fn xz(&self) -> Vec2 {
        Vec2::new(self.x, self.z)
    }

    /// Creates a `Vec4` from this vector and a given w component.
    ///
    /// # Parameters
    /// - `w`: The w component for the resulting `Vec4`.
    ///
    /// # Returns
    /// A `Vec4` with x, y, z from this vector and the specified w.
    pub fn xyzw(&self, w: f32) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, w)
    }

    /// Computes the cross product with another 3D vector.
    ///
    /// # Parameters
    /// - `other`: The other vector.
    ///
    /// # Returns
    /// The cross product as a `Vec3`.
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        let matrix = unsafe { self.data.matrix.cross(&other.data.matrix) };
        Vec3::from(matrix)
    }

    /// Computes the dot product with another 3D vector.
    ///
    /// # Parameters
    /// - `other`: The other vector.
    ///
    /// # Returns
    /// The dot product as a `f32`.
    pub fn dot(&self, other: &Vec3) -> f32 {
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
