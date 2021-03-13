use cgmath::prelude::*;
use cgmath::Vector3;
use crate::vertex::Vertex;

#[derive(Clone, Copy)]
pub struct Line<T>(pub T, pub T);

#[derive(Clone, Copy)]
pub struct Triangle<T>(pub T, pub T, pub T);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WindingOrder {
    CounterClockwise,
    Clockwise,
    Both,
}

impl Triangle<Vertex> {
    pub fn order(&self) -> WindingOrder {
        let u = self.1.position - self.0.position;
        let v = self.2.position - self.0.position;

        let p = -self.0.position.to_vec();
        let n = Vector3::cross(u, v);

        let dot = Vector3::dot(p, n);
        if dot > 0.0 { WindingOrder::CounterClockwise }
        else if dot < 0.0 { WindingOrder::Clockwise }
        else { WindingOrder::Both }
    }
}
