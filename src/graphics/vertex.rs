use cgmath::Vector4;
use sdl2::pixels::Color;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: Vector4<f32>,
    pub color: Vector4<f32>,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, color: Color) -> Vertex {
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

impl Add for Vertex {
    type Output = Vertex;
    fn add(self, rhs: Vertex) -> Self::Output {
        let position = self.position + rhs.position;
        let color = self.color + rhs.color;

        Vertex { position, color }
    }
}

impl AddAssign for Vertex {
    fn add_assign(&mut self, rhs: Vertex) {
        *self = *self + rhs;
    }
}

impl Div<f32> for Vertex {
    type Output = Vertex;

    fn div(self, rhs: f32) -> Self::Output {
        let position = self.position / rhs;
        let color = self.color / rhs;

        Self { position, color }
    }
}

impl Mul<Vertex> for f32 {
    type Output = Vertex;

    fn mul(self, rhs: Vertex) -> Self::Output {
        let position = rhs.position * self;
        let color = rhs.color * self;

        Vertex { position, color }
    }
}

impl Mul<f32> for Vertex {
    type Output = Vertex;

    fn mul(self, rhs: f32) -> Self::Output {
        let position = self.position * rhs;
        let color = self.color * rhs;

        Vertex { position, color }
    }
}

impl Sub for Vertex {
    type Output = Vertex;
    fn sub(self, rhs: Vertex) -> Self::Output {
        let position = self.position - rhs.position;
        let color = self.color - rhs.color;

        Vertex { position, color }
    }
}
