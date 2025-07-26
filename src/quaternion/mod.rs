use crate::vector::vec3::Vec3;

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

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag.abs() < f32::EPSILON {
            return Self::identity();
        }
        Quaternion {
            vector: self.vector / mag,
            scalar: self.scalar / mag,
        }
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
    pub fn from_euler_angles(x: f32, y: f32, z: f32) -> Self {
        let qx = Self::from_x_axis(x);
        let qy = Self::from_y_axis(y);
        let qz = Self::from_z_axis(z);
        // The correct order for a YXZ rotation is qy * qx * qz
        qz * qy * qx
    }

    pub fn rotate_vector(&self, vector: Vec3) -> Vec3 {
        let q_vector = Quaternion::from(vector);
        let q_conjugate = self.conjugate();
        let rotated = self * q_vector * q_conjugate;
        rotated.vector
    }

    pub fn to_axis_angle(&self) -> (Vec3, f32) {
        let angle = 2.0 * self.scalar.acos();
        let sin_half_angle = (1.0 - self.scalar.powi(2)).sqrt();
        if sin_half_angle < f32::EPSILON {
            return (Vec3::new(1.0, 0.0, 0.0), angle);
        }
        let axis = self.vector / sin_half_angle;
        (axis, angle)
    }

    pub fn to_euler_angles(&self) -> Vec3 {
        let qx = self.vector.x;
        let qy = self.vector.y;
        let qz = self.vector.z;
        let qw = self.scalar;

        // roll (x-axis rotation)
        let sinr_cosp = 2.0 * (qw * qx + qy * qz);
        let cosr_cosp = 1.0 - 2.0 * (qx * qx + qy * qy);
        let roll = sinr_cosp.atan2(cosr_cosp);

        // pitch (y-axis rotation)
        let sinp = 2.0 * (qw * qy - qz * qx);
        let pitch = if sinp.abs() >= 1.0 {
            sinp.signum() * std::f32::consts::FRAC_PI_2 // use 90 degrees if out of range
        } else {
            sinp.asin()
        };

        // yaw (z-axis rotation)
        let siny_cosp = 2.0 * (qw * qz + qx * qy);
        let cosy_cosp = 1.0 - 2.0 * (qy * qy + qz * qz);
        let yaw = siny_cosp.atan2(cosy_cosp);

        Vec3::new(roll, pitch, yaw)
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
