pub fn discriminant(a: f32, b: f32, c: f32) -> f32 {
    b * b - 4. * a * c
}

pub fn discriminant_intersections(a: f32, b: f32, c: f32) -> Option<u8> {
    let d = discriminant(a, b, c);
    if  d > 0. {
        Some(2)
    } else if d == 0. {
        Some(1)
    } else {
        None
    }
}