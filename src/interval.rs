use core::f32;

pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new() -> Self {
        Interval::from(f32::INFINITY, f32::NEG_INFINITY)
    }

    pub fn from(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn universe() -> Self {
        Interval::from(f32::NEG_INFINITY, f32::INFINITY)
    }

    pub fn empty() -> Self {
        Interval::new()
    }
}