use sdl2::pixels::Color;

use crate::graphics::{Line, Triangle, ColorVertex};

use super::{IndexedLineList, IndexedTriangleList};

/// Represents a simple cube.
pub struct Cube {
    pub length: f32,
}

impl Cube {
    /// Builds a new cube model.
    pub fn new(length: f32) -> Self {
        Self { length }
    }

    fn build_vertices(&self) -> Vec<ColorVertex> {
        let hl = self.length / 2.0;

        let mut vertices = Vec::new();
        vertices.push(ColorVertex::new(-hl, -hl, -hl, Color::RED));
        vertices.push(ColorVertex::new( hl, -hl, -hl, Color::GREEN));
        vertices.push(ColorVertex::new(-hl,  hl, -hl, Color::BLUE));
        vertices.push(ColorVertex::new( hl,  hl, -hl, Color::YELLOW));
        vertices.push(ColorVertex::new(-hl, -hl,  hl, Color::CYAN));
        vertices.push(ColorVertex::new( hl, -hl,  hl, Color::MAGENTA));
        vertices.push(ColorVertex::new(-hl,  hl,  hl, Color::WHITE));
        vertices.push(ColorVertex::new( hl,  hl,  hl, Color::BLACK));
        vertices
    }
}

impl Into<IndexedTriangleList<ColorVertex>> for &Cube {
    fn into(self) -> IndexedTriangleList<ColorVertex> {
        let vertices = self.build_vertices();
        
        let mut primitives = Vec::new();
        primitives.push(Triangle(0, 2, 1));
        primitives.push(Triangle(2, 3, 1));
        primitives.push(Triangle(1, 3, 5));
        primitives.push(Triangle(3, 7, 5));
        primitives.push(Triangle(2, 6, 3));
        primitives.push(Triangle(3, 6, 7));
        primitives.push(Triangle(4, 5, 7));
        primitives.push(Triangle(4, 7, 6));
        primitives.push(Triangle(0, 4, 2));
        primitives.push(Triangle(2, 4, 6));
        primitives.push(Triangle(0, 1, 4));
        primitives.push(Triangle(1, 5, 4));

        IndexedTriangleList { vertices, primitives }
    }
}

impl Into<IndexedLineList<ColorVertex>> for &Cube {
    fn into(self) -> IndexedLineList<ColorVertex> {
        let vertices = self.build_vertices();

        let mut primitives = Vec::new();
        primitives.push(Line(0, 1));
        primitives.push(Line(0, 2));
        primitives.push(Line(0, 4));
        primitives.push(Line(1, 3));
        primitives.push(Line(1, 5));
        primitives.push(Line(2, 3));
        primitives.push(Line(3, 7));
        primitives.push(Line(2, 6));
        primitives.push(Line(4, 5));
        primitives.push(Line(4, 6));
        primitives.push(Line(5, 7));
        primitives.push(Line(6, 7));

        IndexedLineList { vertices, primitives }
    }
}
