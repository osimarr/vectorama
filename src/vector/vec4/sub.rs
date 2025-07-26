use std::ops::{Sub, SubAssign};

use crate::vector::vec4::Vec4;

// Vector -----------------------------------------------------------------------------------------
impl Sub<&Vec4> for &Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: &Vec4) -> Self::Output {
        let matrix = unsafe { self.data.matrix - rhs.data.matrix };
        Vec4::from(matrix)
    }
}

#[allow(clippy::op_ref)]
impl Sub<&Vec4> for Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: &Vec4) -> Self::Output {
        &self - rhs
    }
}

#[allow(clippy::op_ref)]
impl Sub<Vec4> for &Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: Vec4) -> Self::Output {
        self - &rhs
    }
}

impl Sub<Vec4> for Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: Vec4) -> Self::Output {
        &self - &rhs
    }
}

impl SubAssign<&Vec4> for Vec4 {
    fn sub_assign(&mut self, rhs: &Vec4) {
        unsafe { self.data.matrix -= rhs.data.matrix }
    }
}

impl SubAssign<Vec4> for Vec4 {
    fn sub_assign(&mut self, rhs: Vec4) {
        *self -= &rhs
    }
}
// Vector -----------------------------------------------------------------------------------------
