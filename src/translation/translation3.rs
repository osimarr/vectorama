use std::ops::{AddAssign, Deref, DerefMut, Mul, SubAssign};

use crate::{
    matrix::Matrix,
    translation::{from_homogeneous_matrix, to_homogeneous_matrix},
    vector::{Vector, vec3::Vec3},
};

/// A 3D translation represented by a vector.
///
/// `Translation3` encodes a translation in 3D space using a `Vec3`. It provides convenient methods
/// for constructing, combining, and applying translations, as well as conversion to and from
/// homogeneous transformation matrices.
///
/// # Example
/// ```
/// use vectorama::Translation3;
/// let t = Translation3::new(1.0, 2.0, 3.0);
/// ```
#[derive(Default, Debug, Clone, Copy)]
pub struct Translation3 {
    vector: Vec3,
}

impl Translation3 {
    /// Creates a new translation from x, y, z components.
    ///
    /// # Parameters
    /// - `x`: Translation along the X axis.
    /// - `y`: Translation along the Y axis.
    /// - `z`: Translation along the Z axis.
    ///
    /// # Returns
    /// A new `Translation3` representing the given translation.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Translation3 {
            vector: Vec3::new(x, y, z),
        }
    }

    /// Returns the 4x4 homogeneous transformation matrix for this translation.
    ///
    /// # Returns
    /// A 4x4 matrix representing the translation in homogeneous coordinates.
    pub fn homogeneous_matrix(&self) -> Matrix<4, 4> {
        to_homogeneous_matrix(self.vector)
    }

    /// Applies this translation to a vector.
    ///
    /// # Parameters
    /// - `vector`: The vector to translate.
    ///
    /// # Returns
    /// The translated vector.
    pub fn translate(&self, vector: &Vec3) -> Vec3 {
        vector + self.vector
    }
}

// From -------------------------------------------------------------------------------------------
impl From<Vec3> for Translation3 {
    fn from(vector: Vec3) -> Self {
        Translation3 { vector }
    }
}

impl From<Translation3> for Vec3 {
    fn from(translation: Translation3) -> Self {
        translation.vector
    }
}

impl From<[f32; 3]> for Translation3 {
    fn from(array: [f32; 3]) -> Self {
        Translation3::from(Vec3::from(array))
    }
}

impl From<Vector<3>> for Translation3 {
    fn from(vector: Vector<3>) -> Self {
        Translation3::from(Vec3::from(vector))
    }
}

impl From<&Matrix<4, 4>> for Translation3 {
    fn from(matrix: &Matrix<4, 4>) -> Self {
        from_homogeneous_matrix(matrix).into()
    }
}

impl From<Matrix<4, 4>> for Translation3 {
    fn from(matrix: Matrix<4, 4>) -> Self {
        Translation3::from(&matrix)
    }
}
// From -------------------------------------------------------------------------------------------

// Deref and DerefMut -----------------------------------------------------------------------------
impl Deref for Translation3 {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.vector
    }
}

impl DerefMut for Translation3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vector
    }
}
// Deref and DerefMut -----------------------------------------------------------------------------

// Mul Vector -------------------------------------------------------------------------------------
impl Mul<&Vec3> for &Translation3 {
    type Output = Vec3;

    fn mul(self, vector: &Vec3) -> Self::Output {
        self.translate(vector)
    }
}

#[allow(clippy::op_ref)]
impl Mul<Vec3> for &Translation3 {
    type Output = Vec3;

    fn mul(self, vector: Vec3) -> Self::Output {
        self * &vector
    }
}

#[allow(clippy::op_ref)]
impl Mul<&Vec3> for Translation3 {
    type Output = Vec3;
    fn mul(self, vector: &Vec3) -> Self::Output {
        &self * vector
    }
}

impl Mul<Vec3> for Translation3 {
    type Output = Vec3;

    fn mul(self, vector: Vec3) -> Self::Output {
        &self * &vector
    }
}
// Mul Vector -------------------------------------------------------------------------------------

// AddAssign Vector -------------------------------------------------------------------------------
impl AddAssign<&Vec3> for Translation3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.vector += rhs;
    }
}
impl AddAssign<Vec3> for Translation3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self += &rhs;
    }
}
// AddAssign Vector -------------------------------------------------------------------------------

// SubAssign Vector -------------------------------------------------------------------------------
impl SubAssign<&Vec3> for Translation3 {
    fn sub_assign(&mut self, rhs: &Vec3) {
        self.vector -= rhs;
    }
}
impl SubAssign<Vec3> for Translation3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self -= &rhs;
    }
}
// SubAssign Vector -------------------------------------------------------------------------------
