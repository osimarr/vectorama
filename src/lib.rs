pub mod matrix;
pub mod quaternion;
pub mod scale;
pub mod translation;
pub mod vector;

pub mod nalgebra;

pub use matrix::Matrix;
/// A 3x3 column-major matrix of `f32` values, matching OpenGL and glTF conventions.
///
/// # Example
/// ```
/// use vectorama::Mat3;
/// let m = Mat3::ones();
/// ```
pub type Mat3 = Matrix<3, 3>;

/// A 4x4 column-major matrix of `f32` values, matching OpenGL and glTF conventions.
///
/// # Example
/// ```
/// use vectorama::Mat4;
/// let m = Mat4::ones();
/// ```
pub type Mat4 = Matrix<4, 4>;

pub use vector::Vector;
pub use vector::vec2::Vec2;
pub use vector::vec3::Vec3;
pub use vector::vec4::Vec4;

pub use quaternion::Quaternion;
pub use quaternion::unit::UnitQuaternion;

pub use scale::scale2::Scale2;
pub use scale::scale3::Scale3;
pub use translation::translation2::Translation2;
pub use translation::translation3::Translation3;
