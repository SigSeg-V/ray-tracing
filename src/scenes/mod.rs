mod spectrum;
mod ray_background;
mod ray_sphere;
mod surface_normals;

pub use spectrum::spectrum;
pub use ray_background::ray_background;
pub use ray_sphere::ray_sphere;
pub use surface_normals::surface_normals;

use crate::{interval::Interval, object::Hittable};

#[derive(Clone)]
pub struct ObjectList<'a> {
    objects: Vec<&'a dyn Hittable>,
}

impl<'a> ObjectList<'a> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }

    pub fn from(object: &'a dyn Hittable) -> Self {
        Self { objects: vec![object] }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn push(&mut self, object: &'a dyn Hittable) {
        self.objects.push(object)
    }
}

impl Hittable for ObjectList<'_> {
    fn hit(&self, ray: &crate::ray::Ray, ray_t: Interval) -> Option<crate::object::HitRecord> {
        let mut record = None;

        let mut closest = ray_t.max;

        for object in self.objects.iter() {
            if let Some(rec) = (*object).hit(ray, Interval::from(ray_t.min, closest)) {
                record = Some(rec.clone());
                closest = rec.t;
            }
        }

        record
    }
}