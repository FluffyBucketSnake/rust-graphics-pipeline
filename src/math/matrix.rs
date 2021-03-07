/// Represents a right-handed 4x4 matrix, which can be used to apply transformation to vectors.
///
/// # Examples
///
/// ```
/// let vector = crate::math::Vec3f::new(1.0, 0.0, 0.0);
///
/// let transform = crate::math::Matrix::translate(1.0, 1.0, 1.0);
///
/// assert_eq!(vector + 1.0, vector * transform);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Matrix {
    pub m11: f32, pub m12: f32, pub m13: f32, pub m14: f32,
    pub m21: f32, pub m22: f32, pub m23: f32, pub m24: f32,
    pub m31: f32, pub m32: f32, pub m33: f32, pub m34: f32,
    pub m41: f32, pub m42: f32, pub m43: f32, pub m44: f32,
}

impl Matrix {
    /// Constructs a new matrix using each element individually.
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
    
    /// The identity matrix.
    pub fn identity() -> Self {
        Self::scale_uniform(1.0)
    }

    /// Constructs a matrix for scale transformations.
    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        Self::new(  x, 0.0, 0.0, 0.0,
                  0.0,   y, 0.0, 0.0,
                  0.0, 0.0,   z, 0.0,
                  0.0, 0.0, 0.0, 1.0)
    }

    /// Constructs a matrix for uniform scale transformations.
    pub fn scale_uniform(value: f32) -> Self {
        Self::scale(value, value, value)
    }

    /// Constructs a matrix for rotating around the x-axis.
    pub fn rotate_x(angle: f32) -> Self {
        Self::new(1.0,         0.0,          0.0, 0.0,
                  0.0, angle.cos(), -angle.sin(), 0.0,
                  0.0, angle.sin(),  angle.cos(), 0.0,
                  0.0,         0.0,          0.0, 1.0)
    }
    
    /// Constructs a matrix for rotating around the y-axis.
    pub fn rotate_y(angle: f32) -> Self {
        Self::new( angle.cos(), 0.0, angle.sin(), 0.0,
                           0.0, 1.0,         0.0, 0.0,
                  -angle.sin(), 0.0, angle.cos(), 0.0,
                           0.0, 0.0,         0.0, 1.0)
    }

    /// Constructs a matrix for rotating around the z-axis.
    pub fn rotate_z(angle: f32) -> Self {
        Self::new(angle.cos(), -angle.sin(), 0.0, 0.0,
                  angle.sin(),  angle.cos(), 0.0, 0.0,
                          0.0,          0.0, 1.0, 0.0,
                          0.0,          0.0, 0.0, 1.0)
    }

    /// Constructs a matrix for translation transformations.
    pub fn translate(x: f32, y: f32, z: f32) -> Self {
        Self::new(1.0, 0.0, 0.0,   x,
                  0.0, 1.0, 0.0,   y,
                  0.0, 0.0, 1.0,   z,
                  0.0, 0.0, 0.0, 1.0)
    }

    /// Construct a matrix for orthographic projection.
    pub fn ortho(right: f32, left: f32, top: f32, bottom: f32, far: f32, near: f32) -> Self {
        Self::new(2.0 / (right - left),                  0.0,                 0.0, -(right + left) / (right - left),
                                   0.0, 2.0 / (top - bottom),                 0.0, -(top + bottom) / (top - bottom),
                                   0.0,                  0.0, -2.0 / (far - near),     -(far + near) / (far - near),
                                   0.0,                  0.0,                 0.0,                              1.0) 

    }

    /// Construct a matrix for perspective projection.
    pub fn persp(right: f32, left: f32, top: f32, bottom: f32, far: f32, near: f32) -> Self {
        Self::new((2.0 * near) / (right - left),                           0.0, (right + left) / (right - left),                                0.0,
                                            0.0, (2.0 * near) / (top - bottom), (top + bottom) / (top - bottom),                                0.0,
                                            0.0,                           0.0,    -(far + near) / (far - near), (-2.0 * far * near) / (far - near),
                                            0.0,                           0.0,                            -1.0,                                0.0)
    }

    /// Construct a matrix for perspective projection.
    pub fn persp_aspect(aspect: f32, vfov: f32, far: f32, near: f32) -> Self {
        let c = 1.0 / (vfov / 2.0).tan();
        Self::new(c / aspect, 0.0,                          0.0,                                0.0,
                         0.0,   c,                          0.0,                                0.0,
                         0.0, 0.0, -(far + near) / (far - near), (-2.0 * far * near) / (far - near),
                         0.0, 0.0,                         -1.0,                                0.0)
    }
}

