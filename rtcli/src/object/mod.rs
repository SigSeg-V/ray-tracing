use crate::{
    material::{Material, Metallic},
    ray::Ray,
    utils::Interval,
};
use glam::Vec3A;
use image::codecs::png::FilterType::Paeth;

pub mod sphere;
pub mod world;
pub use sphere::Sphere;
pub use world::World;

#[derive(Clone)]
pub struct HitRecord {
    pub point: Vec3A,
    pub normal: Vec3A,
    pub t: f32,
    pub front_face: bool,
    pub material: Material,
}

impl HitRecord {
    /// sets the hit record
    /// BEWARE: `outward_normal` MUST be normalised
    pub fn new(t: f32, ray: &Ray, point: Vec3A, outward_normal: Vec3A, material: Material) -> Self {
        let front_face = ray.direction().dot(outward_normal) < 0.;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

pub enum Object {
    World(World),
    Sphere(Sphere),
}

impl Hittable for Object {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        match &self {
            Object::World(e) => e.hit(ray, ray_t),
            Object::Sphere(e) => e.hit(ray, ray_t),
        }
    }
}