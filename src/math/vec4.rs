use super::matrix::{Matrix, Transform};
use super::vec3::Vec3f;
use std::ops::{Neg, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

/// Represents a 4D vector.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4f {
    // 
    // Constructors
    //
    
    /// Creates a new 4D vector using the x, y, z and w coordinates.
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Creates a new 3D vector using a 3D vector(x, y and z) and a scalar(w).
    pub fn from_vec3(xyz: Vec3f, w: f32) -> Self {
        Self { x: xyz.x, y: xyz.y, z: xyz.z, w }
    }

    /// Creates a new 3D point using a 3D vector(x, y and z).
    pub fn from_point(xyz: Vec3f) -> Self {
        Self::from_vec3(xyz, 1.0)
    }

    /// Creates a new 3D direction using a 3D vector(x, y and z).
    pub fn from_direction(xyz: Vec3f) -> Self {
        Self::from_vec3(xyz, 0.0)
    }

    /// Creates a 3D vector with the same x, y and z coordinates values and a unique w value.
    pub fn from_uniform(u: f32, w: f32) -> Self {
        Self::new(u, u, u, w)
    }

    //
    // Defaults
    // 

    /// The additive-identity of 4D vectors.
    ///
    /// Value: (0.0, 0.0, 0.0, 0.0)
    pub fn zero() -> Self {
        Self::from_uniform(0.0, 0.0)
    }

    /// The multiplicative-identity of 3D vectors.
    ///
    /// Value: (1.0, 1.0, 1.0, 1.0)
    pub fn one() -> Self {
        Self::from_uniform(1.0, 1.0)
    }

    /// (1.0, 0.0, 0.0, 0.0)
    pub fn unitx() -> Self {
        Self::new(1.0, 0.0, 0.0, 0.0)
    }

    /// (0.0, 1.0, 0.0, 0.0)
    pub fn unity() -> Self {
        Self::new(0.0, 1.0, 0.0, 0.0)
    }
    
    /// (0.0, 0.0, 1.0, 0.0)
    pub fn unitz() -> Self {
        Self::new(0.0, 0.0, 1.0, 0.0)
    }

    /// (0.0, 0.0, 0.0, 1.0)
    pub fn unitw() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }

    //
    // Operations
    //
    
    /// Calculates the dot product between two vectors.
    pub fn dot(&self, other: &Self) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
    }

    /// Returns the x and y coordinates as a 2D vector.
    pub fn xyz(self) -> Vec3f {
        Vec3f::new(self.x, self.y, self.z)
    }

    /// Returns the squared length of the vector.
    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }

    /// Returns the length of the vector.
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Normalizes this vector.
    pub fn normalize_self(&mut self) {
        *self /= self.length();
    }

    /// Returns a vector with the normalized values from this vector.
    pub fn normalize(&self) -> Vec4f {
        let mut result = *self;
        result.normalize_self();
        result
    }

    /// Homogenizes this vector.
    pub fn homogenize_self(&mut self) {
        *self /= self.w;
    }

    /// Returns a vector with the homogenized values from this vector.
    pub fn homogenize(&self) -> Vec4f {
        let mut result = *self;
        result.homogenize_self();
        result
    }
}

