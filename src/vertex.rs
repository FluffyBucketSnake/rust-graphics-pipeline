use cgmath::Point3;
use sdl2::pixels::Color;

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: Point3<f32>,
    pub color: Color,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, color: Color) -> Vertex {
        Self {
            position: Point3::new(x, y, z),
            color,
        }
    }
}

