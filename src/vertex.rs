use cgmath::Vector4;
use sdl2::pixels::Color;
use std::ops::{Add, AddAssign, Div, Sub,};

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: Vector4<f32>,
    pub color: Color,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, color: Color) -> Vertex {
        Self {
            position: Vector4::new(x, y, z, 1.0),
            color,
        }
    }
}

impl Add for Vertex {
    type Output = Vertex;
    fn add(self, rhs: Vertex) -> Self::Output {
        let position = self.position + rhs.position;

        // TODO: Refactor color operation code.
        let color = Color::from((
            self.color.r + rhs.color.r, // R
            self.color.g + rhs.color.g, // G
            self.color.b + rhs.color.b, // B
            self.color.a + rhs.color.a, // A
        ));
        
        Vertex {
            position,
            color,
        }
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

        // TODO: Refactor color operation code.
        let color = Color::from((
            (self.color.r as f32 / rhs) as u8, // R
            (self.color.g as f32 / rhs) as u8, // G
            (self.color.b as f32 / rhs) as u8, // B
            (self.color.a as f32 / rhs) as u8, // A
        ));
        
        Vertex {
            position,
            color,
        }
    }
}

impl Sub for Vertex {
    type Output = Vertex;
    fn sub(self, rhs: Vertex) -> Self::Output {
        let position = self.position - rhs.position;

        // TODO: Refactor color operation code.
        let color = Color::from((
            self.color.r - rhs.color.r, // R
            self.color.g - rhs.color.g, // G
            self.color.b - rhs.color.b, // B
            self.color.a - rhs.color.a, // A
        ));
        
        Vertex {
            position,
            color,
        }
    }
}