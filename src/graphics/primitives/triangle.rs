use cgmath::Vector3;
use cgmath::prelude::*;

use crate::graphics::{IndexedPrimitive, Primitive, Vertex, WindingOrder};

#[derive(Clone, Copy)]
pub struct Triangle<T>(pub T, pub T, pub T);

impl<V: Vertex> Primitive<V> for Triangle<V> {
    fn get_winding(&self) -> WindingOrder {
        let u = (self.1.position() - self.0.position()).truncate();
        let v = (self.2.position() - self.0.position()).truncate();
        
        let p = -self.0.position().truncate();
        let n = Vector3::cross(u, v);

        let dot = Vector3::dot(p, n);
        if dot <= 0.0 { WindingOrder::CounterClockwise }
        else { WindingOrder::Clockwise }
    }
}

impl<V: Vertex> IndexedPrimitive<V> for Triangle<usize> {
    type Unindexed = Triangle<V>;

    fn unindex(&self, vertices: &[V]) -> Self::Unindexed {
        Triangle(vertices[self.0], vertices[self.1], vertices[self.2])
    }
}