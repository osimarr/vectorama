use std::ops::Div;

use crate::Quaternion;

// Quaternion -------------------------------------------------------------------------------------
impl Div<f32> for &Quaternion {
    type Output = Quaternion;

    fn div(self, rhs: f32) -> Self::Output {
        Quaternion {
            vector: self.vector / rhs,
            scalar: self.scalar / rhs,
        }
    }
}

impl Div<f32> for Quaternion {
    type Output = Quaternion;

    fn div(self, rhs: f32) -> Self::Output {
        &self / rhs
    }
}
// Quaternion -------------------------------------------------------------------------------------
