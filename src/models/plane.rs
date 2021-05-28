use sdl2::pixels::Color;

use cgmath::Vector3;

use crate::graphics::{Line, Triangle, ColorVertex};

use super::{IndexedLineList, IndexedTriangleList};

pub struct Plane {
    a: Vector3<f32>,
    b: Vector3<f32>,
}

impl Plane {
    pub fn new(a: Vector3<f32>, b: Vector3<f32>) -> Self {
        Self { a, b }
    }

    fn build_vertices(&self) -> Vec<ColorVertex> {
        let mut vertices = Vec::new();
        let p0 = -self.a + -self.b;
        let p1 =  self.a + -self.b;
        let p2 = -self.a +  self.b;
        let p3 =  self.a +  self.b;
        vertices.push(ColorVertex::new(p0.x, p0.y, p0.z, Color::RED));
        vertices.push(ColorVertex::new(p1.x, p1.y, p1.z, Color::GREEN));
        vertices.push(ColorVertex::new(p2.x, p2.y, p2.z, Color::BLUE));
        vertices.push(ColorVertex::new(p3.x, p3.y, p3.z, Color::WHITE));
        vertices
    }
}

impl Into<IndexedTriangleList<ColorVertex>> for &Plane {
    fn into(self) -> IndexedTriangleList<ColorVertex> {
        let vertices = self.build_vertices();
        
        let mut primitives = Vec::new();
        primitives.push(Triangle(0, 2, 1));
        primitives.push(Triangle(2, 3, 1));

        IndexedTriangleList { vertices, primitives }
    }
}

impl Into<IndexedLineList<ColorVertex>> for &Plane {
    fn into(self) -> IndexedLineList<ColorVertex> {
        let vertices = self.build_vertices();
        
        let mut primitives = Vec::new();
        primitives.push(Line(0, 1));
        primitives.push(Line(1, 3));
        primitives.push(Line(3, 2));
        primitives.push(Line(2, 0));

        IndexedLineList { vertices, primitives }
    }
}