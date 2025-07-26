use std::ops::{Sub, SubAssign};

use crate::vector::vec3::Vec3;

// Vector -----------------------------------------------------------------------------------------
impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        let matrix = unsafe { self.data.matrix - rhs.data.matrix };
        Vec3::from(matrix)
    }
}

#[allow(clippy::op_ref)]
impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        &self - rhs
    }
}

#[allow(clippy::op_ref)]
impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        self - &rhs
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        &self - &rhs
    }
}

impl SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: &Vec3) {
        unsafe { self.data.matrix -= rhs.data.matrix }
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self -= &rhs
    }
}
// Vector -----------------------------------------------------------------------------------------
