use crate::math::Color;

use super::Vertex;

pub trait Effect {
    fn ps(&self, input: &Vertex) -> Vertex;
    fn vs(&self, input: &Vertex) -> Color;
}