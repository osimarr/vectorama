use crate::vector::vec3::Vec3;

pub mod div;
pub mod mul;
pub mod unit;

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
    pub fn new(vector: Vec3, scalar: f32) -> Self {
        Quaternion { vector, scalar }
    }

    pub fn identity() -> Self {
        Quaternion {
            vector: Vec3::zeros(),
            scalar: 1.0,
        }
    }

    pub fn conjugate(&self) -> Self {
        Quaternion {
            vector: -self.vector,
            scalar: self.scalar,
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.vector.magnitude().powi(2) + self.scalar.powi(2)).sqrt()
    }

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

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag.abs() < f32::EPSILON {
            return Self::identity();
        }
        *self / mag
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.vector.dot(&other.vector) + self.scalar * other.scalar
    }

    pub fn from_x_axis(angle: f32) -> Self {
        let half_angle = angle / 2.0;
        Quaternion {
            vector: Vec3::new(1.0, 0.0, 0.0) * half_angle.sin(),
            scalar: half_angle.cos(),
        }
    }

    pub fn from_y_axis(angle: f32) -> Self {
        let half_angle = angle / 2.0;
        Quaternion {
            vector: Vec3::new(0.0, 1.0, 0.0) * half_angle.sin(),
            scalar: half_angle.cos(),
        }
    }

    pub fn from_z_axis(angle: f32) -> Self {
        let half_angle = angle / 2.0;
        Quaternion {
            vector: Vec3::new(0.0, 0.0, 1.0) * half_angle.sin(),
            scalar: half_angle.cos(),
        }
    }

    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        let half_angle = angle / 2.0;
        Quaternion {
            vector: axis.normalize() * half_angle.sin(),
            scalar: half_angle.cos(),
        }
    }

    // Using glTF's YXZ order for Euler angles
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
