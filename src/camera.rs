use core::f32;
use std::{cell::RefCell, rc::Rc, sync::{Arc, Mutex, RwLock}, time::Instant};

use image::ImageBuffer;
use rayon::prelude::*;

use crate::{object::{Hittable, World}, ray::Ray, utils::{self, rng::random_float, Interval}, vec3::{Color, Point3, Vec3}};

pub struct Camera {
    aspect_ratio: f32, // ratio of image width / height
    image_width: u32, // image width in px
    image_height: u32, // image height in px
    camera_pos: Point3, // center point of the camera
    px_top_left: Vec3, // location of the top left pixel in the viewport
    px_dx: Vec3, // distance between pixels in the x axis in viewport
    px_dy: Vec3, // distance between pixels in the y axis in viewport
    num_samples: u32, // number of samples taken of each pixel in the frame
    px_sample_scale: f32, // Color scale factor for a sum of pixel samples
    max_bounce_depth: u32, // maximum number of bounces a ray can perform before expiring
}

impl Camera {
    pub fn render(self, world: World) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let mut imgbuf: ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::new(self.image_width, self.image_height);

        // currently parallelisation this is extremely slow
        // imgbuf.par_enumerate_pixels_mut().for_each(|(x,y,px)| {
        //     let mut color = Color::new(0., 0., 0.);
        //     for _ in 0..self.num_samples {
        //         let ray = self.get_ray(x, y);
        //         color += Self::ray_color(&ray, &world);
        //     }
            
        //     *px = image::Rgb((color * self.px_sample_scale).to_rgb());
        // });

        let time_before = Instant::now();
        imgbuf.enumerate_pixels_mut().for_each(|(x,y,px)| {
            let mut color = Color::new(0., 0., 0.);
            for _ in 0..self.num_samples {
                let ray = self.get_ray(x, y);
                color += Self::ray_color(&ray, &world, self.max_bounce_depth);
            }
            
            *px = image::Rgb((color * self.px_sample_scale).to_gamma().to_rgb());
        });
        let time_after = Instant::now();
        let time = time_after - time_before;

        println!("Time taken\nNormal\t{}", time.as_millis());

        imgbuf
    }

    pub fn from(aspect_ratio: f32, image_width: u32, num_samples: u32, max_bounce_depth: u32) -> Camera {
        // calc img height, it has to be at least 1 px
        let image_height = (image_width as f32 / aspect_ratio) as u32;
        let image_height = if image_height > 0 { image_height } else { 1 };

        // Camera
        // real aspect ratio differs because of flooring when converting to a u32
        let real_aspect_ratio = image_width as f32 / image_height as f32;
        let viewport_height = 2.0f32;
        let viewport_width = viewport_height * (real_aspect_ratio);
        let focal_length = 1.0f32;
        let camera_pos = Point3::new(0., 0., 0.);

        // sampling 
        let px_sample_scale = 1./num_samples as f32;

        // direction to render pixels in
        let viewport_x = Vec3::new(viewport_width, 0., 0.);
        let viewport_y = Vec3::new(0., -viewport_height, 0.); // invert viewport height to start top and go to bottom

        // distance between pixel center points
        let px_dx = viewport_x / image_width as f32;
        let px_dy = viewport_y / image_height as f32;

        // calc the location of the top left pixel
        let viewport_top_left = camera_pos - Vec3::new(0., 0., focal_length) - viewport_x/2. - viewport_y/2.;
        let px_top_left = viewport_top_left + 0.5 * (px_dx + px_dy);


        Camera{
            aspect_ratio,
            image_width,
            image_height,
            camera_pos,
            px_top_left,
            px_dx,
            px_dy,
            num_samples,
            px_sample_scale,
            max_bounce_depth
        }
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let x = x as f32;
        let y = y as f32;

        let offset = Self::sample_square();
        let px_sample = self.px_top_left + ((x + offset.x()) * self.px_dx) + ((y + offset.y()) * self.px_dy);

        let direction = px_sample - self.camera_pos;
        return Ray::new(self.camera_pos, direction);
    }

    fn ray_color(ray: &Ray, world: &World, num_bounces: u32) -> Color {
        if num_bounces == 0 { // hit recursion limit
            return Color::new(0., 0., 0.);
        }
        if let Some(record) = world.hit(ray, Interval::from(0.001, f32::INFINITY)) { // from 0.001 to fix shadow acne, where rays bounce many times off same point
            let direction = Vec3::random_on_hemisphere(&record.normal) + record.normal; // lambertian distribution to mimic random reflection
            return 0.5 * Self::ray_color(&Ray::new(record.point, direction), world, num_bounces - 1); // diffuse 50% of the color from a bounce
        }
        
        // background color
        let unit_direction = ray.direction().unit();
        let scale = 0.5 * (unit_direction.y() + 1.); // blend in the y axis, midpoint halfway down

        //             // white color                           ligh blue color
        (1. - scale) * Color::new(1., 1., 1.) + scale * Color::new(0.5, 0.7, 1.)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(utils::rng::random_float() - 0.5, random_float() - 0.5, 0.)
    }
}
