use std::ops::Neg;

use crate::vector::vec3::Vec3;

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        let matrix = unsafe { -self.data.matrix };
        Vec3::from(matrix)
    }
}
