use glam::Vec3A;
use crate::camera::Camera;
use crate::material::{self, Dielectric, Diffuse, Material, Metallic};
use crate::object::{self, Object, Sphere, World};
use crate::utils::rng::{random_float, random_float_range};
use image::ImageBuffer;
use rand::{thread_rng, Rng};

use crate::prelude::*;

pub fn large_scene() -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let aspect_ratio = 16. / 9.;
    let image_width: u32 = 1920;
    let fov = 25.;
    let focus_distance = 10.;
    let depth_of_field_angle = 0.5;
    let num_samples = 16;
    let max_bounce_depth = 16;
    let camera_pos = Vec3A::new(13., 2., 3.);
    let target = Vec3A::new(0., 0., 0.);
    let direction = camera_pos - target;
    let camera_up = Vec3A::new(0., 1., 0.);

    // world
    let mut world = World::new();

    // floor
    let floor_material = Material::Diffuse(Diffuse::new(&Vec3A::new(0.5, 0.5, 0.5)));
    world.push(Object::Sphere(Sphere::new(
        &Vec3A::new(0., -1000., 0.),
        1000.,
        floor_material,
    )));

    // generate random spheres
    let mut rng = thread_rng();
    for x in -15..15 {
        for y in -15..15 {
            let center = Vec3A::new(
                x as f32 + 0.9 * random_float(),
                0.2,
                y as f32 + 0.9 * random_float(),
            );
            let random_material = {
                match rng.gen_range(0..100) {
                    0..=79 => {
                        // diffuse material
                        let albedo = Vec3A::new_random() * Vec3A::new_random();
                        Material::Diffuse(Diffuse::new(&albedo))
                    }
                    80..=89 => {
                        // metal
                        let albedo = Vec3A::new_random() * Vec3A::new_random();
                        let fuzz = random_float_range(0.0f32..0.5f32);
                        Material::Metallic(Metallic::new(&albedo, fuzz))
                    }
                    90..=99 => {
                        //glass
                        Material::Dielectric(Dielectric::new(1.5))
                    }
                    _ => unreachable!(),
                }
            };

            let end_center = center + Vec3A::new(0., random_float_range(0.0..0.5), 0.);
            let obj = Object::Sphere(Sphere::new_translated(&center, &end_center, 0.2, random_material));
            world.push(obj);
        }
    }

    let glass_material = Material::Dielectric(Dielectric::new(1.5));
    let diffuse_material = Material::Diffuse(Diffuse::new(&Vec3A::new(0.4, 0.3, 0.2)));
    let metal_material =
        Material::Metallic(Metallic::new(&Vec3A::new(0.7, 0.6, 0.5), 0.));

    let big_spheres = vec![
        Object::Sphere(Sphere::new(
            &Vec3A::new(0., 1., 0.),
            1.0,
            glass_material,
        )),
        Object::Sphere(Sphere::new(
            &Vec3A::new(4., 1., 0.),
            1.0,
            metal_material,
        )),
        Object::Sphere(Sphere::new(
            &Vec3A::new(-4., 1., 0.),
            1.0,
            diffuse_material,
        )),
    ];

    for obj in big_spheres {
        world.push(obj);
    }

    let camera = Camera::from(
        aspect_ratio,
        image_width,
        num_samples,
        max_bounce_depth,
        fov,
        focus_distance,
        depth_of_field_angle,
        direction,
        camera_up,
        camera_pos,
    );

    camera.render(world)
}
