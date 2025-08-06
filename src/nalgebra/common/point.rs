use crate::{Vec2, Vec3, Vec4, Vector};

use super::na;

// Vec2 -------------------------------------------------------------------------------------------
impl From<na::Point2<f32>> for Vec2 {
    fn from(value: na::Point2<f32>) -> Self {
        Self::from([value.x, value.y])
    }
}

impl From<Vec2> for na::Point2<f32> {
    fn from(value: Vec2) -> Self {
        na::Point2::new(value.x, value.y)
    }
}

impl From<na::Point2<f32>> for Vector<2> {
    fn from(value: na::Point2<f32>) -> Self {
        Vector::from([value.x, value.y])
    }
}

impl From<Vector<2>> for na::Point2<f32> {
    fn from(value: Vector<2>) -> Self {
        na::Point2::new(value[0], value[1])
    }
}
// Vec2 -------------------------------------------------------------------------------------------

// Vec3 -------------------------------------------------------------------------------------------
impl From<na::Point3<f32>> for Vec3 {
    fn from(value: na::Point3<f32>) -> Self {
        Self::from([value.x, value.y, value.z])
    }
}

impl From<Vec3> for na::Point3<f32> {
    fn from(value: Vec3) -> Self {
        na::Point3::new(value.x, value.y, value.z)
    }
}

impl From<na::Point3<f32>> for Vector<3> {
    fn from(value: na::Point3<f32>) -> Self {
        Vector::from([value.x, value.y, value.z])
    }
}

impl From<Vector<3>> for na::Point3<f32> {
    fn from(value: Vector<3>) -> Self {
        na::Point3::new(value[0], value[1], value[2])
    }
}
// Vec3 -------------------------------------------------------------------------------------------

// Vec4 -------------------------------------------------------------------------------------------
impl From<na::Point4<f32>> for Vec4 {
    fn from(value: na::Point4<f32>) -> Self {
        Self::from([value.x, value.y, value.z, value.w])
    }
}

impl From<Vec4> for na::Point4<f32> {
    fn from(value: Vec4) -> Self {
        na::Point4::new(value.x, value.y, value.z, value.w)
    }
}

impl From<na::Point4<f32>> for Vector<4> {
    fn from(value: na::Point4<f32>) -> Self {
        Vector::from([value.x, value.y, value.z, value.w])
    }
}

impl From<Vector<4>> for na::Point4<f32> {
    fn from(value: Vector<4>) -> Self {
        na::Point4::new(value[0], value[1], value[2], value[3])
    }
}
// Vec4 -------------------------------------------------------------------------------------------
