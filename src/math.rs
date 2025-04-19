#[cfg(feature = "nalgebra")]
pub type Vec2 = nalgebra::Vector2<f32>;

#[cfg(feature = "nalgebra")]
pub type Vec3 = nalgebra::Vector3<f32>;

#[cfg(feature = "nalgebra")]
pub type Mat2 = nalgebra::Matrix2<f32>;

#[cfg(feature = "nalgebra")]
pub type Mat3 = nalgebra::Matrix3<f32>;

#[cfg(feature = "glam")]
pub type Vec2 = glam::Vec2;

#[cfg(feature = "glam")]
pub type Vec3 = glam::Vec3A;

#[cfg(feature = "glam")]
pub type Mat2 = glam::Mat2;

#[cfg(feature = "glam")]
pub type Mat3 = glam::Mat3A;
