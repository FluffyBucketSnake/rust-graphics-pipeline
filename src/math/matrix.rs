use crate::math::Vec3f;

#[derive(Copy, Clone)]
pub struct Matrix {
    pub m11: f32, pub m12: f32, pub m13: f32, pub m14: f32,
    pub m21: f32, pub m22: f32, pub m23: f32, pub m24: f32,
    pub m31: f32, pub m32: f32, pub m33: f32, pub m34: f32,
    pub m41: f32, pub m42: f32, pub m43: f32, pub m44: f32,
}

impl Matrix {
    pub fn new(m11: f32, m12: f32, m13: f32, m14: f32,
               m21: f32, m22: f32, m23: f32, m24: f32,
               m31: f32, m32: f32, m33: f32, m34: f32,
               m41: f32, m42: f32, m43: f32, m44: f32,) -> Self {
        Self {
            m11, m12, m13, m14,
            m21, m22, m23, m24,
            m31, m32, m33, m34,
            m41, m42, m43, m44,
        }
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        Self::new(  x, 0.0, 0.0, 0.0,
                  0.0,   y, 0.0, 0.0,
                  0.0, 0.0,   z, 0.0,
                  0.0, 0.0, 0.0, 1.0)
    }

    pub fn scale_uniform(value: f32) -> Self {
        Self::scale(value, value, value)
    }
    
    pub fn identity() -> Self {
        Self::scale_uniform(1.0)
    }

    pub fn rotate_x(angle: f32) -> Self {
        Self::new(1.0,         0.0,          0.0, 0.0,
                  0.0, angle.cos(), -angle.sin(), 0.0,
                  0.0, angle.sin(),  angle.cos(), 0.0,
                  0.0,         0.0,          0.0, 1.0)
    }
    
    pub fn rotate_y(angle: f32) -> Self {
        Self::new( angle.cos(), 0.0, angle.sin(), 0.0,
                           0.0, 1.0,         0.0, 0.0,
                  -angle.sin(), 0.0, angle.cos(), 0.0,
                           0.0, 0.0,         0.0, 1.0)
    }

