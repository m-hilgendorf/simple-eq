//! Data structures and methods to do the actual filtering
//!
//! The approach taken here is a state-space model of the system evolution.
use nalgebra::{one, zero, Matrix2 as Mat2, RealField as Real, Vector2 as Vec2, Vector3 as Vec3};

/// A 2nd order filter "kernel" implemented as a state-space formulation. For reference on the derivation/description, check out
/// [Raph Levien's excellent notebook](https://github.com/google/music-synthesizer-for-android/blob/master/lab/Second%20order%20sections%20in%20matrix%20form.ipynb)    
#[derive(Copy, Clone, Debug)]
#[allow(non_snake_case)]
pub struct Kernel<R: Real> {
    pub A: Mat2<R>,
    pub B: Vec2<R>,
    pub C: Vec3<R>,
    pub s: Vec2<R>,
}

impl<R: Real> Default for Kernel<R> {
    fn default() -> Self {
        Self {
            A: zero(),
            B: Vec2::new(one(), zero()),
            C: zero(),
            s: zero(),
        }
    }
}

impl<R: Real> Kernel<R> {
    /// Constuct a new filter kernel. Default's to pass-through
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset the filter's state to zero.
    pub fn reset(&mut self) {
        self.s = zero();
    }

    /// set the coefficient matrices. 
    pub fn set(&mut self, num: Vec3<R>, den: Vec3<R>) {
        //let (b0, num[1], b2, den[1], den[2]) = (num[0], num[1], num[2], den[1], den[2]);
        self.A = Mat2::new(
            -den[1], one(), 
            -den[2], zero(),
        );
        self.B = Vec2::new(
            num[1] - den[1] * num[0],
            num[2] - den[2] * num[0], 
        ); 
        self.C = Vec3::new(num[0], one(), zero());
    }

    /// Evaluate the kernel's transfer characteristics
    pub fn eval(&mut self, x: R) -> R {
        let u   = Vec3::new(x, self.s[0], self.s[1]);
        let out = self.C.dot(&u);
        self.s  = self.A * self.s + self.B * x;
        out
    }
}