use std::ops::{Add, AddAssign};

use crate::vector::vec2::Vec2;

// Vector -----------------------------------------------------------------------------------------
impl Add<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: &Vec2) -> Self::Output {
        let matrix = unsafe { self.data.matrix + rhs.data.matrix };
        Vec2::from(matrix)
    }
}

#[allow(clippy::op_ref)]
impl Add<&Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: &Vec2) -> Self::Output {
        &self + rhs
    }
}

#[allow(clippy::op_ref)]
impl Add<Vec2> for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        self + &rhs
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        &self + &rhs
    }
}

impl AddAssign<&Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: &Vec2) {
        unsafe { self.data.matrix += rhs.data.matrix }
    }
}

impl AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        *self += &rhs
    }
}
// Vector -----------------------------------------------------------------------------------------
