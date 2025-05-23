use crate::math::*;
use core::{convert::From, f32::consts::PI};

#[derive(Copy, Clone, Debug)]
pub struct Parameters {
    pub curve: Curve,
    pub frequency: f32,
    pub resonance: f32,
    pub gain: f32,
}

#[derive(Copy, Clone, Debug)]
pub enum Curve {
    Lowpass,
    Highpass,
    Bandpass,
    Notch,
    Peak,
    Lowshelf,
    Highshelf,
}

impl From<i32> for Curve {
    fn from(i: i32) -> Curve {
        match i {
            0 => Curve::Lowpass,
            1 => Curve::Highpass,
            2 => Curve::Bandpass,
            3 => Curve::Notch,
            4 => Curve::Peak,
            5 => Curve::Lowpass,
            6 => Curve::Highpass,
            _ => panic!("invalid argument"),
        }
    }
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            curve: Curve::Peak,
            frequency: 1.0 / 24.0,
            resonance: 0.5f32.sqrt(),
            gain: 0.0,
        }
    }
}

impl Parameters {
    /// Compute the dgital transfer function.
    pub fn digital_xfer_fn(&self) -> (Vec3, Vec3) {
        let (a_num, a_den) = self.analog_xfer_fn();
        let (t_num, t_den) = (trans_quad(a_num), trans_quad(a_den));
        let scale = t_den[0];
        (t_num / scale, t_den / scale)
    }

    /// Compute the continuous time transfer function of the filter
    #[rustfmt::skip]
    pub fn analog_xfer_fn (&self) -> (Vec3, Vec3) {
        let omega_c = prewarp(self.frequency);
        let scale   = db2lin(self.gain).sqrt();
        let den     = [1.0, omega_c / self.resonance, omega_c * omega_c];
        let (num, den) = match self.curve {
            Curve::Lowpass  => {
                ([0.0, 0.0, omega_c * omega_c], den)
            },
            Curve::Highpass => ([1.0, 0.0, 0.0], den),
            Curve::Bandpass => ([0.0, omega_c / self.resonance, 0.0], den),
            Curve::Notch    => ([1.0, 0.0, omega_c * omega_c], den),
            Curve::Peak     => (
                [1.0, omega_c * scale / self.resonance, omega_c * omega_c],
                [1.0, omega_c / (self.resonance * scale), omega_c * omega_c],
            ),
            Curve::Highshelf => {
                let (mut num, mut den) = (
                    [scale, omega_c * scale.sqrt() / self.resonance, omega_c * omega_c],
                    [1.0, omega_c * scale.sqrt() / self.resonance, omega_c * omega_c * scale],
                );
                for i in 0..3 {
                    num[i] *= scale;
                    den[i] *= scale;
                }
                (num, den)
            }
            Curve::Lowshelf => {
                let (mut num, mut den) = (
                    [1.0, omega_c * scale.sqrt() / self.resonance, omega_c * omega_c * scale],
                    [scale, omega_c * scale.sqrt() / self.resonance, omega_c * omega_c],
                );
                for i in 0..3 {
                    num[i] *= scale;
                    den[i] *= scale;
                }
                (num, den)
            }
        };

        (Vec3::new(num[0], num[1], num[2]), Vec3::new(den[0], den[1], den[2]))
    }

    /// Compute the frequency response at a desired frequency.
    pub fn eval(&self, normalized_frequency: f32) -> Complex {
        debug_assert!(normalized_frequency >= 0.0 && normalized_frequency <= 1.0);
        let (num, den) = self.digital_xfer_fn();
        let z = Complex::new(0.0, normalized_frequency * PI);
        let num = (z * z) * num[0] + z * num[1] + num[2];
        let den = (z * z) * den[0] + z * den[1] * den[2];
        num / den
    }
}

/// Normalize a frequency in Hertz (1/s) to its discrete time equivalent (1/samples) given
/// the system's sample rate. Will panic if you try and normalize a frequency past Nyquist.
#[inline]
pub fn normalize_frequency(frequency: f32, sample_rate: f32) -> f32 {
    assert!(frequency < (sample_rate / 2.0));
    frequency / sample_rate
}

#[inline]
fn db2lin(db: f32) -> f32 {
    10.0f32.powf(db / 20.0)
}

#[inline]
fn prewarp(normalized_freq: f32) -> f32 {
    4.0 * (normalized_freq * 0.5 * PI).tan()
}

#[inline]
#[allow(non_snake_case)]
#[rustfmt::skip]
fn trans_quad(Q: Vec3) -> Vec3 {
    let TQ: Vec3 = Vec3::new(
        1.0 * Q[0],
        1.0 / 4.0 * Q[1],
        1.0 / 16.0 * Q[2],
    );
    #[cfg(feature = "nalgebra")]
    {
        let X = Mat3::new(
             1.0,  1.0, 1.0,
            -2.0,  0.0, 2.0,
             1.0, -1.0, 1.0,
        );
        X * TQ
    }
    #[cfg(feature = "glam")]
    {
        let X = Mat3::from_cols(
            Vec3::new(1.0, -2.0, 1.0),
            Vec3::new(1.0, 0.0, -1.0),
            Vec3::new(1.0, 2.0, 1.0),
        );
        X * TQ
    }
}
