use crate::UnitQuaternion;

impl From<na::UnitQuaternion<f32>> for UnitQuaternion {
    fn from(value: na::UnitQuaternion<f32>) -> Self {
        let (roll, pitch, yaw) = value.euler_angles(); // nalgebra: (x, y, z) in ZYX order
        UnitQuaternion::from_euler_angles(roll, pitch, yaw)
    }
}

impl From<UnitQuaternion> for na::UnitQuaternion<f32> {
    fn from(value: UnitQuaternion) -> Self {
        let euler = value.to_euler_angles(); // XYZ order
        na::UnitQuaternion::from_euler_angles(euler.x, euler.y, euler.z) // Convert to nalgebra's ZYX order
    }
}
