use glam::Vec3A;
use crate::object::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::utils::Interval;

#[derive(Debug, Default)]
struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    /// creates an empty bounding box
    pub fn new() -> Self {
        Self {..Default::default()}
    }

    pub fn from(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_point(a: &Vec3A, b: &Vec3A) -> Self {
        Self {
            x: if a.x < b.x {
                Interval::from(a.x, b.x)
            } else {
                Interval::from(b.x, a.x)
            },
            y: if a.y < b.y {
                Interval::from(a.y, b.y)
            } else {
                Interval::from(b.y, a.y)
            },
            z: if a.z < b.z {
                Interval::from(a.z, b.z)
            } else {
                Interval::from(b.z, a.z)
            },
        }
    }

    pub fn axis_interval(&self, n: i32) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }


}

impl Hittable for AABB {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        todo!()
    }
}