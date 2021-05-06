use crate::math::Color;

use super::Vertex;

pub trait Effect {
    fn vs(&self, input: &Vertex) -> Vertex;
    fn ps(&self, input: &Vertex) -> Color;
}