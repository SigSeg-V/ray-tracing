use crate::{interval::Interval, math, vec3::Point3};

use super::{HitRecord, Hittable};

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, ray_t: Interval) -> Option<HitRecord> {
        // Eq for sphere is x^2 + y^2 + z^2 = r^2
        // To put the sphere in an arbitrary point (C) in space is :
        // (C_x - x)^2 + (C_y - y)^2 + (C_z - z)^2 = r^2
        // This can be expressed in vector form:
        // (C - P).(C - P) = r^2
        // Where C is the center of the sphere and P is the point of origin
        // of the ray. We can cast the ray with P(t) = Q + td (Q = origin which is the camera,
        // d is direction) to get to:
        // (C − (Q + td)).(C − (Q + td)) = r^2
        // This can be arranged to a quardatic equation:
        // t^2d.d − 2td.(C − Q) + (C − Q).(C − Q) − r^2 = 0
        // Because we want to know t which gives us the distance from Q where the ray intersects
        // the sphere, we can extract the constants:
        // a = d.d
        // b = -2d.(C - Q)
        // c = (C - Q).(C - Q) - r^2
        // Where:
        // d -> direction of the ray
        // Q -> origin of the ray
        // C -> center point of the sphere
        // r -> radius of the sphere
        // (C - Q) -> unnormalised vector from the center of the sphere to the origin of the ray (camera)
        // We then use the quadratic formula to find t (or discriminant to find if there is at least one
        // intersection in the basic, unnormaled cased)

        let camera_to_center = self.center - *ray.origin();

        let a = ray.direction().len_sq();
        let h = camera_to_center.dot(ray.direction());
        let c = camera_to_center.len_sq() - self.radius * self.radius;

        if let Some((minus, plus)) = math::quadratic_formula(a, h, c) {
            // find the closest root to the camera that is within tmin and tmax
            let root = match ray_t {
                x if x.surrounds(minus) => minus,
                x if x.surrounds(plus) => plus,
                _ => return None,
            };

            // assemble hit record
            let point = ray.at(root);
            let record = HitRecord::new(
                root,
                ray,
                point,
                (point - self.center)/self.radius,
            );

            return Some(record)
        }

        None
    }
}
