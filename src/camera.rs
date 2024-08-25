use core::f32;
use std::{cell::RefCell, rc::Rc};

use image::ImageBuffer;

use crate::{interval::Interval, object::Hittable, ray::Ray, vec3::{Color, Point3, Vec3}};

pub struct Camera {
    aspect_ratio: f32, // ratio of image width / height
    image_width: u32, // image width in px
    image_height: u32, // image height in px
    camera_pos: Point3, // center point of the camera
    px_top_left: Vec3, // location of the top left pixel in the viewport
    px_dx: Vec3, // distance between pixels in the x axis in viewport
    px_dy: Vec3, // distance between pixels in the y axis in viewport
    pub imgbuf: Rc<RefCell<ImageBuffer<image::Rgb<u8>, Vec<u8>>>> // buffer for the raw image
}

impl<'a> Camera {
    pub fn render(&mut self, world: &'a dyn Hittable) {
        for (x, y, px) in self.imgbuf.borrow_mut().enumerate_pixels_mut() {
            let px_center = self.px_top_left + x as f32 * self.px_dx + y as f32 * self.px_dy;
            let ray_direction = px_center - self.camera_pos;
            let ray = Ray::new(self.camera_pos,ray_direction);

            let color = Self::ray_color(&ray, world.clone());
            *px = image::Rgb(color.to_rgb());
        }
    }

    pub fn from(aspect_ratio: f32, image_width: u32, ) -> Camera {
        // calc img height, it has to be at least 1 px
        let image_height = (image_width as f32 / aspect_ratio) as u32;
        let image_height = if image_height > 0 { image_height } else { 1 };

        // Camera
        // real aspect ratio differs because of flooring when converting to a u32
        let real_aspect_ratio = image_width as f32 / image_height as f32;
        let viewport_height = 2.0f32;
        // 
        let viewport_width = viewport_height * (real_aspect_ratio);
        let focal_length = 1.0f32;
        let camera_pos = Point3::new(0., 0., 0.);

        // direction to render pixels in
        let viewport_x = Vec3::new(viewport_width, 0., 0.);
        let viewport_y = Vec3::new(0., -viewport_height, 0.); // invert viewport height to start top and go to bottom

        // distance between pixel center points
        let px_dx = viewport_x / image_width as f32;
        let px_dy = viewport_y / image_height as f32;

        // calc the location of the top left pixel
        let viewport_top_left = camera_pos - Vec3::new(0., 0., focal_length) - viewport_x/2. - viewport_y/2.;
        let px_top_left = viewport_top_left + 0.5 * (px_dx + px_dy);

        let imgbuf  = ImageBuffer::new(image_width, image_height);

        Camera{
            aspect_ratio,
            image_width,
            image_height,
            camera_pos,
            px_top_left,
            px_dx,
            px_dy,
            imgbuf: Rc::from(RefCell::new(imgbuf)),
        }
    }

    fn ray_color(ray: &Ray, world: &'a dyn Hittable) -> Color {
        if let Some(record) = world.hit(ray, Interval::from(0., f32::INFINITY)) {
            let colors = record.normal + Vec3::new(1., 1., 1.);
            return 0.5 * (Color::new(colors.x(), colors.y(), colors.z()));
        }
        
        // background color
        let unit_direction = ray.direction().unit();
        let scale = 0.5 * (unit_direction.y() + 1.); // blend in the y axis, midpoint halfway down

        //             // white color                           ligh blue color
        (1. - scale) * Color::new(1., 1., 1.) + scale * Color::new(0.5, 0.7, 1.)
    }
}