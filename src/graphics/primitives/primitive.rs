use crate::graphics::Vertex;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WindingOrder {
    CounterClockwise,
    Clockwise,
    Both,
}

pub trait Primitive<V: Vertex> {
    // TODO: Replace with is_front_face(&self, WindingOrder)
    fn get_winding(&self) -> WindingOrder;
}

pub trait IndexedPrimitive<V: Vertex> {
    type Unindexed: Primitive<V>;

    fn unindex(&self, vertices: &[V]) -> Self::Unindexed;
}