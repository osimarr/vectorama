use crate::vector::vec3::Vec3;

pub mod div;
pub mod mul;
pub mod unit;

///
/// A quaternion represented by a vector (imaginary part) and a scalar (real part).
///
/// Quaternions are used to represent rotations in 3D space. This struct stores the quaternion as a
/// vector part (`x`, `y`, `z`) and a scalar part (`w`). All components use `f32` precision.
///
/// **Note:** This type does **not** guarantee the quaternion is normalized (unit length).
/// For guaranteed unit quaternions, use [`UnitQuaternion`].
///
/// # Example
/// ```
/// use vectorama::Quaternion;
/// let q = Quaternion::new([0.0, 1.0, 0.0].into(), 1.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
    pub vector: Vec3,
    pub scalar: f32,
}

impl Default for Quaternion {
    fn default() -> Self {
        Self::identity()
    }
}

impl Quaternion {
    /// Creates a new quaternion from a vector part and a scalar part.
    ///
    /// # Parameters
    /// - `vector`: The vector (imaginary) part of the quaternion.
    /// - `scalar`: The scalar (real) part of the quaternion.
    ///
    /// # Returns
    /// A new quaternion with the specified components.
    pub fn new(vector: Vec3, scalar: f32) -> Self {
        Quaternion { vector, scalar }
    }

    /// Returns the identity quaternion (no rotation).
    ///
    /// # Returns
    /// A quaternion representing no rotation: (0, 0, 0, 1).
    pub fn identity() -> Self {
        Quaternion {
            vector: Vec3::zeros(),
            scalar: 1.0,
        }
    }

    /// Returns the conjugate of the quaternion.
    ///
    /// # Returns
    /// The conjugate, which negates the vector part and keeps the scalar part.
    pub fn conjugate(&self) -> Self {
        Quaternion {
            vector: -self.vector,
            scalar: self.scalar,
        }
    }

    /// Computes the magnitude (norm) of the quaternion.
    ///
    /// # Returns
    /// The magnitude as a `f32`.
    pub fn magnitude(&self) -> f32 {
        (self.vector.magnitude().powi(2) + self.scalar.powi(2)).sqrt()
    }

    /// Computes the inverse of the quaternion.
    ///
    /// # Returns
    /// The inverse quaternion, or the identity if the norm is too small.
    pub fn inverse(&self) -> Self {
        let mag_sq = self.magnitude().powi(2);
        if mag_sq.abs() < f32::EPSILON {
            return Self::identity();
        }
        Quaternion {
            vector: -self.vector / mag_sq,
            scalar: self.scalar / mag_sq,
        }
    }

    /// Returns the normalized (unit) quaternion.
    ///
    /// # Returns
    /// The normalized quaternion, or the identity if the norm is too small.
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag.abs() < f32::EPSILON {
            return Self::identity();
        }
        *self / mag
    }

    /// Computes the dot product with another quaternion.
    ///
    /// # Parameters
    /// - `other`: The other quaternion.
    ///
    /// # Returns
    /// The dot product as a `f32`.
    pub fn dot(&self, other: &Self) -> f32 {
        self.vector.dot(&other.vector) + self.scalar * other.scalar
    }

    /// Creates a quaternion representing a rotation around the X axis.
    ///
    /// # Parameters
    /// - `angle`: The rotation angle in radians.
    ///
    /// # Returns
    /// The quaternion representing the rotation.
    pub fn from_x_axis(angle: f32) -> Self {
        let half_angle = angle / 2.0;
        Quaternion {
            vector: Vec3::new(1.0, 0.0, 0.0) * half_angle.sin(),
            scalar: half_angle.cos(),
        }
    }

    /// Creates a quaternion representing a rotation around the Y axis.
    ///
    /// # Parameters
    /// - `angle`: The rotation angle in radians.
    ///
    /// # Returns
    /// The quaternion representing the rotation.
    pub fn from_y_axis(angle: f32) -> Self {
        let half_angle = angle / 2.0;
        Quaternion {
            vector: Vec3::new(0.0, 1.0, 0.0) * half_angle.sin(),
            scalar: half_angle.cos(),
        }
    }

    /// Creates a quaternion representing a rotation around the Z axis.
    ///
    /// # Parameters
    /// - `angle`: The rotation angle in radians.
    ///
    /// # Returns
    /// The quaternion representing the rotation.
    pub fn from_z_axis(angle: f32) -> Self {
        let half_angle = angle / 2.0;
        Quaternion {
            vector: Vec3::new(0.0, 0.0, 1.0) * half_angle.sin(),
            scalar: half_angle.cos(),
        }
    }

    /// Creates a quaternion from an axis and an angle.
    ///
    /// # Parameters
    /// - `axis`: The axis of rotation (will be normalized).
    /// - `angle`: The rotation angle in radians.
    ///
    /// # Returns
    /// The quaternion representing the rotation.
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        let half_angle = angle / 2.0;
        Quaternion {
            vector: axis.normalize() * half_angle.sin(),
            scalar: half_angle.cos(),
        }
    }

    /// Creates a quaternion from Euler angles using the YXZ order (glTF standard).
    ///
    /// # Parameters
    /// - `x`: Rotation around the X axis (pitch), in radians.
    /// - `y`: Rotation around the Y axis (yaw), in radians.
    /// - `z`: Rotation around the Z axis (roll), in radians.
    ///
    /// # Returns
    /// The quaternion representing the combined rotation.
    pub fn from_euler_angles_yxz(x: f32, y: f32, z: f32) -> Self {
        let qx = Self::from_x_axis(x);
        let qy = Self::from_y_axis(y);
        let qz = Self::from_z_axis(z);

        qy * qx * qz
    }
}

// From -------------------------------------------------------------------------------------------
impl From<Vec3> for Quaternion {
    fn from(vector: Vec3) -> Self {
        Quaternion {
            vector,
            scalar: 0.0,
        }
    }
}

impl From<[f32; 4]> for Quaternion {
    fn from(value: [f32; 4]) -> Self {
        Quaternion {
            vector: Vec3::from([value[0], value[1], value[2]]),
            scalar: value[3],
        }
    }
}
// From -------------------------------------------------------------------------------------------
