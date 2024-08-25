mod spectrum;
mod ray_background;
mod ray_sphere;
mod surface_normals;

use std::{rc::Rc, sync::{Arc, RwLock}};

pub use spectrum::spectrum;
pub use ray_background::ray_background;
pub use ray_sphere::ray_sphere;
pub use surface_normals::surface_normals;

use crate::{object::{self, sphere, HitRecord, Hittable}, utils::Interval};

