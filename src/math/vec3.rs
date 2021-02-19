use super::vec2::Vec2f;
use std::ops::{Neg, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

#[derive(Clone, Copy, PartialEq)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// Operations
impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn from_vec2(xy: Vec2f, z: f32) -> Self {
        Self { x: xy.x, y: xy.y, z }
    }

    pub fn from_uniform(u: f32) -> Self {
        Self::new(u, u, u)
    }

    pub fn zero() -> Self {
        Self::from_uniform(0.0)
    }

    pub fn one() -> Self {
        Self::from_uniform(1.0)
    }

    pub fn xy(self) -> Vec2f {
        Vec2f::new(self.x, self.y)
    }
}

impl Neg for Vec3f {
    type Output = Vec3f;

    fn neg(self) -> Self::Output {
       Self::new(-self.x, -self.y, -self.z) 
    }
}

impl Add for Vec3f {
    type Output = Vec3f;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3f {
    type Output = Vec3f;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f32> for Vec3f {
    type Output = Vec3f;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f32> for Vec3f {
    type Output = Vec3f;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Mul<Vec3f> for f32 {
    type Output = Vec3f;

    fn mul(self, rhs: Vec3f) -> Self::Output {
        Self::Output::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl AddAssign for Vec3f {
    fn add_assign(&mut self, rhs: Vec3f) {
       self.x += rhs.x;
       self.y += rhs.y;
       self.z += rhs.z;
    }
}

impl SubAssign for Vec3f {
    fn sub_assign(&mut self, rhs: Vec3f) {
       self.x -= rhs.x;
       self.y -= rhs.y;
       self.z -= rhs.z;
    }
}

impl MulAssign<f32> for Vec3f {
    fn mul_assign(&mut self, rhs: f32) {
       self.x *= rhs;
       self.y *= rhs;
       self.z *= rhs;
    }
}

impl DivAssign<f32> for Vec3f {
    fn div_assign(&mut self, rhs: f32) {
       self.x /= rhs;
       self.y /= rhs;
       self.z /= rhs;
    }
}
