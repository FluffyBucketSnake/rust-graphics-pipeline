use cgmath::prelude::*;
use cgmath::Vector3;
use super::vertex::Vertex;

/// Line primitive.
#[derive(Clone, Copy)]
pub struct Line<T>(pub T, pub T);

/// Triangle primitive.
#[derive(Clone, Copy)]
pub struct Triangle<T>(pub T, pub T, pub T);

/// The winding order of polygon. Used for determining the front face of a primitive. 
/// 
/// Only really used on triangles, since lines do not have a face.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WindingOrder {
    CounterClockwise,
    Clockwise,
    Both,
}

impl Triangle<Vertex> {
    /// Returns the winding order of this triangle.
    pub fn order(&self) -> WindingOrder {
        let u = (self.1.position - self.0.position).truncate();
        let v = (self.2.position - self.0.position).truncate();
        
        let p = -self.0.position.truncate();
        let n = Vector3::cross(u, v);

        let dot = Vector3::dot(p, n);
        if dot <= 0.0 { WindingOrder::CounterClockwise }
        else { WindingOrder::Clockwise }
    }
}
