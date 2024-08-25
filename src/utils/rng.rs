use std::ops::Range;

use rand::{thread_rng, Rng};

pub fn random_float() -> f32 {
    let mut rng = thread_rng();
    rng.gen_range(0.0f32..1.0f32)
}

pub fn random_float_range(range: Range<f32>) -> f32 {
    let mut rng = thread_rng();
    rng.gen_range(range)
}