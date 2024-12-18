use std::{
    fmt::Display,
    ops::Range,
};
use glam::Vec3A;

use crate::utils::{
    rng::{random_float, random_float_range},
    Interval,
};

/// extensions to the `glam::Vec3A`
pub trait Vec3Ext {
    fn new_random() -> Self;
    fn new_random_range(range: Range<f32>) -> Self;
    fn new_random_in_unit_sphere() -> Self; // returns a unit sphere or circle on the axes that are not 0
    fn new_random_in_unit_dim(dimensions: &Vec3A) -> Self;
    fn new_random_in_unit_circle_xy() -> Vec3A;
    fn new_random_unit() -> Self;
    fn new_random_on_hemisphere(normal: &Vec3A) -> Self;
    fn is_near_zero(&self) -> bool;
}

/// extensions to the `glam::Vec3A`
impl Vec3Ext for Vec3A {
    fn new_random() -> Self {
        Self::new(random_float(), random_float(), random_float())
    }

    fn new_random_range(range: Range<f32>) -> Self {
        Self::new(
            random_float_range(range.clone()),
            random_float_range(range.clone()),
            random_float_range(range),
        )
    }
    fn new_random_in_unit_sphere() -> Self {
        loop {
            let v = Self::new_random_range(-1.0f32..1.0f32);
            if v.length_squared() < 1. {
                return v;
            }
        }
    }

    // returns a unit sphere or circle on the axes that are not 0
    fn new_random_in_unit_dim(dimensions: &Vec3A) -> Self {
        // cap to unit
        let dimensions = Vec3A::new(
            1.0f32.min(dimensions.x.max(0.)),
            1.0f32.min(dimensions.y.max(0.)),
            1.0f32.min(dimensions.z.max(0.)),
        );

        Self::new_random_in_unit_sphere() * dimensions
    }

    fn new_random_in_unit_circle_xy() -> Self {
        Self::new_random_in_unit_dim(&Vec3A::new(1., 1., 0.))
    }

    fn new_random_unit() -> Self {
        Self::new_random_in_unit_sphere().normalize()
    }

    fn new_random_on_hemisphere(normal: &Vec3A) -> Self {
        let on_unit_sphere = Self::new_random_unit();
        if on_unit_sphere.dot(*normal) > 0. {
            // in the same hemisphere as normal
            return on_unit_sphere;
        }
        -on_unit_sphere
    }

    fn is_near_zero(&self) -> bool {
        const LIMIT: f32 = 1e-6;
        self.x.abs() < LIMIT && self.y.abs() < LIMIT && self.z.abs() < LIMIT
    }
}

/// extensions to the `glam::Vec3A`
pub trait Color {
    fn to_rgb(&self) -> [u8; 3];
    fn to_gamma(&self) -> Self;
    fn linear_to_gamma(linear: f32) -> f32;
}

/// extensions to the `glam::Vec3A`
impl Color for Vec3A {
    fn to_rgb(&self) -> [u8; 3] {
        let intensity = Interval::from(0., 0.999);
        [
            (intensity.clamp(self.x) * 256.) as u8,
            (intensity.clamp(self.y) * 256.) as u8,
            (intensity.clamp(self.z) * 256.) as u8,
        ]
    }

    fn to_gamma(&self) -> Self {
        Vec3A::new(
            Self::linear_to_gamma(self.x),
            Self::linear_to_gamma(self.y),
            Self::linear_to_gamma(self.z),
        )
    }

    fn linear_to_gamma(linear: f32) -> f32 {
        if linear > 0. {
            linear.sqrt()
        } else {
            0.
        }
    }
}


