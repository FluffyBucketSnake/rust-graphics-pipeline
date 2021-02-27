use crate::math::Matrix;
use crate::vertex::Vertex;
use super::{BitmapOutput, GPU};
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

impl<B: BitmapOutput> GPU<(&[Vertex],&[(usize, usize)]), B> for Pipeline {
    fn draw(&self, (vertices, primitives): (&[Vertex],&[(usize, usize)]), target: &mut B) {
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
            self.rasterizer.draw((vertices[primitive.0], vertices[primitive.1]), target);
        }
    }
}

impl<B: BitmapOutput> GPU<&[(Vertex, Vertex)], B> for Pipeline {
    fn draw(&self, primitives: &[(Vertex, Vertex)], target: &mut B) {
        for primitive in primitives {
            // Copy input data.
            let (mut start, mut end) = primitive;

            // Vertex stage.
            self.vertex_processor(&mut start);
            self.vertex_processor(&mut end);

            // Primitive stage.
            // Render primitive.
            self.rasterizer.draw((start, end), target);
        }
    }
}
