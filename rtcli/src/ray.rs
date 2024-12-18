use glam::Vec3A;

#[derive(Default, Clone, Copy)]
pub struct Ray {
    origin: Vec3A,
    direction: Vec3A,
}

impl Ray { 
    pub fn new(origin: &Vec3A, direction: &Vec3A) -> Self {
        Self {
            origin: *origin,
            direction: *direction,
        }
    }

    pub fn origin(&self) -> &Vec3A {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3A {
        &self.direction
    }

    pub fn at(&self, t: f32) -> Vec3A {
        self.origin + self.direction * t
    }
}