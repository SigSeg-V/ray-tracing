use glam::Vec3A;

#[derive(Default, Clone, Copy)]
pub struct Ray {
    origin: Vec3A,
    direction: Vec3A,
    time: f32,
}

impl Ray { 
    pub fn new(origin: &Vec3A, direction: &Vec3A) -> Self {
        Self {
            origin: *origin,
            direction: *direction,
            time: 0.0,
        }
    }

    /// creates a new ray at fix point in time `time`
    pub fn new_time(origin: &Vec3A, direction: &Vec3A, time: f32) -> Self {
        Self {
            origin: *origin,
            direction: *direction,
            time,
        }

    }

    pub fn origin(&self) -> &Vec3A {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3A {
        &self.direction
    }
    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn at(&self, t: f32) -> Vec3A {
        self.origin + self.direction * t
    }
}