use cgmath::Matrix4;
use cgmath::prelude::*;
use sdl2::pixels::Color;
use sdl2::surface::Surface;

use crate::graphics::{ColorTextureVertex, ColorVertex, Effect, sample_texture};

pub struct BasicEffect<'a> {
    pub worldviewproj: Matrix4<f32>,
    pub texture: Option<Surface<'a>>,
}

impl<'a> BasicEffect<'a> {
    pub fn new(worldviewproj: Matrix4<f32>) -> Self { 
        Self { 
            worldviewproj,
            texture: None,
        } 
    }
}

impl<'a> Default for BasicEffect<'a> {
    fn default() -> Self {
        Self::new(Matrix4::identity())
    }
}

impl<'a> Effect<ColorVertex> for BasicEffect<'a> {
    fn vs(&self, input: &ColorVertex) -> ColorVertex {
        let mut vertex = *input;

        vertex.position = self.worldviewproj * vertex.position;

        vertex
    }

    fn ps(&self, input: &ColorVertex) -> Color {
        crate::math::vector4_to_color(&input.color)
    }
}

impl<'a> Effect<ColorTextureVertex> for BasicEffect<'a> {
    fn vs(&self, input: &ColorTextureVertex) -> ColorTextureVertex {
        let mut vertex = *input;

        vertex.position = self.worldviewproj * vertex.position;

        vertex
    }

    fn ps(&self, input: &ColorTextureVertex) -> Color {
        let mut color = input.color;
        if let Some(texture) = &self.texture {
            let sample = sample_texture(texture, input.uv.x, input.uv.y);
            color.x *= sample.x;
            color.y *= sample.y;
            color.z *= sample.z;
            color.w *= sample.w;
        }
        crate::math::vector4_to_color(&color)
    }
}