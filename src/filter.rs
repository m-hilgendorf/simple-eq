use crate::parameters::*;
use crate::state::*;
use crate::math::*;
/// A single filter band.
#[derive(Copy, Clone, Debug)]
pub struct Filter {
    kernel: State,
    design: Parameters,
    sample_rate: f32,
}

impl Filter {
    /// Construct a new filter instance
    pub fn new(sample_rate: f32) -> Self {
        let design = Parameters {
            curve: Curve::Peak,
            gain: 0.0,
            frequency: 0.1,
            resonance: 1.0,
        };
        let kernel = State::new();
        let mut self_ = Self {
            design,
            kernel,
            sample_rate,
        };
        self_.update();
        self_
    }

    /// Get a copy of the filter's current design parameters.
    pub fn get_design(&self) -> Parameters {
        self.design
    }

    /// Get a copy of the current filter state.
    pub fn get_state(&self) -> Vec2 {
        self.kernel.s
    }

    /// Set the curve parameter (lowpass, highpass, bandpass, etc) of the filter.
    pub fn set_curve(&mut self, curve: Curve) {
        self.design.curve = curve;
        self.update();
    }

    /// Set the critical frequency of the filter.
    pub fn set_frequency(&mut self, freq_hz: f32) {
        self.design.frequency = normalize_frequency(freq_hz, self.sample_rate);
        self.update();
    }

    /// set the gain of the filter. Meaningless for some filter curves.
    #[allow(non_snake_case)]
    pub fn set_gain(&mut self, gain_dB: f32) {
        self.design.gain = gain_dB;
        self.update();
    }

    /// Set the resonance (aka "Q" factor) of the filter
    pub fn set_resonance(&mut self, resonance: f32) {
        self.design.resonance = resonance;
        self.update();
    }

    /// Change the sample rate of the filter. This will reset the filter state.
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.update();
    }

    /// Zero the state of the filter.
    pub fn reset(&mut self) {
        self.kernel.reset();
    }

    fn update(&mut self) {
        let (num, den) = self.design.digital_xfer_fn();
        self.kernel.set(num, den);
    }

    #[inline]
    pub fn filter(&mut self, x: f32) -> f32 {
        self.kernel.eval(x)
    }
}