/// Types that supports matrix transformations.
pub trait Transform {
    fn transform(&self, matrix: &Matrix) -> Self;
    fn transform_self(&mut self, matrix: &Matrix);
}

impl Transform for Matrix {
    /// Concatenates matrix transformations together.
    fn transform(&self, matrix: &Self) -> Self {
        Matrix::new((matrix.m11 * self.m11) + (matrix.m12 * self.m21) + (matrix.m13 * self.m31) + (matrix.m14 * self.m41),
                    (matrix.m11 * self.m12) + (matrix.m12 * self.m22) + (matrix.m13 * self.m32) + (matrix.m14 * self.m42),
                    (matrix.m11 * self.m13) + (matrix.m12 * self.m23) + (matrix.m13 * self.m33) + (matrix.m14 * self.m43),
                    (matrix.m11 * self.m14) + (matrix.m12 * self.m24) + (matrix.m13 * self.m34) + (matrix.m14 * self.m44),
                    (matrix.m21 * self.m11) + (matrix.m22 * self.m21) + (matrix.m23 * self.m31) + (matrix.m24 * self.m41),
                    (matrix.m21 * self.m12) + (matrix.m22 * self.m22) + (matrix.m23 * self.m32) + (matrix.m24 * self.m42),
                    (matrix.m21 * self.m13) + (matrix.m22 * self.m23) + (matrix.m23 * self.m33) + (matrix.m24 * self.m43),
                    (matrix.m21 * self.m14) + (matrix.m22 * self.m24) + (matrix.m23 * self.m34) + (matrix.m24 * self.m44),
                    (matrix.m31 * self.m11) + (matrix.m32 * self.m21) + (matrix.m33 * self.m31) + (matrix.m34 * self.m41),
                    (matrix.m31 * self.m12) + (matrix.m32 * self.m22) + (matrix.m33 * self.m32) + (matrix.m34 * self.m42),
                    (matrix.m31 * self.m13) + (matrix.m32 * self.m23) + (matrix.m33 * self.m33) + (matrix.m34 * self.m43),
                    (matrix.m31 * self.m14) + (matrix.m32 * self.m24) + (matrix.m33 * self.m34) + (matrix.m34 * self.m44),
                    (matrix.m41 * self.m11) + (matrix.m42 * self.m21) + (matrix.m43 * self.m31) + (matrix.m44 * self.m41),
                    (matrix.m41 * self.m12) + (matrix.m42 * self.m22) + (matrix.m43 * self.m32) + (matrix.m44 * self.m42),
                    (matrix.m41 * self.m13) + (matrix.m42 * self.m23) + (matrix.m43 * self.m33) + (matrix.m44 * self.m43),
                    (matrix.m41 * self.m14) + (matrix.m42 * self.m24) + (matrix.m43 * self.m34) + (matrix.m44 * self.m44))
    }

    /// Concatenates matrix transformations into itself.
    fn transform_self(&mut self, matrix: &Matrix) {
        *self = self.transform(matrix);
    }
}

impl std::ops::Neg for Matrix {
    type Output = Matrix;

    fn neg(self) -> Self::Output {
        Self::new(-self.m11, -self.m12, -self.m13, -self.m14,
                  -self.m21, -self.m22, -self.m23, -self.m24,
                  -self.m31, -self.m32, -self.m33, -self.m34,
                  -self.m41, -self.m42, -self.m43, -self.m44)
    }
}

impl<T: Transform> std::ops::Mul<T> for Matrix {
    type Output = T;

    fn mul(self, rhs: T) -> Self::Output {
        rhs.transform(&self)
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
}

op_impl!(matrix, Add, add, AddAssign, add_assign, +);
op_impl!(matrix, Sub, sub, SubAssign, sub_assign, -);
op_impl!(uniform, commutative, f32, Mul, mul, MulAssign, mul_assign, *);
op_impl!(uniform, f32, Div, div, DivAssign, div_assign, /);

