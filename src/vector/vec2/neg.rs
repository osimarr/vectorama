use std::ops::Neg;

use crate::vector::vec2::Vec2;

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        let matrix = unsafe { -self.data.matrix };
        Vec2::from(matrix)
    }
}
