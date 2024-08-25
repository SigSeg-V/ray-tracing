use std::cell::RefCell;
use std::rc::Rc;

use crate::camera::Camera;
use crate::object:: Sphere;
use crate::vec3::Point3;
use image::ImageBuffer;

use super::ObjectList;

pub fn surface_normals() -> Rc<RefCell<ImageBuffer<image::Rgb<u8>, Vec<u8>>>> {
    let aspect_ratio = 16./9.;
    let image_width: u32 = 1920;

    // world
    let mut world = ObjectList::new();
    let sphere1 = Sphere::new(
        Point3::new(0., 0., -1.), 0.5
    );
    world.push(&sphere1);

    let sphere2 = Sphere::new(
        Point3::new(0., -100.5, -1.), 100.
    );
    world.push(&sphere2);

    let mut camera = Camera::from(aspect_ratio, image_width);

    camera.render(&world);

    camera.imgbuf
}