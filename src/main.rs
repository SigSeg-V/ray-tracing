mod error;
mod prelude;
mod vec3;
mod scenes;
mod ray;
mod object;
mod utils;
mod camera;

use vec3::Color;

use crate::prelude::*;

fn main() -> Result<()> {
    // creating a PPM image
    let imgbuf = scenes::surface_normals();
    imgbuf.save("image.png").unwrap();
    Ok(())
}
