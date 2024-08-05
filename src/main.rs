mod error;
mod prelude;
mod vec3;

use std::io::Write;

use vec3::Color;

use crate::prelude::*;

const IMAGE_WIDTH:i32 = 256;
const IMAGE_HEIGHT:i32 = 256;
const MAX_INTENSITY:i32 = 255;

fn main() -> Result<()> {
    // creating a PPM image
    let mut image = String::new();

    // denoting ascii colours
    image.push_str("P3\n");
    // number of columns and rows for the pixels
    image.push_str(&fmt!("{IMAGE_WIDTH} {IMAGE_HEIGHT}\n"));
    // defining the color intensity (0-255)
    image.push_str(&fmt!("{MAX_INTENSITY}\n"));

    for y in 0..IMAGE_HEIGHT {
        eprint!("\rScan lines remaining: {}", IMAGE_HEIGHT-y);
        std::io::stderr().flush()?;
        for x in 0..IMAGE_WIDTH {
            // calculate RGB with a 0..=1.0 intensity, then scale to our format's integral intensity

            let pixel_color = Color::new(x as f32/(IMAGE_HEIGHT-1) as f32, y as f32/(IMAGE_HEIGHT-1) as f32, 0.);

            // add pixel row to image
            image.push_str(&(pixel_color.to_string() + "\n"));
        }
    }

    eprintln!("\rDone!                                            ");
    print!("{}", image);

    Ok(())
}