use enum_dispatch::enum_dispatch;

use crate::{
    object::HitRecord,
    ray::Ray,
    utils::rng::random_float,
    vec3::{Color, Vec3},
};

#[enum_dispatch(Scatter)]
#[derive(Debug, Clone)]
pub enum Material {
    Diffuse,    // lambertian reflection
    Metallic,   // angle of incident == angle of reflection + fuzz
    Dielectric, // using snell's law
}

#[enum_dispatch]
pub trait Scatter {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Debug, Clone)]
pub struct Diffuse {
    albedo: Color,
}

impl Diffuse {
    pub fn new(albedo: &Color) -> Self {
        Self { albedo: *albedo }
    }
}

impl Scatter for Diffuse {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = record.normal + Vec3::random_unit();

        if scatter_direction.is_near_zero() {
            scatter_direction = record.normal;
        }

        let scattered = Ray::new(record.point, scatter_direction);
        Some((scattered, self.albedo))
    }
}

#[derive(Debug, Clone)]
pub struct Metallic {
    albedo: Color, // color of the reflection/meterial
    fuzz: f32,     // size of the radius of diffusion on the reflection
}

impl Metallic {
    pub fn new(albedo: &Color, fuzz: f32) -> Self {
        Self {
            albedo: *albedo,
            fuzz: fuzz.min(1.),
        }
    }
}

impl Scatter for Metallic {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let mut reflected = ray.direction().reflect(&record.normal);
        reflected = reflected.unit() + self.fuzz * Vec3::random_unit();

        let scattered = Ray::new(record.point, reflected);

        (scattered.direction().dot(&record.normal) > 0.).then_some((scattered, self.albedo))
    }
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    refractive_index: f32,
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        Self { refractive_index }
    }

    fn schlick_reflectance(&self, cos: f32, ri: f32) -> f32 {
        let mut r = (1. - self.refractive_index) / (1. + self.refractive_index);
        r *= r;
        r + (1. - r) * (1. - cos).powi(5)
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1., 1., 1.);
        let ri = if record.front_face {
            1. / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = ray.direction().unit();

        let cos_theta = (-unit_direction).dot(&record.normal);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        // must reflect the ray when sin(Theta') > 1 or with schlick's reflectance approximation
        let direction =
            if ri * sin_theta > 1. || self.schlick_reflectance(cos_theta, ri) > random_float() {
                unit_direction.reflect(&record.normal)
            } else {
                unit_direction.refract(&record.normal, ri)
            };
        let refracted_ray = Ray::new(record.point, direction);

        Some((refracted_ray, attenuation))
    }
}
