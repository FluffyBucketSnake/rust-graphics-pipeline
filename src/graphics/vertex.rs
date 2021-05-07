use cgmath::Vector4;
use sdl2::pixels::Color;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

#[derive(Clone, Copy)]
pub struct ColorVertex {
    pub position: Vector4<f32>,
    pub color: Vector4<f32>,
}

impl ColorVertex {
    pub fn new(x: f32, y: f32, z: f32, color: Color) -> ColorVertex {
        Self {
            position: Vector4::new(x, y, z, 1.0),
            color: crate::math::color_to_vector4(&color),
        }
    }

    pub fn interpolate(&self, other: &Self, t: f32) -> Self {
        let delta = *other - *self;
        *self + (t * delta)
    }
}

impl Add for ColorVertex {
    type Output = ColorVertex;
    fn add(self, rhs: ColorVertex) -> Self::Output {
        let position = self.position + rhs.position;
        let color = self.color + rhs.color;

        ColorVertex { position, color }
    }
}

impl AddAssign for ColorVertex {
    fn add_assign(&mut self, rhs: ColorVertex) {
        *self = *self + rhs;
    }
}

impl Div<f32> for ColorVertex {
    type Output = ColorVertex;

    fn div(self, rhs: f32) -> Self::Output {
        let position = self.position / rhs;
        let color = self.color / rhs;

        Self { position, color }
    }
}

impl Mul<ColorVertex> for f32 {
    type Output = ColorVertex;

    fn mul(self, rhs: ColorVertex) -> Self::Output {
        let position = rhs.position * self;
        let color = rhs.color * self;

        ColorVertex { position, color }
    }
}

impl Mul<f32> for ColorVertex {
    type Output = ColorVertex;

    fn mul(self, rhs: f32) -> Self::Output {
        let position = self.position * rhs;
        let color = self.color * rhs;

        ColorVertex { position, color }
    }
}

impl Sub for ColorVertex {
    type Output = ColorVertex;
    fn sub(self, rhs: ColorVertex) -> Self::Output {
        let position = self.position - rhs.position;
        let color = self.color - rhs.color;

        ColorVertex { position, color }
    }
}
