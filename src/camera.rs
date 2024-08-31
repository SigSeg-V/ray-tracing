use core::f32;
use std::time::Instant;

use image::ImageBuffer;
use rayon::prelude::*;

use crate::{
    material::Scatter,
    object::{Hittable, World},
    ray::Ray,
    utils::{self, math, rng::random_float, Interval},
    vec3::{Color, Point3, Vec3},
};

pub struct Camera {
    aspect_ratio: f32,                      // ratio of image width / height
    image_width: u32,                       // image width in px
    image_height: u32,                      // image height in px
    camera_pos: Point3,                     // center point of the camera
    fov: f32,                               // vertical fov of the camera
    direction: Vec3, // unit vector for the direction the camera is pointing in
    camera_up: Vec3, // Camera-relative up direction
    camera_basis_frame: (Vec3, Vec3, Vec3), // camera basis frame
    px_top_left: Vec3, // location of the top left pixel in the viewport
    px_dx: Vec3,     // distance between pixels in the x axis in viewport
    px_dy: Vec3,     // distance between pixels in the y axis in viewport
    num_samples: u32, // number of samples taken of each pixel in the frame
    px_sample_scale: f32, // Color scale factor for a sum of pixel samples
    max_bounce_depth: u32, // maximum number of bounces a ray can perform before expiring
}

impl Camera {
    pub fn render(self, world: World) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let mut imgbuf: ImageBuffer<image::Rgb<u8>, Vec<u8>> =
            ImageBuffer::new(self.image_width, self.image_height);

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
        imgbuf.enumerate_pixels_mut().for_each(|(x, y, px)| {
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

    pub fn from(
        aspect_ratio: f32,
        image_width: u32,
        num_samples: u32,
        max_bounce_depth: u32,
        fov: f32,
        direction: Vec3,
        camera_up: Vec3,
        camera_pos: Vec3,
    ) -> Camera {
        // calc img height, it has to be at least 1 px
        let image_height = (image_width as f32 / aspect_ratio) as u32;
        let image_height = if image_height > 0 { image_height } else { 1 };

        // Camera
        // real aspect ratio differs because of flooring when converting to a u32
        let real_aspect_ratio = image_width as f32 / image_height as f32;
        let focal_length = direction.len();
        // calc viewport height from fov
        let theta = math::deg_to_rad(fov);
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h * focal_length;
        let viewport_width = viewport_height * (real_aspect_ratio);

        // init camera basis frame
        let w = direction.unit();
        let u = camera_up.cross(&w).unit();
        let v = w.cross(&u);
        let camera_basis_frame = (w, u, v);

        // sampling
        let px_sample_scale = 1. / num_samples as f32;

        // direction to render pixels in
        let viewport_x = viewport_width * u;
        let viewport_y = viewport_height * -v; // invert viewport height to start top and go to bottom

        // distance between pixel center points
        let px_dx = viewport_x / image_width as f32;
        let px_dy = viewport_y / image_height as f32;

        // calc the location of the top left pixel
        let viewport_top_left = camera_pos - focal_length * w - viewport_x / 2. - viewport_y / 2.;
        let px_top_left = viewport_top_left + 0.5 * (px_dx + px_dy);

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            camera_pos,
            px_top_left,
            px_dx,
            px_dy,
            num_samples,
            px_sample_scale,
            max_bounce_depth,
            fov,
            direction,
            camera_up,
            camera_basis_frame,
        }
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let x = x as f32;
        let y = y as f32;

        let offset = Self::sample_square();
        let px_sample =
            self.px_top_left + ((x + offset.x()) * self.px_dx) + ((y + offset.y()) * self.px_dy);

        let direction = px_sample - self.camera_pos;
        return Ray::new(self.camera_pos, direction);
    }

    fn ray_color(ray: &Ray, world: &World, num_bounces: u32) -> Color {
        if num_bounces == 0 {
            // hit recursion limit
            return Color::new(0., 0., 0.);
        }
        if let Some(record) = world.hit(ray, Interval::from(0.001, f32::INFINITY)) {
            // from 0.001 to fix shadow acne, where rays bounce many times off same point

            if let Some((scattered_ray, attenuation)) = record.material.scatter(ray, &record) {
                return attenuation * Self::ray_color(&scattered_ray, world, num_bounces - 1);
            };

            return Color::new(0., 0., 0.);
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
