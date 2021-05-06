use crate::graphics::{Line, Triangle, Vertex};

/// A model made of a line list.
pub struct LineList {
    pub primitives: Vec<Line<Vertex>>,
}

/// A model made of a line list with indexed vertices.
pub struct IndexedLineList {
    pub vertices: Vec<Vertex>,
    pub primitives: Vec<Line<usize>>,
}

/// A model made of a triangle list.
pub struct TriangleList {
    pub primitives: Vec<Triangle<Vertex>>,
}

/// A model made of a triangle list with indexed vertices.
pub struct IndexedTriangleList {
    pub vertices: Vec<Vertex>,
    pub primitives: Vec<Triangle<usize>>,
}