use crate::graphics::{Line, Triangle, Vertex};

/// A model made of a line list.
pub struct LineList<V: Vertex> {
    pub primitives: Vec<Line<V>>,
}

/// A model made of a line list with indexed vertices.
pub struct IndexedLineList<V: Vertex> {
    pub vertices: Vec<V>,
    pub primitives: Vec<Line<usize>>,
}

/// A model made of a triangle list.
pub struct TriangleList<V: Vertex> {
    pub primitives: Vec<Triangle<V>>,
}

/// A model made of a triangle list with indexed vertices.
pub struct IndexedTriangleList<V: Vertex> {
    pub vertices: Vec<V>,
    pub primitives: Vec<Triangle<usize>>,
}