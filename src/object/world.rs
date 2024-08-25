use std::sync::{Arc, RwLock};

use crate::utils::Interval;

use super::{Hittable, Object};


#[derive(Clone)]
pub struct World {
    objects: Arc<RwLock<Vec<Object>>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Arc::from(RwLock::new(Vec::new()))
        }
    }

    pub fn from(object: Object) -> Self {
        Self { objects: Arc::from(RwLock::new(vec![object])) }
    }

    pub fn clear(&mut self) {
        self.objects.write().unwrap().clear()
    }

    pub fn push(&mut self, object: Object) {
        self.objects.write().unwrap().push(object)
    }
}

impl Hittable for World {
    fn hit(&self, ray: &crate::ray::Ray, ray_t: Interval) -> Option<crate::object::HitRecord> {
        let mut record = None;

        let mut closest = ray_t.max;

        for object in self.objects.read().unwrap().iter() {
            if let Some(rec) = object.hit(ray, Interval::from(ray_t.min, closest)) {
                record = Some(rec.clone());
                closest = rec.t;
            }
        }

        record
    }
}