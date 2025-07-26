use std::ops::{Add, AddAssign};

use crate::vector::vec3::Vec3;

// Vector -----------------------------------------------------------------------------------------
impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        let matrix = unsafe { self.data.matrix + rhs.data.matrix };
        Vec3::from(matrix)
    }
}

#[allow(clippy::op_ref)]
impl Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        &self + rhs
    }
}

#[allow(clippy::op_ref)]
impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        self + &rhs
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        &self + &rhs
    }
}

impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        unsafe { self.data.matrix += rhs.data.matrix }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self += &rhs
    }
}
// Vector -----------------------------------------------------------------------------------------
