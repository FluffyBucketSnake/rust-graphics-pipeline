use cgmath::{Vector2, Vector4};

use sdl2::pixels::Color;

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
}

impl Vertex for ColorVertex {
    fn position(&self) -> Vector4<f32> {
        self.position
    }

    fn to_screen_coords(&mut self, width: f32, height: f32) {
        self.position /= self.position.w;

        self.position.x += 1.0;
        self.position.x *= width / 2.0;
        self.position.y -= 1.0;
        self.position.y *= -height / 2.0;
    }

    fn interpolate(&self, other: &Self, t: f32) -> Self {
        (*self * (1.0 - t)) + (*other * t)
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
        self.position += rhs.position;
        self.color += rhs.color;
    }
}

impl SubAssign for ColorVertex {
    fn sub_assign(&mut self, rhs: ColorVertex) {
        self.position -= rhs.position;
        self.color -= rhs.color;
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

#[derive(Clone, Copy)]
pub struct ColorTextureVertex {
    pub position: Vector4<f32>,
    pub color: Vector4<f32>,
    pub uv: Vector2<f32>,
}

impl ColorTextureVertex {
    pub fn new(x: f32, y: f32, z: f32, color: Color, u: f32, v: f32) -> ColorTextureVertex {
        Self {
            position: Vector4::new(x, y, z, 1.0),
            color: crate::math::color_to_vector4(&color),
            uv: Vector2::new(u, v),
        }
    }
}

impl Vertex for ColorTextureVertex {
    fn position(&self) -> Vector4<f32> {
        self.position
    }

    fn to_screen_coords(&mut self, width: f32, height: f32) {
        self.position /= self.position.w;

        self.position.x += 1.0;
        self.position.x *= width / 2.0;
        self.position.y -= 1.0;
        self.position.y *= -height / 2.0;
    }

    fn interpolate(&self, other: &Self, t: f32) -> Self {
        (*self * (1.0 - t)) + (*other * t)
    }
}

impl Add for ColorTextureVertex {
    type Output = ColorTextureVertex;
    fn add(self, rhs: ColorTextureVertex) -> Self::Output {
        let position = self.position + rhs.position;
        let color = self.color + rhs.color;
        let uv  = self.uv + rhs.uv;

        ColorTextureVertex { position, color, uv }
    }
}

impl AddAssign for ColorTextureVertex {
    fn add_assign(&mut self, rhs: ColorTextureVertex) {
        self.position += rhs.position;
        self.color += rhs.color;
        self.uv += rhs.uv;
    }
}

impl SubAssign for ColorTextureVertex {
    fn sub_assign(&mut self, rhs: ColorTextureVertex) {
        self.position -= rhs.position;
        self.color -= rhs.color;
        self.uv -= rhs.uv;
    }
}

impl Div<f32> for ColorTextureVertex {
    type Output = ColorTextureVertex;

    fn div(self, rhs: f32) -> Self::Output {
        let position = self.position / rhs;
        let color = self.color / rhs;
        let uv = self.uv / rhs;

        Self { position, color, uv }
    }
}

impl Mul<f32> for ColorTextureVertex {
    type Output = ColorTextureVertex;

    fn mul(self, rhs: f32) -> Self::Output {
        let position = self.position * rhs;
        let color = self.color * rhs;
        let uv = self.uv * rhs;

        ColorTextureVertex { position, color, uv }
    }
}

impl Sub for ColorTextureVertex {
    type Output = ColorTextureVertex;
    fn sub(self, rhs: ColorTextureVertex) -> Self::Output {
        let position = self.position - rhs.position;
        let color = self.color - rhs.color;
        let uv = self.uv - rhs.uv;

        ColorTextureVertex { position, color, uv }
    }
}
