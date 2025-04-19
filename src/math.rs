use core::ops::{Add, Div, Mul, Sub};

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

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Complex(Vec2);

#[cfg(feature = "std")]
impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} + j{}", self.real(), self.imag())
    }
}

impl Complex {
    pub fn new(real: f32, imag: f32) -> Self {
        Self(Vec2::new(real, imag))
    }

    #[inline]
    pub fn real(&self) -> f32 {
        self.0[0]
    }

    #[inline]
    pub fn imag(&self) -> f32 {
        self.0[1]
    }

    #[inline]
    pub fn abs(&self) -> f32 {
        (self.real() * self.real() + self.imag() * self.imag()).sqrt()
    }

    #[inline]
    pub fn arg(&self) -> f32 {
        self.imag().atan2(self.real())
    }
}

impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Add<f32> for Complex {
    type Output = Self;
    fn add(self, rhs: f32) -> Self::Output {
        Self(Vec2::new(self.real() + rhs, self.imag()))
    }
}

impl Mul<f32> for Complex {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Div<f32> for Complex {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl From<f32> for Complex {
    fn from(value: f32) -> Self {
        Self(Vec2::new(value, 0.0))
    }
}

