use crate::graphics::{Line, Triangle, ColorVertex};

/// A model made of a line list.
pub struct LineList {
    pub primitives: Vec<Line<ColorVertex>>,
}

/// A model made of a line list with indexed vertices.
pub struct IndexedLineList {
    pub vertices: Vec<ColorVertex>,
    pub primitives: Vec<Line<usize>>,
}

/// A model made of a triangle list.
pub struct TriangleList {
    pub primitives: Vec<Triangle<ColorVertex>>,
}

/// A model made of a triangle list with indexed vertices.
pub struct IndexedTriangleList {
    pub vertices: Vec<ColorVertex>,
    pub primitives: Vec<Triangle<usize>>,
}