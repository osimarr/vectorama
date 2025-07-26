use std::ops::{Mul, MulAssign};

use crate::quaternion::{Quaternion, unit::UnitQuaternion};

// Quaternion -------------------------------------------------------------------------------------
impl Mul<&Quaternion> for &Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: &Quaternion) -> Self::Output {
        let vector_part = self.vector.cross(&rhs.vector);
        let scalar_part = self.scalar * rhs.scalar - self.vector.dot(&rhs.vector);
        Quaternion {
            vector: vector_part + rhs.vector * self.scalar + self.vector * rhs.scalar,
            scalar: scalar_part,
        }
    }
}

#[allow(clippy::op_ref)]
impl Mul<&Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: &Quaternion) -> Self::Output {
        &self * rhs
    }
}

#[allow(clippy::op_ref)]
impl Mul<Quaternion> for &Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Self::Output {
        self * &rhs
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Self::Output {
        &self * &rhs
    }
}
// Quaternion -------------------------------------------------------------------------------------

// UnitQuaternion ---------------------------------------------------------------------------------
impl Mul<&UnitQuaternion> for &UnitQuaternion {
    type Output = UnitQuaternion;

    fn mul(self, rhs: &UnitQuaternion) -> Self::Output {
        let vector_part = self.vector.cross(&rhs.vector);
        let scalar_part = self.scalar * rhs.scalar - self.vector.dot(&rhs.vector);
        UnitQuaternion::from(Quaternion {
            vector: vector_part + rhs.vector * self.scalar + self.vector * rhs.scalar,
            scalar: scalar_part,
        })
    }
}

#[allow(clippy::op_ref)]
impl Mul<&UnitQuaternion> for UnitQuaternion {
    type Output = UnitQuaternion;

    fn mul(self, rhs: &UnitQuaternion) -> Self::Output {
        &self * rhs
    }
}

#[allow(clippy::op_ref)]
impl Mul<UnitQuaternion> for &UnitQuaternion {
    type Output = UnitQuaternion;

    fn mul(self, rhs: UnitQuaternion) -> Self::Output {
        self * &rhs
    }
}

impl Mul<UnitQuaternion> for UnitQuaternion {
    type Output = UnitQuaternion;

    fn mul(self, rhs: UnitQuaternion) -> Self::Output {
        &self * &rhs
    }
}

impl MulAssign<&UnitQuaternion> for UnitQuaternion {
    fn mul_assign(&mut self, rhs: &UnitQuaternion) {
        *self = *self * rhs;
    }
}

impl MulAssign<UnitQuaternion> for UnitQuaternion {
    fn mul_assign(&mut self, rhs: UnitQuaternion) {
        *self *= &rhs;
    }
}
// UnitQuaternion ---------------------------------------------------------------------------------
