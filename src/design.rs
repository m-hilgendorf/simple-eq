#![allow(clippy::just_underscores_and_digits)]
//! Structures and methods for calculating filter coefficients from
//! design parameters.
//!

use core::convert::From;
use nalgebra::{convert as _c, Matrix3, RealField as Real, Vector3 as Vec3};

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

#[derive(Copy, Clone, Debug)]
pub struct Design<R: Real> {
    pub curve: Curve,
    pub frequency: R,
    pub resonance: R,
    pub gain: R,
}

impl<R: Real + Copy> Default for Design<R> {
    fn default() -> Self {
        Self {
            curve: Curve::Peak,
            frequency: _c(1.0 / 24.0),
            resonance: _c(0.5f64.sqrt()),
            gain: _c(0.0),
        }
    }
}

impl<R: Real + Copy> Design<R> {
    /// Compute the dgital transfer function.
    pub fn digital_xfer_fn(&self) -> (Vec3<R>, Vec3<R>) {
        let (a_num, a_den) = self.analog_xfer_fn();
        let (t_num, t_den) = (trans_quad(a_num), trans_quad(a_den));
        let scale = t_den[0];
        (t_num / scale, t_den / scale)
    }

    /// Compute the continuous time transfer function of the filter
    #[rustfmt::skip]
    pub fn analog_xfer_fn (&self) -> (Vec3<R>, Vec3<R>) {
        let omega_c = prewarp(self.frequency);
        let scale   = db2lin(self.gain).sqrt();
        let den     = [_c(1.0), omega_c / self.resonance, omega_c * omega_c];
        let (num, den) = match self.curve {
            Curve::Lowpass  => {
                ([_c(0.0), _c(0.0), omega_c * omega_c], den)
            },
            Curve::Highpass => ([_c(1.0), _c(0.0), _c(0.0)], den),
            Curve::Bandpass => ([_c(0.0), omega_c / self.resonance, _c(0.0)], den),
            Curve::Notch    => ([_c(1.0), _c(0.0), omega_c * omega_c], den),
            Curve::Peak     => (
                [_c(1.0), omega_c * scale / self.resonance, omega_c * omega_c],
                [_c(1.0), omega_c / (self.resonance * scale), omega_c * omega_c],
            ),
            Curve::Highshelf => {
                let (mut num, mut den) = (
                    [scale, omega_c * scale.sqrt() / self.resonance, omega_c * omega_c],
                    [_c(1.0), omega_c * scale.sqrt() / self.resonance, omega_c * omega_c * scale],
                );
                for i in 0..3 {
                    num[i] *= scale;
                    den[i] *= scale;
                }
                (num, den)
            }
            Curve::Lowshelf => {
                let (mut num, mut den) = (
                    [_c(1.0), omega_c * scale.sqrt() / self.resonance, omega_c * omega_c * scale],
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
}

/// Normalize a frequency in Hertz (1/s) to its discrete time equivalent (1/samples) given
/// the system's sample rate. Will panic if you try and normalize a frequency past Nyquist.
#[inline]
pub fn normalize_frequency<R: Real + Copy>(frequency: R, sample_rate: R) -> R {
    assert!(frequency < (sample_rate / _c(2.0)));
    frequency / sample_rate
}

#[inline]
fn db2lin<R: Real + Copy>(db: R) -> R {
    _c::<f64, R>(10.0).powf(db / _c(20.0))
}

#[inline]
fn prewarp<R: Real + Copy>(normalized_freq: R) -> R {
    //let prewarped = 4.0 * (normalized_frequency * 0.5 * std::f64::consts::PI).tan();
    let (_4, _0_5, pi) = (_c::<f64, R>(4.0), _c::<f64, R>(0.5), R::pi());
    _4 * (normalized_freq * _0_5 * pi).tan()
}

#[inline] 
#[allow(non_snake_case)]
#[rustfmt::skip]
fn trans_quad<R: Real + Copy>(Q: Vec3<R>) -> Vec3<R> {
    let TQ: Vec3<R> = Vec3::new(
        _c::<f64, R> (1.0) * Q[0], 
        _c::<f64, R>(1.0 / 4.0) * Q[1], 
        _c::<f64, R>(1.0 / 16.0) * Q[2]
    ); 
    let X: Matrix3<R> = Matrix3::new(
        _c(1.0), _c(1.0), _c(1.0), 
        _c(-2.0), _c(0.0), _c(2.0),
        _c(1.0), _c(-1.0), _c(1.0)
    );
    X * TQ 
}
