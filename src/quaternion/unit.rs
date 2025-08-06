use std::ops::Deref;

use crate::{matrix::Matrix, quaternion::Quaternion, vector::vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct UnitQuaternion {
    quat: Quaternion,
}

impl Default for UnitQuaternion {
    fn default() -> Self {
        Self::identity()
    }
}

impl UnitQuaternion {
    pub fn new_normalized(vector: Vec3, scalar: f32) -> Self {
        let quat = Quaternion { vector, scalar }.normalize();
        Self { quat }
    }

    pub fn identity() -> Self {
        Self {
            quat: Quaternion::identity(),
        }
    }

    pub fn conjugate(&self) -> Self {
        self.quat.conjugate().into()
    }

    pub fn inverse(&self) -> Self {
        self.conjugate()
    }

    pub fn from_x_axis(angle: f32) -> Self {
        Quaternion::from_x_axis(angle).into()
    }

    pub fn from_y_axis(angle: f32) -> Self {
        Quaternion::from_y_axis(angle).into()
    }

    pub fn from_z_axis(angle: f32) -> Self {
        Quaternion::from_z_axis(angle).into()
    }

    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        Quaternion::from_axis_angle(axis, angle).into()
    }

    pub fn from_euler_angles(x: f32, y: f32, z: f32) -> Self {
        Quaternion::from_euler_angles_yxz(x, y, z).into()
    }

    pub fn from_rotation_matrix(matrix: &Matrix<3, 3>) -> Self {
        // Assumes matrix is a valid rotation matrix (orthonormal, det=1)
        let m = matrix;
        let trace = m[(0, 0)] + m[(1, 1)] + m[(2, 2)];
        let (x, y, z, w);

        if trace > 0.0 {
            let s = (trace + 1.0).sqrt() * 2.0;
            w = 0.25 * s;
            x = (m[(2, 1)] - m[(1, 2)]) / s;
            y = (m[(0, 2)] - m[(2, 0)]) / s;
            z = (m[(1, 0)] - m[(0, 1)]) / s;
        } else if m[(0, 0)] > m[(1, 1)] && m[(0, 0)] > m[(2, 2)] {
            let s = (1.0 + m[(0, 0)] - m[(1, 1)] - m[(2, 2)]).sqrt() * 2.0;
            w = (m[(2, 1)] - m[(1, 2)]) / s;
            x = 0.25 * s;
            y = (m[(0, 1)] + m[(1, 0)]) / s;
            z = (m[(0, 2)] + m[(2, 0)]) / s;
        } else if m[(1, 1)] > m[(2, 2)] {
            let s = (1.0 + m[(1, 1)] - m[(0, 0)] - m[(2, 2)]).sqrt() * 2.0;
            w = (m[(0, 2)] - m[(2, 0)]) / s;
            x = (m[(0, 1)] + m[(1, 0)]) / s;
            y = 0.25 * s;
            z = (m[(1, 2)] + m[(2, 1)]) / s;
        } else {
            let s = (1.0 + m[(2, 2)] - m[(0, 0)] - m[(1, 1)]).sqrt() * 2.0;
            w = (m[(1, 0)] - m[(0, 1)]) / s;
            x = (m[(0, 2)] + m[(2, 0)]) / s;
            y = (m[(1, 2)] + m[(2, 1)]) / s;
            z = 0.25 * s;
        }

        UnitQuaternion::new_normalized(Vec3::new(x, y, z), w)
    }

    pub fn rotate_vector(&self, vector: Vec3) -> Vec3 {
        let q_vector = Quaternion::from(vector);
        let q_conjugate = self.quat.conjugate();
        let rotated = self.quat * q_vector * q_conjugate;
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

        // Pitch (X-axis rotation)
        // Handle gimbal lock: when pitch is +/- 90 degrees
        let sinp_arg = 2.0 * (qw * qx - qy * qz);
        let pitch = if sinp_arg.abs() >= 1.0 {
            sinp_arg.signum() * std::f32::consts::FRAC_PI_2
        } else {
            sinp_arg.asin()
        };

        // Yaw (Y-axis rotation)
        let siny_cosp = 2.0 * (qw * qy + qx * qz);
        let cosy_cosp = 1.0 - 2.0 * (qx * qx + qy * qy);
        let yaw = siny_cosp.atan2(cosy_cosp);

        // Roll (Z-axis rotation)
        let sinr_cosp = 2.0 * (qw * qz + qx * qy);
        let cosr_cosp = 1.0 - 2.0 * (qx * qx + qz * qz);
        let roll = sinr_cosp.atan2(cosr_cosp);

        Vec3::new(pitch, yaw, roll)
    }

    pub fn rotation_matrix(&self) -> Matrix<3, 3> {
        self.homogeneous_matrix().view(0, 0)
    }

    pub fn homogeneous_matrix(&self) -> Matrix<4, 4> {
        let q = &self.quat;
        let x = q.vector.x;
        let y = q.vector.y;
        let z = q.vector.z;
        let w = q.scalar;

        let xx = x * x;
        let yy = y * y;
        let zz = z * z;
        let xy = x * y;
        let xz = x * z;
        let yz = y * z;
        let wx = w * x;
        let wy = w * y;
        let wz = w * z;

        Matrix::from([
            [1.0 - 2.0 * (yy + zz), 2.0 * (xy + wz), 2.0 * (xz - wy), 0.0],
            [2.0 * (xy - wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz + wx), 0.0],
            [2.0 * (xz + wy), 2.0 * (yz - wx), 1.0 - 2.0 * (xx + yy), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn slerp(&self, other: &Self, t: f32) -> Self {
        // Dot product of the two quaternions.
        let mut dot = self.dot(other);

        // If dot product is negative, the quaternions are more than 90 degrees apart.
        // To take the shorter path, we can invert one of the quaternions.
        let mut other_quat = *other;
        if dot < 0.0 {
            other_quat.quat.vector = -other_quat.quat.vector;
            other_quat.quat.scalar = -other_quat.quat.scalar;
            dot = -dot;
        }

        const DOT_THRESHOLD: f32 = 0.9995;
        if dot > DOT_THRESHOLD {
            // If the quaternions are very close, we use linear interpolation (lerp).
            // This avoids issues with division by zero when sin(theta) is close to zero.
            let result = Quaternion {
                vector: self.vector + (other_quat.vector - self.vector) * t,
                scalar: self.scalar + (other_quat.scalar - self.scalar) * t,
            };
            return UnitQuaternion {
                quat: result.normalize(),
            };
        }

        // Standard slerp
        let theta_0 = dot.acos(); // angle between input quaternions
        let theta = theta_0 * t; // angle for interpolation

        let sin_theta_0 = theta_0.sin();

        let scale_a = (theta_0 - theta).sin() / sin_theta_0;
        let scale_b = theta.sin() / sin_theta_0;

        let result = Quaternion {
            vector: self.vector * scale_a + other_quat.vector * scale_b,
            scalar: self.scalar * scale_a + other_quat.scalar * scale_b,
        };

        // The result should be normalized, but we'll normalize to be safe.
        UnitQuaternion {
            quat: result.normalize(),
        }
    }
}

// From -------------------------------------------------------------------------------------------
impl From<&Quaternion> for UnitQuaternion {
    fn from(quat: &Quaternion) -> Self {
        Self {
            quat: quat.normalize(),
        }
    }
}

impl From<Quaternion> for UnitQuaternion {
    fn from(quat: Quaternion) -> Self {
        Self::from(&quat)
    }
}

impl From<UnitQuaternion> for Quaternion {
    fn from(unit_quat: UnitQuaternion) -> Self {
        unit_quat.quat
    }
}

impl From<&UnitQuaternion> for Quaternion {
    fn from(unit_quat: &UnitQuaternion) -> Self {
        unit_quat.quat
    }
}

impl From<&Matrix<3, 3>> for UnitQuaternion {
    fn from(matrix: &Matrix<3, 3>) -> Self {
        Self::from_rotation_matrix(matrix)
    }
}

impl From<Matrix<3, 3>> for UnitQuaternion {
    fn from(matrix: Matrix<3, 3>) -> Self {
        Self::from(&matrix)
    }
}

impl From<&Matrix<4, 4>> for UnitQuaternion {
    fn from(matrix: &Matrix<4, 4>) -> Self {
        Self::from_rotation_matrix(&matrix.view(0, 0))
    }
}

impl From<Matrix<4, 4>> for UnitQuaternion {
    fn from(matrix: Matrix<4, 4>) -> Self {
        Self::from(&matrix)
    }
}

impl From<&[f32; 4]> for UnitQuaternion {
    fn from(array: &[f32; 4]) -> Self {
        let vector = Vec3::new(array[0], array[1], array[2]);
        let scalar = array[3];
        Self::new_normalized(vector, scalar)
    }
}

impl From<[f32; 4]> for UnitQuaternion {
    fn from(array: [f32; 4]) -> Self {
        Self::from(&array)
    }
}
// From -------------------------------------------------------------------------------------------

// Deref and DerefMut -----------------------------------------------------------------------------
impl Deref for UnitQuaternion {
    type Target = Quaternion;

    fn deref(&self) -> &Self::Target {
        &self.quat
    }
}
// Deref and DerefMut -----------------------------------------------------------------------------
