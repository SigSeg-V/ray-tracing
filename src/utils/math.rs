use core::f32;

pub fn discriminant(a: f32, h: f32, c: f32) -> f32 {
    h * h - a * c
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

// returns the real solution facing the camera, or None if no solution is found
pub fn quadratic_formula(a: f32, h: f32, c: f32) -> Option<(f32, f32)> {
    let d = discriminant(a, h, c);
    if d < 0. {
        return None;
    }
    let sqrtd = d.sqrt();

    Some(((h - sqrtd) / a, (h + sqrtd) / a))
}

pub fn deg_to_rad(deg: f32) -> f32 {
    deg * f32::consts::PI/180.
}