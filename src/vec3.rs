use std::{fmt::Display, ops::{Add, Mul, Sub, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign}};

#[derive(Default, Copy, Clone)]
pub struct Vec3(f32, f32, f32);

impl Vec3 {

    fn len_sq(&self) -> f32 {
        self.0*self.0 + self.1*self.1 + self.2*self.2
    }


    fn len(&self) -> f32 {
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
        *self/self.len()
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

pub struct Color(f32, f32, f32);

impl Color {
    // creates a new color with intensity between 0.0..=1.0
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color(r, g, b)
    }

    pub fn  r(&self) -> f32 {
        self.0
    }

    pub fn  g(&self) -> f32 {
        self.1
    }
    
    pub fn  b(&self) -> f32 {
        self.2
    }
    
    pub fn to_rgb(&self) -> [u8; 3] {
        [(self.r() * 255.999) as u8, (self.g() * 255.999) as u8, (self.b() * 255.999) as u8]
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [r,g,b] = self.to_rgb();
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