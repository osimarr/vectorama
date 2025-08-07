use std::ops::{AddAssign, Deref, DerefMut, Mul, SubAssign};

use crate::{
    matrix::Matrix,
    translation::{from_homogeneous_matrix, to_homogeneous_matrix},
    vector::{Vector, vec2::Vec2},
};

/// A 2D translation represented by a vector.
///
/// `Translation2` encodes a translation in 2D space using a `Vec2`. It provides convenient methods
/// for constructing, combining, and applying translations, as well as conversion to and from
/// homogeneous transformation matrices.
///
/// # Example
/// ```
/// use vectorama::Translation2;
/// let t = Translation2::new(1.0, 2.0);
/// ```
#[derive(Default, Debug, Clone, Copy)]
pub struct Translation2 {
    vector: Vec2,
}

impl Translation2 {
    /// Creates a new translation from x, y components.
    ///
    /// # Parameters
    /// - `x`: Translation along the X axis.
    /// - `y`: Translation along the Y axis.
    ///
    /// # Returns
    /// A new `Translation2` representing the given translation.
    pub fn new(x: f32, y: f32) -> Self {
        Translation2 {
            vector: Vec2::new(x, y),
        }
    }

    /// Returns the 3x3 homogeneous transformation matrix for this translation.
    ///
    /// # Returns
    /// A 3x3 matrix representing the translation in homogeneous coordinates.
    pub fn homogeneous_matrix(&self) -> Matrix<3, 3> {
        to_homogeneous_matrix(self.vector)
    }

    /// Applies this translation to a vector.
    ///
    /// # Parameters
    /// - `vector`: The vector to translate.
    ///
    /// # Returns
    /// The translated vector.
    pub fn translate(&self, vector: &Vec2) -> Vec2 {
        vector + self.vector
    }
}

// From -------------------------------------------------------------------------------------------
impl From<Vec2> for Translation2 {
    fn from(vector: Vec2) -> Self {
        Translation2 { vector }
    }
}

impl From<Translation2> for Vec2 {
    fn from(translation: Translation2) -> Self {
        translation.vector
    }
}

impl From<[f32; 2]> for Translation2 {
    fn from(array: [f32; 2]) -> Self {
        Translation2::from(Vec2::from(array))
    }
}

impl From<Vector<2>> for Translation2 {
    fn from(vector: Vector<2>) -> Self {
        Translation2::from(Vec2::from(vector))
    }
}

impl From<&Matrix<3, 3>> for Translation2 {
    fn from(matrix: &Matrix<3, 3>) -> Self {
        from_homogeneous_matrix(matrix).into()
    }
}

impl From<Matrix<3, 3>> for Translation2 {
    fn from(matrix: Matrix<3, 3>) -> Self {
        Translation2::from(&matrix)
    }
}
// From -------------------------------------------------------------------------------------------

// Deref and DerefMut -----------------------------------------------------------------------------
impl Deref for Translation2 {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.vector
    }
}

impl DerefMut for Translation2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vector
    }
}
// Deref and DerefMut -----------------------------------------------------------------------------

// Mul Vector -------------------------------------------------------------------------------------
impl Mul<&Vec2> for &Translation2 {
    type Output = Vec2;

    fn mul(self, vector: &Vec2) -> Self::Output {
        self.translate(vector)
    }
}

#[allow(clippy::op_ref)]
impl Mul<Vec2> for &Translation2 {
    type Output = Vec2;

    fn mul(self, vector: Vec2) -> Self::Output {
        self * &vector
    }
}

#[allow(clippy::op_ref)]
impl Mul<&Vec2> for Translation2 {
    type Output = Vec2;
    fn mul(self, vector: &Vec2) -> Self::Output {
        &self * vector
    }
}

impl Mul<Vec2> for Translation2 {
    type Output = Vec2;

    fn mul(self, vector: Vec2) -> Self::Output {
        &self * &vector
    }
}
// Mul Vector -------------------------------------------------------------------------------------

// AddAssign Vector -------------------------------------------------------------------------------
impl AddAssign<&Vec2> for Translation2 {
    fn add_assign(&mut self, rhs: &Vec2) {
        self.vector += rhs;
    }
}
impl AddAssign<Vec2> for Translation2 {
    fn add_assign(&mut self, rhs: Vec2) {
        *self += &rhs;
    }
}
// AddAssign Vector -------------------------------------------------------------------------------

// SubAssign Vector -------------------------------------------------------------------------------
impl SubAssign<&Vec2> for Translation2 {
    fn sub_assign(&mut self, rhs: &Vec2) {
        self.vector -= rhs;
    }
}
impl SubAssign<Vec2> for Translation2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        *self -= &rhs;
    }
}
// SubAssign Vector -------------------------------------------------------------------------------
