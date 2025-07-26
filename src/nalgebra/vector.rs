use crate::{Vec2, Vec3, Vec4};

// Vec2 -------------------------------------------------------------------------------------------
impl From<na::Vector2<f32>> for Vec2 {
    fn from(value: na::Vector2<f32>) -> Self {
        Self::from([value.x, value.y])
    }
}

impl From<Vec2> for na::Vector2<f32> {
    fn from(value: Vec2) -> Self {
        na::Vector2::new(value.x, value.y)
    }
}
// Vec2 -------------------------------------------------------------------------------------------

// Vec3 -------------------------------------------------------------------------------------------
impl From<na::Vector3<f32>> for Vec3 {
    fn from(value: na::Vector3<f32>) -> Self {
        Self::from([value.x, value.y, value.z])
    }
}

impl From<Vec3> for na::Vector3<f32> {
    fn from(value: Vec3) -> Self {
        na::Vector3::new(value.x, value.y, value.z)
    }
}
// Vec3 -------------------------------------------------------------------------------------------

// Vec4 -------------------------------------------------------------------------------------------
impl From<na::Vector4<f32>> for Vec4 {
    fn from(value: na::Vector4<f32>) -> Self {
        Self::from([value.x, value.y, value.z, value.w])
    }
}

impl From<Vec4> for na::Vector4<f32> {
    fn from(value: Vec4) -> Self {
        na::Vector4::new(value.x, value.y, value.z, value.w)
    }
}
// Vec4 -------------------------------------------------------------------------------------------
