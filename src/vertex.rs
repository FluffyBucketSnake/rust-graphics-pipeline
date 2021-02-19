use crate::math::Vec2f;
use sdl2::pixels::Color;

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: Vec2f,
    pub color: Color,
}

impl Vertex {
    pub fn new(position: Vec2f, color: Color) -> Vertex {
        Self {
            position,
            color,
        }
    }
}

