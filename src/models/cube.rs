use cgmath::{Vector3, vec3};

use crate::graphics::{Line, Triangle};

use super::ModelBuilder;

/// Represents a simple cube.
pub struct Cube {
    pub length: f32,
}

impl Cube {
    /// Builds a new cube model.
    pub fn new(length: f32) -> Self {
        Self { length }
    }
    
    fn _build_vertices(&self) -> Vec<Vector3<f32>> {
        let hl = self.length / 2.0;

        let mut vertices = Vec::new();
        vertices.push(vec3(-hl, -hl, -hl));
        vertices.push(vec3( hl, -hl, -hl));
        vertices.push(vec3(-hl,  hl, -hl));
        vertices.push(vec3( hl,  hl, -hl));
        vertices.push(vec3(-hl, -hl,  hl));
        vertices.push(vec3( hl, -hl,  hl));
        vertices.push(vec3(-hl,  hl,  hl));
        vertices.push(vec3( hl,  hl,  hl));
        vertices
    }
}

impl ModelBuilder<Line<usize>, Vector3<f32>> for Cube {
    fn build_vertices(&self) -> Vec<Vector3<f32>> {
        self._build_vertices()
    }

    fn build_primitives(&self) -> Vec<Line<usize>> { 
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
        primitives
    }
}

impl ModelBuilder<Triangle<usize>, Vector3<f32>> for Cube {
    fn build_vertices(&self) -> Vec<Vector3<f32>> {
        self._build_vertices()
    }

    fn build_primitives(&self) -> Vec<Triangle<usize>> {  
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
        primitives
    }
}