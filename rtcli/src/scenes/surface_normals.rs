use crate::camera::Camera;
use crate::material::{Dielectric, Diffuse, Material, Metallic};
use crate::object::{Object, Sphere, World};
use crate::vec3::{Color, Point3, Vec3};
use image::ImageBuffer;

pub fn surface_normals() -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let aspect_ratio = 16. / 9.;
    let image_width: u32 = 1920;
    let fov = 65.;
    let focus_distance = 3.4;
    let depth_of_field_angle = 10.;
    let num_samples = 32;
    let max_bounce_depth = 16;
    let camera_pos = Point3::new(-2., 2., 1.);
    let target = Point3::new(0., 0., -1.);
    let direction = camera_pos - target;
    let camera_up = Vec3::new(0., 1., 0.);

    // world
    let mut world = World::new();

    let mat_center = Material::Diffuse(Diffuse::new(&Color::new(0.1, 0.2, 0.5)));
    let mat_grnd = Material::Diffuse(Diffuse::new(&Color::new(0.8, 0.8, 0.0)));
    let mat_left = Material::Dielectric(Dielectric::new(1.5));
    let mat_bubble = Material::Dielectric(Dielectric::new(1. / 1.5));
    let mat_right = Material::Metallic(Metallic::new(&Color::new(0.8, 0.6, 0.2), 0.0));

    world.push(Object::Sphere(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        mat_grnd,
    )));

    world.push(Object::Sphere(Sphere::new(
        Point3::new(0., 0., -1.2),
        0.5,
        mat_center,
    )));

    world.push(Object::Sphere(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        mat_left,
    )));

    world.push(Object::Sphere(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.4,
        mat_bubble,
    )));

    world.push(Object::Sphere(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        mat_right,
    )));
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
