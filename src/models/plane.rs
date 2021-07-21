use cgmath::Vector3;

use crate::graphics::{Line, Triangle};

use super::ModelBuilder;

pub struct Plane {
    a: Vector3<f32>,
    b: Vector3<f32>,
}

impl Plane {
    pub fn new(a: Vector3<f32>, b: Vector3<f32>) -> Self {
        Self { a, b }
    }

    fn _build_vertices(&self) -> Vec<Vector3<f32>> {
        let mut vertices = Vec::new();
        vertices.push(-self.a + -self.b);
        vertices.push(self.a + -self.b);
        vertices.push(-self.a +  self.b);
        vertices.push(self.a +  self.b);
        vertices
    }
}

impl ModelBuilder<Triangle<usize>, Vector3<f32>> for Plane {
    fn build_primitives(&mut self) -> Vec<Triangle<usize>> {
        let mut primitives = Vec::new();
        primitives.push(Triangle(0, 2, 1));
        primitives.push(Triangle(2, 3, 1));
        primitives
    }

    fn build_vertices(&mut self) -> Vec<Vector3<f32>> {
        self._build_vertices()
    }
}

impl ModelBuilder<Line<usize>, Vector3<f32>> for Plane {
    fn build_primitives(&mut self) -> Vec<Line<usize>> {
        let mut primitives = Vec::new();
        primitives.push(Line(0, 1));
        primitives.push(Line(1, 3));
        primitives.push(Line(3, 2));
        primitives.push(Line(2, 0));
        primitives
    }

    fn build_vertices(&mut self) -> Vec<Vector3<f32>> {
        self._build_vertices()
    }
}