use cgmath::prelude::*;
use cgmath::Matrix4;

use crate::math::Color;

use super::ColorVertex;

pub trait Effect {
    fn vs(&self, input: &ColorVertex) -> ColorVertex;
    fn ps(&self, input: &ColorVertex) -> Color;
}

pub struct BasicEffect {
    pub worldviewproj: Matrix4<f32>,
}

impl BasicEffect {
    pub fn new(worldviewproj: Matrix4<f32>) -> Self { 
        Self { worldviewproj } 
    }
}

impl Default for BasicEffect {
    fn default() -> Self {
        Self::new(Matrix4::identity())
    }
}

impl Effect for BasicEffect {
    fn vs(&self, input: &ColorVertex) -> ColorVertex {
        let mut vertex = *input;

        vertex.position = self.worldviewproj * vertex.position;

        vertex
    }

    fn ps(&self, input: &ColorVertex) -> Color {
        crate::math::vector4_to_color(&input.color)
    }
}