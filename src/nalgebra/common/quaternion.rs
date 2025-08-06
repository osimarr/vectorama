use crate::UnitQuaternion;

use super::na;

impl From<na::UnitQuaternion<f32>> for UnitQuaternion {
    fn from(value: na::UnitQuaternion<f32>) -> Self {
        // nalgebra (roll, pitch, yaw) translates to vectorama (x, y, z)
        let (x, y, z) = value.euler_angles();
        UnitQuaternion::from_euler_angles(x, y, z)
    }
}

impl From<UnitQuaternion> for na::UnitQuaternion<f32> {
    fn from(value: UnitQuaternion) -> Self {
        let euler = value.to_euler_angles();
        na::UnitQuaternion::from_euler_angles(euler.x, euler.y, euler.z)
    }
}
