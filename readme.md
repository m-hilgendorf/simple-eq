# simple-eq 
## A Simple Audio Equalizer 

`simple-eq` is a crate that implements a simple audio equalizer in Rust. It supports a maximum of 32 filter bands. 

## Usage: 

```rust 
use simple_eq::*; 
use simple_eq::design::Curve; 

// create an EQ for a given sample rate
let sample_rate = 48.0e3;
let mut eq = Equalizer::new(sample_rate);

// set the filters to something, this will remove the bypass
eq.set(0, Curve::Lowshelf, 100.0, 1.0, 12.0);
eq.set(1, Curve::Notch, 1.0e3, 10.0, 0.0); 
eq.set(2, Curve::Highshelf, 5.0e4, 4.0, 0.0);

// process a signal with it 
for sample in signal {
    let output = eq.process(sample);
}

// to bypass a filter 
eq.set_bypass(1, true);
```

