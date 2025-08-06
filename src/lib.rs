pub mod matrix;
pub mod quaternion;
pub mod scale;
pub mod translation;
pub mod vector;

pub mod nalgebra;

pub use matrix::Matrix;
pub type Mat3 = Matrix<3, 3>;
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
