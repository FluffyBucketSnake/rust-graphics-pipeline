use crate::math::Vec3f;
use sdl2::pixels::Color;

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: Vec3f,
    pub color: Color,
}

impl Vertex {
    pub fn new(position: Vec3f, color: Color) -> Vertex {
        Self {
            position,
            color,
        }
    }
}