    pub fn rotate_z(angle: f32) -> Self {
        Self::new(angle.cos(), -angle.sin(), 0.0, 0.0,
                  angle.sin(),  angle.cos(), 0.0, 0.0,
                          0.0,          0.0, 1.0, 0.0,
                          0.0,          0.0, 0.0, 1.0)
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Self {
        Self::new(1.0, 0.0, 0.0,   x,
                  0.0, 1.0, 0.0,   y,
                  0.0, 0.0, 1.0,   z,
                  0.0, 0.0, 0.0, 1.0)
    }

    pub fn transform(&self, rhs: &Self) -> Self {
        Self::new((self.m11 * rhs.m11) + (self.m12 * rhs.m21) + (self.m13 * rhs.m31) + (self.m14 * rhs.m41),
                  (self.m11 * rhs.m12) + (self.m12 * rhs.m22) + (self.m13 * rhs.m32) + (self.m14 * rhs.m42),
                  (self.m11 * rhs.m13) + (self.m12 * rhs.m23) + (self.m13 * rhs.m33) + (self.m14 * rhs.m43),
                  (self.m11 * rhs.m14) + (self.m12 * rhs.m24) + (self.m13 * rhs.m34) + (self.m14 * rhs.m44),
                  (self.m21 * rhs.m11) + (self.m22 * rhs.m21) + (self.m23 * rhs.m31) + (self.m24 * rhs.m41),
                  (self.m21 * rhs.m12) + (self.m22 * rhs.m22) + (self.m23 * rhs.m32) + (self.m24 * rhs.m42),
                  (self.m21 * rhs.m13) + (self.m22 * rhs.m23) + (self.m23 * rhs.m33) + (self.m24 * rhs.m43),
                  (self.m21 * rhs.m14) + (self.m22 * rhs.m24) + (self.m23 * rhs.m34) + (self.m24 * rhs.m44),
                  (self.m31 * rhs.m11) + (self.m32 * rhs.m21) + (self.m33 * rhs.m31) + (self.m34 * rhs.m41),
                  (self.m31 * rhs.m12) + (self.m32 * rhs.m22) + (self.m33 * rhs.m32) + (self.m34 * rhs.m42),
                  (self.m31 * rhs.m13) + (self.m32 * rhs.m23) + (self.m33 * rhs.m33) + (self.m34 * rhs.m43),
                  (self.m31 * rhs.m14) + (self.m32 * rhs.m24) + (self.m33 * rhs.m34) + (self.m34 * rhs.m44),
                  (self.m41 * rhs.m11) + (self.m42 * rhs.m21) + (self.m43 * rhs.m31) + (self.m44 * rhs.m41),
                  (self.m41 * rhs.m12) + (self.m42 * rhs.m22) + (self.m43 * rhs.m32) + (self.m44 * rhs.m42),
                  (self.m41 * rhs.m13) + (self.m42 * rhs.m23) + (self.m43 * rhs.m33) + (self.m44 * rhs.m43),
                  (self.m41 * rhs.m14) + (self.m42 * rhs.m24) + (self.m43 * rhs.m34) + (self.m44 * rhs.m44))
    }

    pub fn transform_vec3f(&self, v: &Vec3f) -> Vec3f {
        Vec3f::new((self.m11 * v.x) + (self.m12 * v.y) + (self.m13 * v.z) + self.m14,
                   (self.m21 * v.x) + (self.m22 * v.y) + (self.m23 * v.z) + self.m24,
                   (self.m31 * v.x) + (self.m32 * v.y) + (self.m33 * v.z) + self.m34)
    }
}

macro_rules! op_impl {
    (matrix, $opt:ident, $opf:ident, $aopt:ident, $aopf:ident, $op:tt) => {
        impl std::ops::$opt for Matrix {
            type Output = Matrix;

            fn $opf(self, rhs: Self) -> Self::Output {
                Matrix::new(self.m11 $op rhs.m11, self.m12 $op rhs.m12, self.m13 $op rhs.m13, self.m14 $op rhs.m14,
                            self.m21 $op rhs.m21, self.m22 $op rhs.m22, self.m23 $op rhs.m23, self.m24 $op rhs.m24,
                            self.m31 $op rhs.m31, self.m32 $op rhs.m32, self.m33 $op rhs.m33, self.m34 $op rhs.m34,
                            self.m41 $op rhs.m41, self.m42 $op rhs.m42, self.m43 $op rhs.m43, self.m44 $op rhs.m44)
            }
        }

        impl std::ops::$aopt for Matrix {
            fn $aopf(&mut self, rhs: Self) {
                *self = *self $op rhs;
            }
        }
    };
    (uniform, commutative, $ut:ident, $opt:ident, $opf:ident, $aopt:ident, $aopf:ident, $op:tt) => {
        op_impl!(uniform, $ut, $opt, $opf, $aopt, $aopf, $op);

        impl std::ops::$opt<Matrix> for $ut {
            type Output = Matrix;

            fn $opf(self, rhs: Matrix) -> Self::Output {
                rhs * self
            }
        }
    };
    (uniform, $ut:ident, $opt:ident, $opf:ident, $aopt:ident, $aopf:ident, $op:tt) => {
        impl std::ops::$opt<$ut> for Matrix {
            type Output = Matrix;

            fn $opf(self, rhs: $ut) -> Self::Output {
                Matrix::new(self.m11 $op rhs, self.m12 $op rhs, self.m13 $op rhs, self.m14 $op rhs,
                            self.m21 $op rhs, self.m22 $op rhs, self.m23 $op rhs, self.m24 $op rhs,
                            self.m31 $op rhs, self.m32 $op rhs, self.m33 $op rhs, self.m34 $op rhs,
                            self.m41 $op rhs, self.m42 $op rhs, self.m43 $op rhs, self.m44 $op rhs)
            }
        }

        impl std::ops::$aopt<$ut> for Matrix {
            fn $aopf(&mut self, rhs: $ut) {
                *self = *self $op rhs;
            }
        }
    };
    (transform, $tt:ident, $tm:ident) => {
        impl std::ops::Mul<$tt> for Matrix {
            type Output = $tt;

            fn mul(self, rhs: $tt) -> Self::Output {
                self.$tm(&rhs)
            }
        }
    }
}

op_impl!(matrix, Add, add, AddAssign, add_assign, +);
op_impl!(matrix, Sub, sub, SubAssign, sub_assign, -);
op_impl!(uniform, commutative, f32, Mul, mul, MulAssign, mul_assign, *);
op_impl!(uniform, f32, Div, div, DivAssign, div_assign, /);
op_impl!(transform, Matrix, transform);
op_impl!(transform, Vec3f, transform_vec3f);

impl std::ops::Neg for Matrix {
    type Output = Matrix;

    fn neg(self) -> Self::Output {
        Self::new(-self.m11, -self.m12, -self.m13, -self.m14,
                  -self.m21, -self.m22, -self.m23, -self.m24,
                  -self.m31, -self.m32, -self.m33, -self.m34,
                  -self.m41, -self.m42, -self.m43, -self.m44)
    }
}
