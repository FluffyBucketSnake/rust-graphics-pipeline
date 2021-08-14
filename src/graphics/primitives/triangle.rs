use cgmath::Vector3;
use cgmath::prelude::*;

use crate::graphics::{IndexedPrimitive, Primitive, Vertex, WindingOrder};

#[derive(Clone, Copy)]
pub struct Triangle<T>(pub T, pub T, pub T);

impl<V: Vertex> Primitive<V> for Triangle<V> {
    fn get_winding(&self) -> WindingOrder {
        let mut p0 = self.0.position();
        let mut p1 = self.1.position();
        let mut p2 = self.2.position();

        p0 /= p0.w;
        p1 /= p1.w;
        p2 /= p2.w;

        let u = p1 - p0;
        let v = p2 - p0;

        let normal = Vector3::cross(u.truncate(), v.truncate());

        let dot = normal.z; // n * (0, 0, 1) = n.z
        if dot > 0.0 { WindingOrder::CounterClockwise }
        else if dot < 0.0 { WindingOrder::Clockwise }
        else { WindingOrder::Both }
    }
}

impl<V: Vertex> IndexedPrimitive<V> for Triangle<usize> {
    type Unindexed = Triangle<V>;

    fn unindex(&self, vertices: &[V]) -> Self::Unindexed {
        Triangle(vertices[self.0], vertices[self.1], vertices[self.2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cgmath::vec4;

    #[test]
    fn triangle_front_face() {
        let triangle = Triangle(
            vec4(0.5, -0.253404379, 0.557841897, 0.756527364),
            vec4(-0.5, -0.253404379, 0.557841897, 0.756527364),
            vec4(0.5, -0.66014105, 1.4732163, 1.67007279),
        );

        assert_eq!(WindingOrder::CounterClockwise, triangle.get_winding())
    }
}