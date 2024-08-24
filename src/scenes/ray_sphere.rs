use crate::math;
use crate::vec3::{Color, Point3, Vec3};
use crate::ray::Ray;
use image::ImageBuffer;

pub fn ray_sphere() -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let aspect_ratio = 16./9.;
    let image_width: u32 = 400;
    
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

    let mut imgbuf  = ImageBuffer::new(image_width, image_height);

    for (x, y, px) in imgbuf.enumerate_pixels_mut() {
        let px_center = px_top_left + x as f32 * px_dx + y as f32 * px_dy;
        let ray_direction = px_center - camera_pos;
        let ray = Ray::new(camera_pos,ray_direction);

        let color = ray_color(&ray);
        *px = image::Rgb(color.to_rgb());
    }

    imgbuf
}

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(&Point3::new(0., 0., 1.0), 0.5, ray) {
        return Color::new(1., 0., 1.);
    }
    
    // background color
    let unit_direction = ray.direction().unit();
    let scale = 0.5 * (unit_direction.y() + 1.); // blend in the y axis, midpoint halfway down

    //             // white color                           ligh blue color
    (1. - scale) * Color::new(1., 1., 1.) + scale * Color::new(0.5, 0.7, 1.)
}

fn hit_sphere(center: &Point3, radius: f32, ray: &Ray) -> bool {
    
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

    let camera_to_center = *center - *ray.origin();

    let a = ray.direction().dot(ray.direction());
    let b = -2. * camera_to_center.dot(ray.direction());
    let c = camera_to_center.dot(&camera_to_center) - radius * radius;

    math::discriminant_intersections(a, b, c).is_some()
}