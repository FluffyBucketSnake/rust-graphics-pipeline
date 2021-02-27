    use super::matrix::{Matrix, Transform};
use std::ops::{Neg, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

#[derive(Clone, Copy, PartialEq)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Vec2f {
    //
    // Constructors
    //
    
    /// Constructs a new 2D vector using the x and y coordinates.
    pub fn new(x: f32, y: f32) -> Self {
        Vec2f { x, y }
    }

    /// Constructs a new 2D vector with same coordinates values.
    pub fn from_uniform(u: f32) -> Self {
        Self::new(u, u)
    }

    /// Constructs a new 2D vector with length `1.0` using an angle (in radians).
    pub fn from_direction(d: f32) -> Self {
        Self::new(d.cos(), d.sin())
    }

    //
    // Defaults
    //
    
    /// The additive-identity of 2D vectors.
    pub fn zero() -> Self {
        Self::from_uniform(0.0)
    }

    /// The multiplicative-identity of 2D vectors.
    pub fn one() -> Self {
        Self::from_uniform(1.0)
    }

    /// (-1.0, 0.0)
    pub fn negative_x() -> Self {
        Self::new(-1.0, 0.0)
    }

    /// (1.0, 0.0)
    pub fn positive_x() -> Self {
        Self::new(1.0, 0.0)
    }

    /// (0.0, -1.0)
    pub fn negative_y() -> Self {
        Self::new(0.0, -1.0)
    }

    /// (0.0, 1.0)
    pub fn positive_y() -> Self {
        Self::new(0.0, 1.0)
    }

    //
    // Directions
    //

    pub fn left() -> Self {
        Self::negative_x()
    }

    pub fn right() -> Self {
        Self::positive_x()
    }

    pub fn bottom() -> Self {
        Self::negative_y()
    }

    pub fn top() -> Self {
        Self::positive_y()
    }

    //
    // Operations
    //

    /// Calculates the dot product between two vectors.
    pub fn dot(&self, other: &Self) -> f32 {
        (self.x * other.x) + (self.y * other.y)
    }

    /// Calculates the cross product between two vectors.
    pub fn cross(&self, other: &Self) -> f32 {
        (self.x * other.y) - (self.y * other.x)
    }
}

impl Neg for Vec2f {
    type Output = Vec2f;

    fn neg(self) -> Self::Output {
       Self::new(-self.x, -self.y) 
    }
}

impl Transform for Vec2f {
    fn transform(&self, matrix: &Matrix) -> Self {
        let mut result = *self;
        result.transform_self(matrix);
        result
    }

    fn transform_self(&mut self, matrix: &Matrix) {
        self.x = (self.x * matrix.m11) + (self.y * matrix.m12) + matrix.m14;
        self.y = (self.x * matrix.m21) + (self.y * matrix.m22) + matrix.m24;
    }
}

macro_rules! op_impl {
    (vector, $trait:ident, $traitf:ident, $atrait:ident, $atraitf:ident, $op:tt, $aop:tt) => {
        impl $trait<Vec2f> for Vec2f {
            type Output = Vec2f;

            fn $traitf(self, rhs: Vec2f) -> Self::Output {
                 Vec2f::new(self.x $op rhs.x, self.y $op rhs.y)
            }
        }

        impl $trait<&Vec2f> for Vec2f {
            type Output = Vec2f;

            fn $traitf(self, rhs: &Vec2f) -> Self::Output {
                 Vec2f::new(self.x $op rhs.x, self.y $op rhs.y)
            }
        }

        impl $trait<Vec2f> for &Vec2f {
            type Output = Vec2f;

            fn $traitf(self, rhs: Vec2f) -> Self::Output {
                Vec2f::new(self.x $op rhs.x, self.y $op rhs.y)
            }
        }
        
        impl $trait<&Vec2f> for &Vec2f {
            type Output = Vec2f;

            fn $traitf(self, &rhs: &Vec2f) -> Self::Output {
                Vec2f::new(self.x $op rhs.x, self.y $op rhs.y)
            }
        }

        impl $atrait<Vec2f> for Vec2f {
            fn $atraitf(&mut self, rhs: Self) {
                self.x $aop rhs.x;
                self.y $aop rhs.y;
            }
        }

        impl $atrait<&Vec2f> for Vec2f {
            fn $atraitf(&mut self, rhs: &Vec2f) {
                self.x $aop rhs.x;
                self.y $aop rhs.y;
            }
        }
    };
    (uniform, $trait:ident, $traitf:ident, $atrait:ident, $atraitf:ident, $op:tt, $aop:tt) => {
        impl $trait<f32> for Vec2f {
            type Output = Vec2f;

            fn $traitf(self, rhs: f32) -> Self::Output {
                 Vec2f::new(self.x $op rhs, self.y $op rhs)
            }
        }

        impl $trait<&f32> for Vec2f {
            type Output = Vec2f;

            fn $traitf(self, rhs: &f32) -> Self::Output {
                 Vec2f::new(self.x $op rhs, self.y $op rhs)
            }
        }

        impl $trait<f32> for &Vec2f {
            type Output = Vec2f;

            fn $traitf(self, rhs: f32) -> Self::Output {
                Vec2f::new(self.x $op rhs, self.y $op rhs)
            }
        }
        
        impl $trait<&f32> for &Vec2f {
            type Output = Vec2f;

            fn $traitf(self, &rhs: &f32) -> Self::Output {
                Vec2f::new(self.x $op rhs, self.y $op rhs)
            }
        }

        impl $atrait<f32> for Vec2f {
            fn $atraitf(&mut self, rhs: f32) {
                self.x $aop rhs;
                self.y $aop rhs;
            }
        }

        impl $atrait<&f32> for Vec2f {
            fn $atraitf(&mut self, rhs: &f32) {
                self.x $aop rhs;
                self.y $aop rhs;
            }
        }
    };
    (uniform, commutative, $trait:ident, $traitf:ident, $atrait:ident, $atraitf:ident, $op:tt, $aop:tt) => {
        op_impl!(uniform, $trait, $traitf, $atrait, $atraitf, $op, $aop);

        impl $trait<Vec2f> for f32 {
            type Output = Vec2f;

            fn $traitf(self, rhs: Vec2f) -> Self::Output {
                rhs $op self
            }
        }

        impl $trait<&Vec2f> for f32 {
            type Output = Vec2f;

            fn $traitf(self, rhs: &Vec2f) -> Self::Output {
                rhs $op self
            }
        }

        impl $trait<Vec2f> for &f32 {
            type Output = Vec2f;

            fn $traitf(self, rhs: Vec2f) -> Self::Output {
                rhs $op self
            }
        }
        
        impl $trait<&Vec2f> for &f32 {
            type Output = Vec2f;

            fn $traitf(self, &rhs: &Vec2f) -> Self::Output {
                rhs $op self
            }
        }
    };
}

op_impl!(vector, Add, add, AddAssign, add_assign, +, +=);
op_impl!(vector, Sub, sub, SubAssign, sub_assign, -, -=);
op_impl!(vector, Mul, mul, MulAssign, mul_assign, *, *=);
op_impl!(vector, Div, div, DivAssign, div_assign, /, /=);
op_impl!(uniform, commutative, Mul, mul, MulAssign, mul_assign, *, *=);
op_impl!(uniform, Div, div, DivAssign, div_assign, /, /=);
