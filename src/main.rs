mod error;
mod prelude;
mod vec3;
mod scenes;
mod ray;

use vec3::Color;

use crate::prelude::*;

fn main() -> Result<()> {
    // creating a PPM image
    let imgbuf = scenes::ray_background();
    imgbuf.save("image.png").unwrap();
    Ok(())
}
