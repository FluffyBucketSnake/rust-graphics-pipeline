use cgmath::Vector4;

use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

pub trait Vertex:
    Add<Output = Self>
    + Sub<Output = Self>
    + AddAssign
    + SubAssign
    + Mul<f32, Output = Self>
    + Div<f32, Output = Self>
    + Clone
    + Copy
    + Sized
{
    fn position(&self) -> Vector4<f32>;
    fn to_screen_coords(&mut self, width: f32, height: f32);
    fn interpolate(&self, other: &Self, t: f32) -> Self;
}