macro_rules! op_impl {
    (vector, $trait:ident, $traitf:ident, $atrait:ident, $atraitf:ident, $op:tt, $aop:tt) => {
        impl $trait<Vec4f> for Vec4f {
            type Output = Self;

            fn $traitf(self, rhs: Self) -> Self::Output {
                 Self::new(self.x $op rhs.x, self.y $op rhs.y, self.z $op rhs.z, self.w $op rhs.w)
            }
        }

        impl $trait<&Vec4f> for Vec4f {
            type Output = Self;

            fn $traitf(self, rhs: &Self) -> Self::Output {
                 Self::new(self.x $op rhs.x, self.y $op rhs.y, self.z $op rhs.z, self.w $op rhs.w)
            }
        }

        impl $trait<Vec4f> for &Vec4f {
            type Output = Vec4f;

            fn $traitf(self, rhs: Vec4f) -> Self::Output {
                Vec4f::new(self.x $op rhs.x, self.y $op rhs.y, self.z $op rhs.z, self.w $op rhs.w)
            }
        }
        
        impl $trait<&Vec4f> for &Vec4f {
            type Output = Vec4f;

            fn $traitf(self, &rhs: &Vec4f) -> Self::Output {
                Vec4f::new(self.x $op rhs.x, self.y $op rhs.y, self.z $op rhs.z, self.w $op rhs.w)
            }
        }

        impl $atrait<Vec4f> for Vec4f {
            fn $atraitf(&mut self, rhs: Self) {
                self.x $aop rhs.x;
                self.y $aop rhs.y;
                self.z $aop rhs.z;
                self.w $aop rhs.w;
            }
        }

        impl $atrait<&Vec4f> for Vec4f {
            fn $atraitf(&mut self, rhs: &Self) {
                self.x $aop rhs.x;
                self.y $aop rhs.y;
                self.z $aop rhs.z;
                self.w $aop rhs.w;
            }
        }
    };
    (uniform, $trait:ident, $traitf:ident, $atrait:ident, $atraitf:ident, $op:tt, $aop:tt) => {
        impl $trait<f32> for Vec4f {
            type Output = Self;

            fn $traitf(self, rhs: f32) -> Self::Output {
                 Self::new(self.x $op rhs, self.y $op rhs, self.z $op rhs, self.w $op rhs)
            }
        }

        impl $trait<&f32> for Vec4f {
            type Output = Self;

            fn $traitf(self, rhs: &f32) -> Self::Output {
                 Self::new(self.x $op rhs, self.y $op rhs, self.z $op rhs, self.w $op rhs)
            }
        }

        impl $trait<f32> for &Vec4f {
            type Output = Vec4f;

            fn $traitf(self, rhs: f32) -> Self::Output {
                Vec4f::new(self.x $op rhs, self.y $op rhs, self.z $op rhs, self.w $op rhs)
            }
        }
        
        impl $trait<&f32> for &Vec4f {
            type Output = Vec4f;

            fn $traitf(self, &rhs: &f32) -> Self::Output {
                Vec4f::new(self.x $op rhs, self.y $op rhs, self.z $op rhs, self.w $op rhs)
            }
        }

        impl $atrait<f32> for Vec4f {
            fn $atraitf(&mut self, rhs: f32) {
                self.x $aop rhs;
                self.y $aop rhs;
                self.z $aop rhs;
                self.w $aop rhs;
            }
        }

        impl $atrait<&f32> for Vec4f {
            fn $atraitf(&mut self, rhs: &f32) {
                self.x $aop rhs;
                self.y $aop rhs;
                self.z $aop rhs;
                self.w $aop rhs;
            }
        }
    };
    (uniform, commutative, $trait:ident, $traitf:ident, $atrait:ident, $atraitf:ident, $op:tt, $aop:tt) => {
        op_impl!(uniform, $trait, $traitf, $atrait, $atraitf, $op, $aop);

        impl $trait<Vec4f> for f32 {
            type Output = Vec4f;

            fn $traitf(self, rhs: Vec4f) -> Self::Output {
                rhs $op self
            }
        }

        impl $trait<&Vec4f> for f32 {
            type Output = Vec4f;

            fn $traitf(self, rhs: &Vec4f) -> Self::Output {
                rhs $op self
            }
        }

        impl $trait<Vec4f> for &f32 {
            type Output = Vec4f;

            fn $traitf(self, rhs: Vec4f) -> Self::Output {
                rhs $op self
            }
        }
        
        impl $trait<&Vec4f> for &f32 {
            type Output = Vec4f;

            fn $traitf(self, &rhs: &Vec4f) -> Self::Output {
                rhs $op self
            }
        }
    };
}

impl Neg for Vec4f {
    type Output = Vec4f;

    fn neg(self) -> Self::Output {
       Self::new(-self.x, -self.y, -self.z, - self.w) 
    }
}

impl Transform for Vec4f {
    fn transform(&self, matrix: &Matrix) -> Self {
        Vec4f::new((matrix.m11 * self.x) + (matrix.m12 * self.y) + (matrix.m13 * self.z) + (matrix.m14 * self.w),
                   (matrix.m21 * self.x) + (matrix.m22 * self.y) + (matrix.m23 * self.z) + (matrix.m24 * self.w),
                   (matrix.m31 * self.x) + (matrix.m32 * self.y) + (matrix.m33 * self.z) + (matrix.m34 * self.w),
                   (matrix.m41 * self.x) + (matrix.m42 * self.y) + (matrix.m43 * self.z) + (matrix.m44 * self.w))
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
