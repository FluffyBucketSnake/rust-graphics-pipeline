use crate::math::Matrix;
use crate::vertex::Vertex;
use super::{BitmapOutput, GPU};
use super::raster::Rasterizer;

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
}

impl<B: BitmapOutput> GPU<&[(Vertex, Vertex)], B> for Pipeline {
    fn draw(&self, primitives: &[(Vertex, Vertex)], target: &mut B) {
        for primitive in primitives {
            let (mut start, mut end) = primitive;

            // Convert world coordinates to clip space.
            start.position = self.worldviewproj * start.position;
            end.position = self.worldviewproj * end.position;

            // Convert coordinates to normalized device coordinates.
            start.position /= start.position.z;
            end.position /= end.position.z;

            // Raster primitive.
            self.rasterizer.draw((start, end), target);
        }
    }
}