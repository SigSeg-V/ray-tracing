#![allow(warnings)]
mod camera;
mod error;
mod material;
mod object;
mod prelude;
mod ray;
mod scenes;
mod utils;
mod vec3;

use vec3::Color;

use crate::prelude::*;

fn main() -> Result<()> {
    // creating a PPM image
    let imgbuf = scenes::large_scene();
    imgbuf.save("image.png").unwrap();
    Ok(())
}
