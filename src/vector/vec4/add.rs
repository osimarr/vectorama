use std::ops::{Add, AddAssign};

use crate::vector::vec4::Vec4;

// Vector -----------------------------------------------------------------------------------------
impl Add<&Vec4> for &Vec4 {
    type Output = Vec4;

    fn add(self, rhs: &Vec4) -> Self::Output {
        let matrix = unsafe { self.data.matrix + rhs.data.matrix };
        Vec4::from(matrix)
    }
}

#[allow(clippy::op_ref)]
impl Add<&Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: &Vec4) -> Self::Output {
        &self + rhs
    }
}

#[allow(clippy::op_ref)]
impl Add<Vec4> for &Vec4 {
    type Output = Vec4;

    fn add(self, rhs: Vec4) -> Self::Output {
        self + &rhs
    }
}

impl Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: Vec4) -> Self::Output {
        &self + &rhs
    }
}

impl AddAssign<&Vec4> for Vec4 {
    fn add_assign(&mut self, rhs: &Vec4) {
        unsafe { self.data.matrix += rhs.data.matrix }
    }
}

impl AddAssign<Vec4> for Vec4 {
    fn add_assign(&mut self, rhs: Vec4) {
        *self += &rhs
    }
}
// Vector -----------------------------------------------------------------------------------------
