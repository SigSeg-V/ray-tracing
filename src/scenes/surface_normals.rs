use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use crate::camera::Camera;
use crate::object::{ Object, Sphere, World};
use crate::vec3::Point3;
use image::ImageBuffer;


pub fn surface_normals() -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let aspect_ratio = 16./9.;
    let image_width: u32 = 1920;
    let num_samples = 100;

    // world
    let mut world = World::new();

    world.push(Object::Sphere(Sphere::new(
        Point3::new(0., 0., -1.), 0.5
    )));

    world.push(Object::Sphere(Sphere::new(
        Point3::new(0., -100.5, -1.), 100.
    )));

    let mut camera = Camera::from(aspect_ratio, image_width, 300);

    camera.render(world)
}