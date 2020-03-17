//! A simple 32 band audio equalizer
//!
//! Usage:
//!
//! ```rust
//! use simple_eq::Equalizer;
//! use simple_eq::design::Curve;
//!
//! fn main() {
//!     // create an input signal, in our case a unit impulse.
//!     let mut h = vec![0.0; 128];
//!     h[0] = 1.0;
//!     
//!     // create the equalizer instance
//!     let mut eq = Equalizer::new(48.0e3);
//!     eq.set(0, Curve::Lowshelf, 100.0, 1.0, 6.0);
//!     eq.set(1, Curve::Peak, 1000.0, 10.0, -12.0);
//!     eq.set(2, Curve::Highpass, 5000.0, 0.5_f64.sqrt(), 3.0);
//!
//!     // process the signal
//!     eq.process_buffer(&mut h);
//!     
//!     // print the impulse response:
//!     println!("h = {:?};", h);
//! }
//! ```
pub mod design;
pub mod filter;
pub mod kernel;

use nalgebra::{convert as _c, RealField as Real};
const NUM_BANDS: usize = 32;
use design::*;
use kernel::*;

#[derive(Copy, Clone, Debug)]
pub struct Equalizer<R: Real> {
    design: [Design<R>; NUM_BANDS],
    kernel: [Kernel<R>; NUM_BANDS],
    bypass: [bool; NUM_BANDS],
    sample_rate: R,
}

impl<R: Real> Equalizer<R> {
    /// Construct a new [Equalizer] instance
    pub fn new(sample_rate: R) -> Self {
        Self {
            design: [Design::default(); NUM_BANDS],
            kernel: [Kernel::default(); NUM_BANDS],
            bypass: [true; NUM_BANDS],
            sample_rate,
        }
    }

    #[inline]
    pub fn set(&mut self, idx: usize, curve: Curve, frequency: R, resonance: R, gain: R) {
        self.design[idx] = Design {
            frequency: normalize_frequency(frequency, self.sample_rate),
            gain,
            resonance,
            curve,
        };
        self.bypass[idx] = false;
        self.update(idx);
    }

    /// Change the sample rate of the instance
    #[inline]
    pub fn set_sample_rate(&mut self, sample_rate: R) {
        for idx in 0..NUM_BANDS {
            let (k, d, _) = (
                &mut self.kernel[idx],
                &mut self.design[idx],
                &mut self.bypass[idx],
            );
            let freq_hz: R = _c::<f64, R>(2.0) * self.sample_rate * d.frequency;
            d.frequency = normalize_frequency(freq_hz, sample_rate);
            self.sample_rate = sample_rate;
            let (num, den) = d.digital_xfer_fn();
            k.set(num, den);
        }
    }

    /// Bypass all filters in the EQ
    #[inline]
    pub fn bypass_all(&mut self, bypass: bool) {
        for idx in 0..NUM_BANDS {
            self.set_bypass(idx, bypass);
        }
    }

    /// Bypass an individal band of the EQ
    #[inline]
    pub fn set_bypass(&mut self, idx: usize, bypass: bool) {
        self.bypass[idx] = bypass;
    }

    /// Set the gain of a single band of the equalizer
    #[inline]
    #[allow(non_snake_case)]
    pub fn set_gain(&mut self, idx: usize, gain_dB: R) {
        self.design[idx].gain = gain_dB;
        self.update(idx);
    }

    /// Set the frequency of an individual band of the equalizer
    #[inline]
    pub fn set_frequency(&mut self, idx: usize, freq_hz: R) {
        self.design[idx].frequency = normalize_frequency(freq_hz, self.sample_rate);
        self.update(idx);
    }

    /// Set the resonance/Q factor of a single band
    #[inline]
    pub fn set_resonance(&mut self, idx: usize, resonance: R) {
        self.design[idx].resonance = resonance;
        self.update(idx);
    }

    /// returns the bypass state of a single filter band
    #[inline]
    pub fn is_bypassed(&self, idx: usize) -> bool {
        self.bypass[idx]
    }

    /// Reset the state of all bands
    pub fn reset(&mut self) {
        for k in self.kernel.iter_mut() {
            k.reset();
        }
    }

    /// Gets the design of a single band. Note that the frequency parameter is
    /// in the units of normalized frequency (1/samples).
    pub fn get_design(&self, idx: usize) -> Design<R> {
        self.design[idx]
    }

    #[inline]
    fn update(&mut self, idx: usize) {
        let (k, d) = (&mut self.kernel[idx], &self.design[idx]);
        let (num, den) = d.digital_xfer_fn();
        k.set(num, den);
    }

    /// Process a single sample of input
    #[inline]
    pub fn process(&mut self, input: R) -> R {
        self.kernel
            .iter_mut()
            .zip(self.bypass.iter())
            .filter(|(_, b)| !*b)
            .fold(input, |x, (k, _)| k.eval(x))
    }

    /// Process a buffer of input samples
    pub fn process_buffer(&mut self, input: &mut [R]) {
        for x in input {
            *x = self.process(*x);
        }
    }
}

#[cfg(test)]
#[cfg(not(feature = "no-std"))]
mod test {
    use super::*;
    #[test]
    fn doc() {
        // create an input signal, in our case a unit impulse.
        let mut h = vec![0.0; 512];
        h[0] = 1.0;
        
        // create the equalizer instance
        let mut eq = Equalizer::new(48.0e3);
        eq.set(0, Curve::Highpass, 100.0, 0.5f64.sqrt(), 0.0);
        eq.set(1, Curve::Peak, 1000.0, 10.0, -12.0);
        eq.process_buffer(&mut h);     

        // print out the impulse response. 
        println!("impulse = {:?};", h); 

        // we can bypass the lowest band 
        eq.set_bypass(0, true);
    }
}