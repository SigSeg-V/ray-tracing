use crate::{utils::Interval, ray::Ray, vec3::{Point3, Vec3}};

pub mod sphere;
pub mod world;
use enum_dispatch::enum_dispatch;
pub use sphere::Sphere;
pub use world::World;

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

#[enum_dispatch]
pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

#[enum_dispatch(Hittable)]
pub enum Object {
    World,
    Sphere,
}
