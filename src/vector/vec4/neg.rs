use std::ops::Neg;

use crate::vector::vec4::Vec4;

impl Neg for Vec4 {
    type Output = Vec4;

    fn neg(self) -> Self::Output {
        let matrix = unsafe { -self.data.matrix };
        Vec4::from(matrix)
    }
}
