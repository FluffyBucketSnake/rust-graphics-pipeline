use crate::math::Matrix;
use crate::vertex::Vertex;
use super::{BitmapOutput, GPU, primitives::WindingOrder};
use super::primitives::{Line, Triangle};
use super::raster::Rasterizer;

/// A software implementation of a raster graphics processor pipeline.
/// 
/// It accepts a collection of primitives as input, while output a raster render of the scene
/// into the specified `BitmapOutput`.
pub struct Pipeline {
    rasterizer: Rasterizer,
    worldviewproj: Matrix,
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            rasterizer: Rasterizer::new(),
            worldviewproj: Matrix::identity(),
        }
    }

    pub fn worldviewproj(&self) -> Matrix {
        self.worldviewproj
    }

    pub fn set_worldviewproj(&mut self, value: Matrix) {
        self.worldviewproj = value;
    }

    fn vertex_processor(&self, vertex: &mut Vertex) {
        // Convert world coordinates to clip space.
        vertex.position = self.worldviewproj * vertex.position;

        // Perspective division.
        vertex.position /= -vertex.position.z;
    }
}

impl<B: BitmapOutput> GPU<(&[Vertex],&[Line<usize>]), B> for Pipeline {
    fn draw(&self, (vertices, primitives): (&[Vertex],&[Line<usize>]), target: &mut B) {
        // Copy input data.
        let mut vertices = vertices.to_vec();
        let mut primitives = primitives.to_vec();

        // Vertex stage.
        for vertex in vertices.iter_mut() {
            self.vertex_processor(vertex);
        }

        // Primitive stage.
        for primitive in primitives {
            // Raster primitive.
            self.rasterizer.draw(Line(vertices[primitive.0], vertices[primitive.1]), target);
        }
    }
}

impl<B: BitmapOutput> GPU<&[Line<Vertex>], B> for Pipeline {
    fn draw(&self, primitives: &[Line<Vertex>], target: &mut B) {
        for primitive in primitives {
            // Copy input data.
            let Line(mut start, mut end) = primitive;

            // Vertex stage.
            self.vertex_processor(&mut start);
            self.vertex_processor(&mut end);

            // Primitive stage.
            // Render primitive.
            self.rasterizer.draw(Line(start, end), target);
        }
    }
}

impl<B: BitmapOutput> GPU<(&[Vertex],&[Triangle<usize>]), B> for Pipeline {
    fn draw(&self, (vertices, primitives): (&[Vertex],&[Triangle<usize>]), target: &mut B) {
        // Copy input data.
        let mut vertices = vertices.to_vec();
        let mut primitives = primitives.to_vec();

        // Vertex stage.
        for vertex in vertices.iter_mut() {
            self.vertex_processor(vertex);
        }

        // Primitive stage.
        for primitive in primitives {
            // Dereference triangle vertices.
            let primitive = Triangle(vertices[primitive.0], vertices[primitive.1], vertices[primitive.2]);

            // Back-face culling.
            match primitive.order() {
                WindingOrder::Clockwise => {
                    // This face has been culled.
                    continue;
                },
                WindingOrder::CounterClockwise
                | WindingOrder::Both => {},
            }

            // Raster primitive.
            // Only wireframe mode is active currently.
            self.rasterizer.draw(Line(primitive.0, primitive.1), target);
            self.rasterizer.draw(Line(primitive.1, primitive.2), target);
            self.rasterizer.draw(Line(primitive.2, primitive.0), target);
        }
    }
}

impl<B: BitmapOutput> GPU<&[Triangle<Vertex>], B> for Pipeline {
    fn draw(&self, primitives: &[Triangle<Vertex>], target: &mut B) {
        for primitive in primitives {
            // Copy input data.
            let Triangle(mut e0, mut e1, mut e2) = primitive;

            // Vertex stage.
            self.vertex_processor(&mut e0);
            self.vertex_processor(&mut e1);
            self.vertex_processor(&mut e2);

            // Primitive stage.
            // Back-face culling.
            match Triangle(e0, e1, e2).order() {
                WindingOrder::Clockwise => {
                    // This face has been culled.
                    continue;
                },
                WindingOrder::CounterClockwise
                | WindingOrder::Both => {},
            }

            // Raster primitive.
            self.rasterizer.draw(Line(primitive.0, primitive.1), target);
            self.rasterizer.draw(Line(primitive.1, primitive.2), target);
            self.rasterizer.draw(Line(primitive.2, primitive.0), target);
        }
    }
}
