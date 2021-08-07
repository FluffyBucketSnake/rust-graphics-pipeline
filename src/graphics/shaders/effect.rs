use sdl2::pixels::Color;

use crate::graphics::Vertex;

pub trait Effect<V: Vertex> {
    fn vs(&self, input: &V) -> V;
    fn ps(&self, input: &V) -> Color;
}