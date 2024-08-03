mod error;
mod prelude;

use std::io::Write;

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

    for j in 0..IMAGE_HEIGHT {
        eprint!("\rScan lines remaining: {}", IMAGE_HEIGHT-j);
        std::io::stderr().flush()?;
        for i in 0..IMAGE_WIDTH {
            // calculate RGB with a 0..=1.0 intensity, then scale to our format's integral intensity
            let r = i as f64/(IMAGE_HEIGHT-1) as f64;
            let g = j as f64/(IMAGE_HEIGHT-1) as f64;
            let b = 0.;

            let r = (r * (MAX_INTENSITY as f64)).clamp(0., MAX_INTENSITY as f64) as u8;
            let g = (g * (MAX_INTENSITY as f64)).clamp(0., MAX_INTENSITY as f64) as u8;
            let b = (b * (MAX_INTENSITY as f64)).clamp(0., MAX_INTENSITY as f64) as u8;

            image.push_str(&fmt!("{r} {g} {b}\n"));
        }
    }

    eprintln!("\rDone!                                            ");
    print!("{}", image);

    Ok(())
}