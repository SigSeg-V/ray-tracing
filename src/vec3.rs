use std::{
    fmt::Display,
    ops::{Add, AddAssign, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Range, Sub, SubAssign},
};

use rand::random;

use crate::utils::{
    rng::{random_float, random_float_range},
    Interval,
};

#[derive(Default, Copy, Clone, Debug)]
pub struct Vec3(f32, f32, f32);

impl Vec3 {
    pub fn len_sq(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn len(&self) -> f32 {
        self.len_sq().sqrt()
    }

    // creates new Vec3
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3(x, y, z)
    }

    // dot product
    pub fn dot(&self, rhs: &Vec3) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    // cross product
    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.len()
    }

    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }

    pub fn z(&self) -> f32 {
        self.2
    }

    pub fn random() -> Self {
        Self(random_float(), random_float(), random_float())
    }

    pub fn random_range(range: Range<f32>) -> Self {
        Self(
            random_float_range(range.clone()),
            random_float_range(range.clone()),
            random_float_range(range),
        )
    }

    pub fn unit_mut(&mut self) {
        *self /= self.len();
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let v = Vec3::random_range(-1.0f32..1.0f32);
            if v.len_sq() < 1. {
                return v;
            }
        }
    }

    // returns a unit sphere or circle on the axes that are not 0
    pub fn random_in_unit_dim(dimensions: Vec3) -> Vec3 {
        // cap to unit
        let dimensions = Vec3::new(
            1.0f32.min(dimensions.x().max(0.)),
            1.0f32.min(dimensions.y().max(0.)),
            1.0f32.min(dimensions.z().max(0.)),
        );

        Vec3::random_in_unit_sphere() * dimensions
    }

    pub fn random_in_unit_circle_xy() -> Vec3 {
        Self::random_in_unit_dim(Vec3::new(1., 1., 0.))
    }

    pub fn random_unit() -> Vec3 {
        Self::random_in_unit_sphere().unit()
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit();
        if on_unit_sphere.dot(normal) > 0. {
            // in the same hemisphere as normal
            return on_unit_sphere;
        }
        -on_unit_sphere
    }

    pub fn is_near_zero(&self) -> bool {
        const LIMIT: f32 = 1e-6;
        self.0.abs() < LIMIT && self.1.abs() < LIMIT && self.2.abs() < LIMIT
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        // reflection of vector is V - 2b
        // where b is distance from V to surface
        // because V points into the surface b
        // needs to be negated
        *self - (2. * normal.clone() * self.dot(normal))
    }

    /// This returns a vector resulting in the refraction of incident vector `self`.
    /// R = R_parallel + R_perpendicular
    /// R_parallel = -sqrt( 1 - |R_perpendicular|^2 * N )
    /// R_perpendicular = RI_incident/RI_refraction * (R + (-r.N) * N)
    ///
    /// Params:
    /// * `normal` - surface normal of the material hit
    /// * `ri ratio` - ratio of refractive index:
    /// [material incident vector is in] / [material hit]
    pub fn refract(&self, normal: &Vec3, ri_ratio: f32) -> Vec3 {
        let cos_theta = 1.0f32.min((-*self).dot(normal));

        // type inference fking up here??
        let r_perpendicular = ri_ratio * (*self + cos_theta * normal.clone());
        let r_parallel = -((1. - r_perpendicular.len_sq()).abs().sqrt()) * normal.clone();

        r_perpendicular + r_parallel
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

// -Vec3
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

// Vec3 += Vec3
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

// Vec3 -= Vec3
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

// Vec3 *= f32
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        self.0 *= t;
        self.1 *= t;
        self.2 *= t;
    }
}

// Vec3 /= f32
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        self.2 /= t;
        self.2 /= t;
        self.2 /= t;
    }
}

// Vec3 + Vec3
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

// Vec3 - Vec3
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

// Vec3 * Vec3
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

// f32 * Vec3
impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

// Vec3 * f32
impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Vec3 {
        Vec3(self.0 * t, self.1 * t, self.2 * t)
    }
}

// Vec3 / f32
impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f32) -> Vec3 {
        Vec3(self.0 / t, self.1 / t, self.2 / t)
    }
}

pub type Point3 = Vec3;

#[derive(Default, Copy, Clone, Debug)]
pub struct Color(f32, f32, f32);

impl Color {
    // creates a new color with intensity between 0.0..=1.0
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color(r, g, b)
    }

    pub fn r(&self) -> f32 {
        self.0
    }

    pub fn g(&self) -> f32 {
        self.1
    }

    pub fn b(&self) -> f32 {
        self.2
    }

    pub fn to_rgb(&self) -> [u8; 3] {
        let intensity = Interval::from(0., 0.999);
        [
            (intensity.clamp(self.r()) * 256.) as u8,
            (intensity.clamp(self.g()) * 256.) as u8,
            (intensity.clamp(self.b()) * 256.) as u8,
        ]
    }

    pub fn to_gamma(&self) -> Color {
        Color(
            Self::linear_to_gamma(self.r()),
            Self::linear_to_gamma(self.g()),
            Self::linear_to_gamma(self.b()),
        )
    }

    fn linear_to_gamma(linear: f32) -> f32 {
        if linear > 0. {
            linear.sqrt()
        } else {
            0.
        }
    }

    pub fn is_near_zero(&self) -> bool {
        const LIMIT: f32 = 1e-6;
        self.0.abs() < LIMIT && self.1.abs() < LIMIT && self.2.abs() < LIMIT
    }

    pub fn random() -> Self {
        Color::new(random_float(), random_float(), random_float())
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [r, g, b] = self.to_rgb();
        write!(f, "{r} {g} {b}")
    }
}

// -Color
impl Neg for Color {
    type Output = Color;

    fn neg(self) -> Color {
        Color(-self.0, -self.1, -self.2)
    }
}

// Color += Color
impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

// Color -= Color
impl SubAssign for Color {
    fn sub_assign(&mut self, rhs: Color) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

// Color *= f32
impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, t: f32) {
        self.0 *= t;
        self.1 *= t;
        self.2 *= t;
    }
}

// Color /= f32
impl DivAssign<f32> for Color {
    fn div_assign(&mut self, t: f32) {
        self.2 /= t;
        self.2 /= t;
        self.2 /= t;
    }
}

// Color + Color
impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

// Color - Color
impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Color {
        Color(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

// Color * Color
impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

// f32 * Color
impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

// Color * f32
impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, t: f32) -> Color {
        Color(self.0 * t, self.1 * t, self.2 * t)
    }
}

// Color / f32
impl Div<f32> for Color {
    type Output = Color;

    fn div(self, t: f32) -> Color {
        Color(self.0 / t, self.1 / t, self.2 / t)
    }
}
