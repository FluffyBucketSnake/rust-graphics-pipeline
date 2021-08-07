use crate::graphics::IndexedPrimitive;
use crate::graphics::WindingOrder;
use crate::graphics::{Primitive, Vertex};

#[derive(Clone, Copy)]
pub struct Line<T>(pub T, pub T);

impl<V: Vertex> Primitive<V> for Line<V> {
    fn get_winding(&self) -> WindingOrder { WindingOrder::Both }
}

impl<V: Vertex> IndexedPrimitive<V> for Line<usize> {
    type Unindexed = Line<V>;

    fn unindex(&self, vertices: &[V]) -> Self::Unindexed { 
        Line(vertices[self.0], vertices[self.1])
    }
}