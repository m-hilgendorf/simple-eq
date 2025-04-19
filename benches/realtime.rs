use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rand::Rng;
use simple_eq::parameters::Curve;
use simple_eq::*;

fn exp_decay(n: usize) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    black_box(
        (0..n)
            .map(|n| n as f32)
            .map(|a| a * rng.r#gen::<f32>())
            .collect(),
    )
}

fn dsp(c: &mut Criterion) {
    let mut signal = exp_decay(1024);
    let mut eq = Equalizer::new(48.0e3);
    for i in 0..32 {
        eq.set(i, Curve::Peak, 1.0e3, 10.0, -12.0);
    }

    c.bench_function("DSP Loop", |b| {
        b.iter(|| eq.process_buffer(&mut signal));
    });

    eq.reset();
    c.bench_function("Sample Accurate Automation", |b| {
        b.iter(|| {
            for sample in signal.iter_mut() {
                for i in 0..32 {
                    eq.set_frequency(i, black_box(1.0e3));
                }
                *sample = eq.process(*sample);
            }
        });
    });
}

criterion_group!(benches, dsp);
criterion_main!(benches);
