use cgmath::Vector4;
use cgmath::prelude::*;
use cgmath::Matrix4;
use sdl2::surface::Surface;
use sdl2::sys::SDL_BIG_ENDIAN;
use sdl2::sys::SDL_BYTEORDER;

use crate::math::Color;

use super::{ColorTextureVertex, ColorVertex, Vertex};

pub trait Effect<V: Vertex> {
    fn vs(&self, input: &V) -> V;
    fn ps(&self, input: &V) -> Color;
}

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

fn sample_texture(texture: &Surface, x: f32, y: f32) -> Vector4<f32> {
    // Source: https://stackoverflow.com/questions/53033971/how-to-get-the-color-of-a-specific-pixel-from-sdl-surface
    let raw = unsafe {
        let raw_surface = texture.raw();
        let bpp = (*(*raw_surface).format).BytesPerPixel as isize;
        let pitch = (*raw_surface).pitch as isize;
        let x = x as isize;
        let y = y as isize;
        let p = ((*texture.raw()).pixels as *mut u8).offset((y * pitch) + (x * bpp));
        match bpp {
            1 => *p as u32,
            2 => *(p as *mut u16) as u32,
            3 => {
                if SDL_BYTEORDER == SDL_BIG_ENDIAN {
                    (*p as u32) << 16 | (*p.offset(1) as u32) << 8 | *p.offset(2) as u32
                } else {
                    (*p as u32) | (*p.offset(1) as u32) << 8 | (*p.offset(2) as u32) << 16
                }
            }
            4 => *(p as *mut u32),
            _ => unreachable!(),
        }
    };
    crate::math::color_to_vector4(&Color::from_u32(&texture.pixel_format(), raw))
}