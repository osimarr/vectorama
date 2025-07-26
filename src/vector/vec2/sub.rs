use std::ops::{Sub, SubAssign};

use crate::vector::vec2::Vec2;

// Vector -----------------------------------------------------------------------------------------
impl Sub<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: &Vec2) -> Self::Output {
        let matrix = unsafe { self.data.matrix - rhs.data.matrix };
        Vec2::from(matrix)
    }
}

#[allow(clippy::op_ref)]
impl Sub<&Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: &Vec2) -> Self::Output {
        &self - rhs
    }
}

#[allow(clippy::op_ref)]
impl Sub<Vec2> for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        self - &rhs
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        &self - &rhs
    }
}

impl SubAssign<&Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: &Vec2) {
        unsafe { self.data.matrix -= rhs.data.matrix }
    }
}

impl SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        *self -= &rhs
    }
}
// Vector -----------------------------------------------------------------------------------------
