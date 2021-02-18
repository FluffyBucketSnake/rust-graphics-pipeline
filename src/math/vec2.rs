use std::ops::{Neg, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

#[derive(Clone, Copy, PartialEq)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

// Operations
impl Vec2f {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2f { x, y }
    }

    pub fn from_uniform(u: f32) -> Self {
        Self::new(u, u)
    }

    pub fn from_direction(d: f32) -> Self {
        Self::new(d.cos(), d.sin())
    }

    pub fn zero() -> Self {
        Self::from_uniform(0.0)
    }

    pub fn one() -> Self {
        Self::from_uniform(1.0)
    }
}

impl Neg for Vec2f {
    type Output = Vec2f;

    fn neg(self) -> Self::Output {
       Self::new(-self.x, -self.y) 
    }
}

impl Add for Vec2f {
    type Output = Vec2f;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2f {
    type Output = Vec2f;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f32> for Vec2f {
    type Output = Vec2f;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f32> for Vec2f {
    type Output = Vec2f;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl Mul<Vec2f> for f32 {
    type Output = Vec2f;

    fn mul(self, rhs: Vec2f) -> Self::Output {
        Self::Output::new(self * rhs.x, self * rhs.y)
    }
}

impl Div<Vec2f> for f32 {
    type Output = Vec2f;

    fn div(self, rhs: Vec2f) -> Self::Output {
        Self::Output::new(self / rhs.x, self / rhs.y)
    }
}

impl AddAssign for Vec2f {
    fn add_assign(&mut self, rhs: Vec2f) {
       self.x += rhs.x;
       self.y += rhs.y;
    }
}

impl SubAssign for Vec2f {
    fn sub_assign(&mut self, rhs: Vec2f) {
       self.x -= rhs.x;
       self.y -= rhs.y;
    }
}

impl MulAssign<f32> for Vec2f {
    fn mul_assign(&mut self, rhs: f32) {
       self.x *= rhs;
       self.y *= rhs;
    }
}

impl DivAssign<f32> for Vec2f {
    fn div_assign(&mut self, rhs: f32) {
       self.x /= rhs;
       self.y /= rhs;
    }
}
