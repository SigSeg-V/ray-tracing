use image::ImageBuffer;
use crate::Color;
const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;


pub fn spectrum() -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let mut imgbuf  = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for (x, y, px) in imgbuf.enumerate_pixels_mut() {
        let pixel_color = Color::new(
            x as f32 / (IMAGE_HEIGHT - 1) as f32,
            y as f32 / (IMAGE_HEIGHT - 1) as f32,
            0.,
        );
        *px = image::Rgb(pixel_color.to_rgb());
    }
    imgbuf
}