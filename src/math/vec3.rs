use super::matrix::{Matrix, Transform};
use super::vec2::Vec2f;
use std::ops::{Neg, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

/// Represents a 3D vector.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    // 
    // Constructors
    //
    
    /// Creates a new 3D vector using the x,y and z coordinates.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Creates a new 3D vector using a 2D vector(x and y) and a scalar(z).
    pub fn from_vec2(xy: Vec2f, z: f32) -> Self {
        Self { x: xy.x, y: xy.y, z }
    }

    /// Creates a 3D vector with the same coordinates values.
    pub fn from_uniform(u: f32) -> Self {
        Self::new(u, u, u)
    }

    //
    // Defaults
    // 

    /// The additive-identity of 3D vectors.
    ///
    /// Value: (0.0, 0.0, 0.0)
    pub fn zero() -> Self {
        Self::from_uniform(0.0)
    }

    /// The multiplicative-identity of 3D vectors.
    ///
    /// Value: (1.0, 1.0, 1.0)
    pub fn one() -> Self {
        Self::from_uniform(1.0)
    }

    /// (-1.0, 0.0, 0.0)
    pub fn negative_x() -> Self {
        Self::new(-1.0, 0.0, 0.0)
    }

    /// (1.0, 0.0, 0.0)
    pub fn positive_x() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    /// (0.0, -1.0, 0.0)
    pub fn negative_y() -> Self {
        Self::new(0.0, -1.0, 0.0)
    }

    /// (0.0, 1.0, 0.0)
    pub fn positive_y() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    /// (0.0, 0.0, -1.0)
    pub fn negative_z() -> Self {
        Self::new(0.0, 0.0, -1.0)
    }

    /// (0.0, 0.0, 1.0)
    pub fn positive_z() -> Self {
        Self::new(0.0, 0.0, 1.0)
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

    pub fn backward() -> Self {
        Self::negative_z()
    }

    pub fn forward() -> Self {
        Self::positive_z()
    }

    //
    // Operations
    //
    
    /// Calculates the dot product between two vectors.
    pub fn dot(&self, other: &Self) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    /// Calculates the cross product between two vectors.
    pub fn cross(&self, other: &Self) -> Self {
        Self::new((self.y * other.z) - (self.z * other.y),
                  (self.z * other.x) - (self.x * other.z),
                  (self.x * other.y) - (self.y * other.x))
    }

    /// Returns the x and y coordinates as a 2D vector.
    pub fn xy(self) -> Vec2f {
        Vec2f::new(self.x, self.y)
    }

    /// Returns the squared length of the vector.
    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }

    /// Returns the length of the vector.
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
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

impl Transform for Vec3f {
    fn transform(&self, matrix: &Matrix) -> Self {
        Vec3f::new((matrix.m11 * self.x) + (matrix.m12 * self.y) + (matrix.m13 * self.z) + matrix.m14,
                   (matrix.m21 * self.x) + (matrix.m22 * self.y) + (matrix.m23 * self.z) + matrix.m24,
                   (matrix.m31 * self.x) + (matrix.m32 * self.y) + (matrix.m33 * self.z) + matrix.m34)
    }

    fn transform_self(&mut self, matrix: &Matrix) {
        *self = self.transform(matrix);
    }
}

op_impl!(vector, Add, add, AddAssign, add_assign, +, +=);
op_impl!(vector, Sub, sub, SubAssign, sub_assign, -, -=);
op_impl!(vector, Mul, mul, MulAssign, mul_assign, *, *=);
op_impl!(vector, Div, div, DivAssign, div_assign, /, /=);
op_impl!(uniform, commutative, Mul, mul, MulAssign, mul_assign, *, *=);
op_impl!(uniform, Div, div, DivAssign, div_assign, /, /=);
