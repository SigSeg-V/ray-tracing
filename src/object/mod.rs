use crate::{interval::Interval, ray::Ray, vec3::{Point3, Vec3}};

pub mod sphere;
pub use sphere::Sphere;

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    /// sets the hit record
    /// BEWARE: `outward_normal` MUST be normalised
    pub fn new(t: f32, ray: &Ray, point: Point3, outward_normal: Vec3) -> Self {
        let front_face = ray.direction().dot(&outward_normal) < 0.;
        let normal = if front_face { outward_normal } else { -outward_normal };
        Self { point, normal, t, front_face }
    }
}   


pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}