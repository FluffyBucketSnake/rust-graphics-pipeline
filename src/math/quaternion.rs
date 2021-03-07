use super::matrix::Matrix;
use super::vec3::Vec3f;
use super::vec4::Vec4f;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {

    //
    // Constructors
    //
    
    /// Constructs a new quaternion using 4 values as its coordinates.
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Quaternion { x, y, z, w, }
    }

    /// Constructs a new quaternion using a 4D vector.
    pub fn from_vec4(vector: Vec4f) -> Self {
        Self::new(vector.x, vector.y, vector.z, vector.w)
    }

    /// Constructs a new quaternion using a vector as its vector part and a scalar as its real
    /// part.
    pub fn from_vec3(vector: Vec3f, w: f32) -> Self {
        Self::new(vector.x, vector.y, vector.z, w)
    }

    /// Constructs a rotation quaternion using a vector and a angle(in radians).
    pub fn rotation(vector: Vec3f, angle: f32) -> Self {
        Self::from_vec3(angle.sin() * vector, angle.cos())
    }

    //
    // Defaults
    //

    /// 0
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    /// 1
    pub fn identity() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }

    /// i 
    pub fn i() -> Self {
        Self::new(1.0, 0.0, 0.0, 0.0)
    }

    /// j
    pub fn j() -> Self {
        Self::new(0.0, 1.0, 0.0, 0.0)
    }

    /// k
    pub fn k() -> Self {
        Self::new(0.0, 0.0, 1.0, 0.0)
    }
    
    //
    // Properties
    //

    /// Returns the vector part of the quaternion as a `Vec3f`.
    pub fn vector(&self) -> Vec3f {
        Vec3f::new(self.x, self.y, self.z)
    }

    /// Returns the real part of the quaternion.
    pub fn real(&self) -> f32 {
        self.w
    }

    //
    // Operations
    //

    /// Returns the quaternion split into its vector and real parts.
    pub fn split(&self) -> (Vec3f, f32) {
        (self.vector(), self.w)
    }

    /// Returns this quaternion's conjulgate.
    pub fn conjulgate(&self) -> Self {
        Self::from_vec3(-self.vector(), self.w)
    }

    /// Returns this quaternion's squared norm.
    pub fn norm_squared(&self) -> f32 {
        self.vector().length_squared() + self.w.powi(2)
    }

    /// Returns this quaternion's norm.
    pub fn norm(&self) -> f32 {
        self.norm_squared().sqrt()
    }

    /// Returns this quaternion's inverse.
    pub fn inverse(&self) -> Quaternion {
        self.conjulgate() / self.norm_squared()
    }

    /// Calculates the spherical linear interpolation between two quaternions.
    pub fn slerp(&self, other: &Quaternion, t: f32) -> Quaternion {
        let theta = ((self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)).acos();
        let lq = (self * (theta * (1.0 - t)).sin()) / theta.sin();
        let lr = (other * (theta * t).sin()) / theta.sin();
        lq + lr
    }
}

macro_rules! forward_bin_ops {
    ($t1:ident, $t2:ident, $to:ident, $imp:ident, $method:ident) => {

        impl std::ops::$imp<&$t2> for $t1 {
            type Output = $to;

            fn $method(self, rhs: &$t2) -> Self::Output {
                self.$method(*rhs)
            }
        }

        impl std::ops::$imp<$t2> for &$t1 {
            type Output = $to;

            fn $method(self, rhs: $t2) -> Self::Output {
                (*self).$method(rhs)
            }
        }

        impl std::ops::$imp<&$t2> for &$t1 {
            type Output = $to;

            fn $method(self, rhs: &$t2) -> Self::Output {
                self.$method(*rhs)
            }
        }
    };
}

macro_rules! forward_assign_ops {
    ($t1:ident, $t2:ident, $imp:ident, $method:ident) => {
        impl std::ops::$imp<&$t2> for $t1 {
            fn $method(&mut self, rhs: &$t2) {
                self.$method(*rhs)
            }
        }
    };
}

macro_rules! bin_op_impl {
    (real, $op:tt, impl $imp:ident, $method:ident) => {
        impl std::ops::$imp<f32> for Quaternion {
            type Output = Quaternion;

            fn $method(self, rhs: f32) -> Self::Output {
                Quaternion::new(self.x, self.y, self.z, self.w $op rhs)
            }
        }

        forward_bin_ops!(Quaternion, f32, Quaternion, $imp, $method);
    };
    (uniform, $op:tt, impl $imp:ident, $method:ident) => {
        impl std::ops::$imp<f32> for Quaternion {
            type Output = Quaternion;

            fn $method(self, rhs: f32) -> Self::Output {
                Quaternion::new(self.x $op rhs, self.y $op rhs, self.z $op rhs, self.w $op rhs)
            }
        }
        
        forward_bin_ops!(Quaternion, f32, Quaternion, $imp, $method);
    };
    (quaternion, $op:tt, impl $imp:ident, $method:ident) => {
        impl std::ops::$imp for Quaternion {
            type Output = Quaternion;

            fn $method(self, rhs: Quaternion) -> Self::Output {
                Quaternion::new(self.x $op rhs.x, self.y $op rhs.y, self.z $op rhs.z, self.w $op rhs.w)
            }
        }
            
        forward_bin_ops!(Quaternion, Quaternion, Quaternion, $imp, $method);
    };
}

