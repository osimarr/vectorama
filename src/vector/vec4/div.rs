use std::ops::{Div, DivAssign};

use crate::vector::vec4::Vec4;

// Scalar -----------------------------------------------------------------------------------------
impl Div<f32> for &Vec4 {
    type Output = Vec4;

    fn div(self, rhs: f32) -> Self::Output {
        let matrix = unsafe { self.data.matrix / rhs };
        Vec4::from(matrix)
    }
}

#[allow(clippy::op_ref)]
impl Div<f32> for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: f32) -> Self::Output {
        &self / rhs
    }
}

impl DivAssign<f32> for Vec4 {
    fn div_assign(&mut self, rhs: f32) {
        unsafe { self.data.matrix /= rhs }
    }
}
// Scalar -----------------------------------------------------------------------------------------
