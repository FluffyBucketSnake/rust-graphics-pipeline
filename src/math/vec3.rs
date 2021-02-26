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

    pub fn left() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    pub fn dot(&self, other: &Self) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self::new((self.y * other.z) - (self.z * other.y),
                  (self.z * other.x) - (self.x * other.z),
                  (self.x * other.y) - (self.y * other.x))
    }

    pub fn xy(self) -> Vec2f {
        Vec2f::new(self.x, self.y)
    }
}

macro_rules! op_impl {
    (vector, $trait:ident, $traitf:ident, $atrait:ident, $atraitf:ident, $op:tt, $aop:tt) => {
        impl $trait<Vec3f> for Vec3f {
            type Output = Vec3f;

            fn $traitf(self, rhs: Vec3f) -> Self::Output {
                 Vec3f::new(self.x $op rhs.x, self.y $op rhs.y, self.z $op rhs.z)
            }
        }

        impl $trait<&Vec3f> for Vec3f {
            type Output = Vec3f;

            fn $traitf(self, rhs: &Vec3f) -> Self::Output {
                 Vec3f::new(self.x $op rhs.x, self.y $op rhs.y, self.z $op rhs.z)
            }
        }

        impl $trait<Vec3f> for &Vec3f {
            type Output = Vec3f;

            fn $traitf(self, rhs: Vec3f) -> Self::Output {
                Vec3f::new(self.x $op rhs.x, self.y $op rhs.y, self.z $op rhs.z)
            }
        }
        
        impl $trait<&Vec3f> for &Vec3f {
            type Output = Vec3f;

            fn $traitf(self, &rhs: &Vec3f) -> Self::Output {
                Vec3f::new(self.x $op rhs.x, self.y $op rhs.y, self.z $op rhs.z)
            }
        }

        impl $atrait<Vec3f> for Vec3f {
            fn $atraitf(&mut self, rhs: Self) {
                self.x $aop rhs.x;
                self.y $aop rhs.y;
                self.z $aop rhs.z;
            }
        }

        impl $atrait<&Vec3f> for Vec3f {
            fn $atraitf(&mut self, rhs: &Vec3f) {
                self.x $aop rhs.x;
                self.y $aop rhs.y;
                self.z $aop rhs.z;
            }
        }
    };
    (uniform, $trait:ident, $traitf:ident, $atrait:ident, $atraitf:ident, $op:tt, $aop:tt) => {
        impl $trait<f32> for Vec3f {
            type Output = Vec3f;

            fn $traitf(self, rhs: f32) -> Self::Output {
                 Vec3f::new(self.x $op rhs, self.y $op rhs, self.z $op rhs)
            }
        }

        impl $trait<&f32> for Vec3f {
            type Output = Vec3f;

            fn $traitf(self, rhs: &f32) -> Self::Output {
                 Vec3f::new(self.x $op rhs, self.y $op rhs, self.z $op rhs)
            }
        }

        impl $trait<f32> for &Vec3f {
            type Output = Vec3f;

            fn $traitf(self, rhs: f32) -> Self::Output {
                Vec3f::new(self.x $op rhs, self.y $op rhs, self.z $op rhs)
            }
        }
        
        impl $trait<&f32> for &Vec3f {
            type Output = Vec3f;

            fn $traitf(self, &rhs: &f32) -> Self::Output {
                Vec3f::new(self.x $op rhs, self.y $op rhs, self.z $op rhs)
            }
        }

        impl $atrait<f32> for Vec3f {
            fn $atraitf(&mut self, rhs: f32) {
                self.x $aop rhs;
                self.y $aop rhs;
                self.z $aop rhs;
            }
        }

        impl $atrait<&f32> for Vec3f {
            fn $atraitf(&mut self, rhs: &f32) {
                self.x $aop rhs;
                self.y $aop rhs;
                self.z $aop rhs;
            }
        }
    };
    (uniform, commutative, $trait:ident, $traitf:ident, $atrait:ident, $atraitf:ident, $op:tt, $aop:tt) => {
        op_impl!(uniform, $trait, $traitf, $atrait, $atraitf, $op, $aop);

        impl $trait<Vec3f> for f32 {
            type Output = Vec3f;

            fn $traitf(self, rhs: Vec3f) -> Self::Output {
                rhs $op self
            }
        }

        impl $trait<&Vec3f> for f32 {
            type Output = Vec3f;

            fn $traitf(self, rhs: &Vec3f) -> Self::Output {
                rhs $op self
            }
        }

        impl $trait<Vec3f> for &f32 {
            type Output = Vec3f;

            fn $traitf(self, rhs: Vec3f) -> Self::Output {
                rhs $op self
            }
        }
        
        impl $trait<&Vec3f> for &f32 {
            type Output = Vec3f;

            fn $traitf(self, &rhs: &Vec3f) -> Self::Output {
                rhs $op self
            }
        }
    };
}

impl Neg for Vec3f {
    type Output = Vec3f;

    fn neg(self) -> Self::Output {
       Self::new(-self.x, -self.y, -self.z) 
    }
}

op_impl!(vector, Add, add, AddAssign, add_assign, +, +=);
op_impl!(vector, Sub, sub, SubAssign, sub_assign, -, -=);
op_impl!(vector, Mul, mul, MulAssign, mul_assign, *, *=);
op_impl!(vector, Div, div, DivAssign, div_assign, /, /=);
op_impl!(uniform, commutative, Mul, mul, MulAssign, mul_assign, *, *=);
op_impl!(uniform, Div, div, DivAssign, div_assign, /, /=);
