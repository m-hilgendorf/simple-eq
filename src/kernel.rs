//! Data structures and methods to do the actual filtering
//!
//! The approach taken here is a state-space model of the system evolution.

use crate::math::*;

/// A 2nd order filter "kernel" implemented as a state-space formulation. For reference on the derivation/description, check out
/// [Raph Levien's excellent notebook](https://github.com/google/music-synthesizer-for-android/blob/master/lab/Second%20order%20sections%20in%20matrix%20form.ipynb)
#[derive(Copy, Clone, Debug)]
#[allow(non_snake_case)]
pub struct Kernel {
    pub A: Mat2,
    pub B: Vec2,
    pub C: Vec3,
    pub s: Vec2,
}

impl Default for Kernel {
    fn default() -> Self {
        Self {
            #[cfg(feature = "glam")]
            A: Mat2::ZERO,
            #[cfg(feature = "nalgebra")]
            A: Mat2::new(0.0, 0.0, 0.0, 0.0),
            B: Vec2::new(1.0, 0.0),
            C: Vec3::new(0.0, 0.0, 0.0),
            s: Vec2::new(0.0, 0.0),
        }
    }
}

impl Kernel {
    /// Constuct a new filter kernel. Default's to pass-through
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset the filter's state to zero.
    pub fn reset(&mut self) {
        self.s = Vec2::new(0.0, 0.0);
    }

    /// set the coefficient matrices.
    pub fn set(&mut self, num: Vec3, den: Vec3) {
        #[cfg(feature = "nalgebra")]
        {
            self.A = Mat2::new(-den[1], 1.0, -den[2], 0.0);
        }
        #[cfg(feature = "glam")]
        {
            self.A = Mat2::from_cols(Vec2::new(-den[1], -den[2]), Vec2::new(1.0, 0.0))
        }
        self.B = Vec2::new(num[1] - den[1] * num[0], num[2] - den[2] * num[0]);
        self.C = Vec3::new(num[0], 1.0, 0.0);
    }

    /// Evaluate the kernel's transfer characteristics
    pub fn eval(&mut self, x: f32) -> f32 {
        #[cfg(feature = "glam")]
        {
            let u = Vec3::new(x, self.s[0], self.s[1]);
            let out = self.C.dot(u);
            self.s = self.A * self.s + self.B * x;
            out
        }
        #[cfg(feature = "nalgebra")]
        {
            let u = Vec3::new(x, self.s[0], self.s[1]);
            let out = self.C.dot(&u);
            self.s = self.A * self.s + self.B * x;
            out
        }
    }
}
