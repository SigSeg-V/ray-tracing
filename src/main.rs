mod error;
mod prelude;
mod vec3;
mod scenes;
mod ray;
mod math;
mod object;
mod interval;
mod camera;

use vec3::Color;

use crate::prelude::*;

fn main() -> Result<()> {
    // creating a PPM image
    let imgbuf = scenes::surface_normals();
    imgbuf.borrow().save("image.png").unwrap();
    Ok(())
}
