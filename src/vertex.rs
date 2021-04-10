use cgmath::Vector4;
use sdl2::pixels::Color;
use std::ops::{Add, AddAssign, Div, Mul, Sub,};

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
            match self.color.r.checked_add(rhs.color.r) { Some(v) => {v}, None => {255}, }, // R
            match self.color.g.checked_add(rhs.color.g) { Some(v) => {v}, None => {255}, }, // G
            match self.color.b.checked_add(rhs.color.b) { Some(v) => {v}, None => {255}, }, // B
            match self.color.a.checked_add(rhs.color.a) { Some(v) => {v}, None => {255}, }, // A
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

impl Mul<Vertex> for f32 {
    type Output = Vertex;
    
    fn mul(self, rhs: Vertex) -> Self::Output { 
        let position = rhs.position * self;

        // TODO: Refactor color operation code.
        let color = Color::from((
            (rhs.color.r as f32 * self) as u8, // R
            (rhs.color.g as f32 * self) as u8, // G
            (rhs.color.b as f32 * self) as u8, // B
            (rhs.color.a as f32 * self) as u8, // A
        ));
        
        Vertex {
            position,
            color,
        }
    }
}

impl Mul<f32> for Vertex {
    type Output = Vertex;
    
    fn mul(self, rhs: f32) -> Self::Output { 
        let position = self.position * rhs;

        // TODO: Refactor color operation code.
        let color = Color::from((
            (self.color.r as f32 * rhs) as u8, // R
            (self.color.g as f32 * rhs) as u8, // G
            (self.color.b as f32 * rhs) as u8, // B
            (self.color.a as f32 * rhs) as u8, // A
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
            match self.color.r.checked_sub(rhs.color.r) { Some(v) => {v}, None => {0}, }, // R
            match self.color.g.checked_sub(rhs.color.g) { Some(v) => {v}, None => {0}, }, // G
            match self.color.b.checked_sub(rhs.color.b) { Some(v) => {v}, None => {0}, }, // B
            match self.color.a.checked_sub(rhs.color.a) { Some(v) => {v}, None => {0}, }, // A
        ));
        
        Vertex {
            position,
            color,
        }
    }
}