macro_rules! assign_op_impl {
    (real, $op:tt, impl $imp:ident, $method:ident) => {
        impl std::ops::$imp<f32> for Quaternion {
            fn $method(&mut self, rhs: f32) {
                self.w $op rhs;
            }
        }
        
        forward_assign_ops!(Quaternion, f32, $imp, $method);
    };
    (uniform, $op:tt, impl $imp:ident, $method:ident) => {
        impl std::ops::$imp<f32> for Quaternion {
            fn $method(&mut self, rhs: f32) {
                self.x $op rhs;
                self.y $op rhs;
                self.z $op rhs;
                self.w $op rhs;
            }
        }
        
        forward_assign_ops!(Quaternion, f32, $imp, $method);
    };
    (quaternion, $op:tt, impl $imp:ident, $method:ident) => {
        impl std::ops::$imp for Quaternion {
            fn $method(&mut self, rhs: Quaternion) {
                self.x $op rhs.x;
                self.y $op rhs.y;
                self.z $op rhs.z;
                self.w $op rhs.w;
            }
        }
            
        forward_assign_ops!(Quaternion, Quaternion, $imp, $method);
    };
}

macro_rules! commutative_impl {
    ($t:ty, impl $imp:ident, $method:ident) => {
        impl std::ops::$imp<Quaternion> for $t {
            type Output = Quaternion;

            fn $method(self, rhs: Quaternion) -> Self::Output {
                rhs.$method(self)
            }
        }

        forward_bin_ops!(f32, Quaternion, Quaternion, $imp, $method);
    };
}

impl From<Vec4f> for Quaternion {
    fn from(v: Vec4f) -> Self {
        Quaternion::from_vec4(v)
    }
}

impl From<f32> for Quaternion {
    fn from(real: f32) -> Self {
        Quaternion::new(0.0, 0.0, 0.0, real)
    }
}

impl From<Quaternion> for Matrix {
    fn from(q: Quaternion) -> Self {
        let s = 2.0 / q.norm_squared();
        Matrix::new(1.0 - (s * (q.y.powi(2) + q.z.powi(2))), s * ((q.x * q.y) - (q.z * q.w)), s * ((q.x * q.z) + (q.y * q.w)), 0.0,
                    s * ((q.x * q.y) + (q.z * q.w)), 1.0 - (s * (q.x.powi(2) + q.z.powi(2))), s * ((q.y * q.z) - (q.x * q.w)), 0.0,
                    s * ((q.x * q.z) - (q.y * q.w)), s * ((q.y * q.z) + (q.x * q.w)), 1.0 - (s * (q.x.powi(2) + q.y.powi(2))), 0.0,
                                                0.0,                             0.0,                                     0.0, 1.0)
    }
}

impl std::ops::Neg for Quaternion {
    type Output = Quaternion;

    fn neg(self) -> Self::Output {
        Quaternion::from_vec3(-self.vector(), -self.w)
    }
}

impl std::ops::Mul for Quaternion {
    type Output = Quaternion;

    fn mul(self, other: Quaternion) -> Self::Output {
        let (qv, qw) = self.split();
        let (rv, rw) = other.split();
        Quaternion::from_vec3(qv.cross(&rv) + (rw * qv) + (qw * rv), (qw * rw) - qv.dot(&rv))
    }
}

bin_op_impl!(real, +, impl Add, add);
bin_op_impl!(real, -, impl Sub, sub);
bin_op_impl!(uniform, *, impl Mul, mul);
bin_op_impl!(uniform, /, impl Div, div);
bin_op_impl!(quaternion, +, impl Add, add);
bin_op_impl!(quaternion, -, impl Sub, sub);

commutative_impl!(f32, impl Add, add);
commutative_impl!(f32, impl Sub, sub);
commutative_impl!(f32, impl Mul, mul);

assign_op_impl!(real, +=, impl AddAssign, add_assign);
assign_op_impl!(real, -=, impl SubAssign, sub_assign);
assign_op_impl!(uniform, *=, impl MulAssign, mul_assign);
assign_op_impl!(uniform, /=, impl DivAssign, div_assign);
assign_op_impl!(quaternion, +=, impl AddAssign, add_assign);
assign_op_impl!(quaternion, -=, impl SubAssign, sub_assign);
