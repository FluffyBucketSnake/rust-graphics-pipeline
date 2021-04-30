use super::clipping::clip_line;
use super::primitives::{Line, Triangle};
use super::raster::Rasterizer;
use super::BitmapOutput;
use crate::vertex::Vertex;
use cgmath::{Matrix4, Vector3};

/// A software implementation of a raster graphics processor pipeline.
///
/// It accepts a collection of primitives as input, while output a raster render of the scene
/// into the specified `BitmapOutput`.
pub struct Pipeline {
    rasterizer: Rasterizer,
    worldviewproj: Matrix4<f32>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            rasterizer: Rasterizer::new(),
            worldviewproj: Matrix4::from_scale(1.0),
        }
    }
    
    pub fn set_worldviewproj(&mut self, value: Matrix4<f32>) {
        self.worldviewproj = value;
    }

    pub fn rasterizer_mut(&mut self) -> &mut Rasterizer {
        &mut self.rasterizer
    }

    pub fn draw_lines<B: BitmapOutput>(&self, primitives: &[Line<Vertex>], target: &mut B) {
        // Copy input.
        let mut primitives = primitives.to_vec();

        // Render each primitive
        for primitive in primitives.iter_mut() {
            // Vertex stage.
            self.vertex_processor(&mut primitive.0);
            self.vertex_processor(&mut primitive.1);

            // Primitive stage.
            if !self.line_processor(primitive, target.size()) {
                // Primitive has been discarded.
                continue;
            }

            // Rasterization.
            self.rasterizer.draw_line(*primitive, target);
        }
    }

    pub fn draw_indexed_lines<B: BitmapOutput>(&self, vertices: &[Vertex], primitives: &[Line<usize>], target: &mut B) {
        // Copy input data.
        let mut vertices = vertices.to_vec();
        let primitives = primitives.to_vec();

        // Vertex stage.
        for vertex in vertices.iter_mut() {
            self.vertex_processor(vertex);
        }

        for primitive in primitives {
            // Primitive stage.
            let mut line = Line(vertices[primitive.0], vertices[primitive.1]);  // Primitive assembly
            if !self.line_processor(&mut line, target.size()) {
                // Primitive has been discarded.
                continue;
            }

            // Rasterization.
            self.rasterizer.draw_line(line, target);
        }
    }
    
    pub fn draw_triangles<B: BitmapOutput>(&self, primitives: &[Triangle<Vertex>], target: &mut B) {
        // Copy input buffer.
        let mut primitives = primitives.to_vec();

        for mut primitive in primitives {
            // Vertex stage.
            self.vertex_processor(&mut primitive.0);
            self.vertex_processor(&mut primitive.1);
            self.vertex_processor(&mut primitive.2);

            // Primitive stage.
            // TODO

            // Raster primitive.
            self.rasterizer.draw_triangle(primitive, target);
        }
    }

    pub fn draw_indexed_triangles<B: BitmapOutput>(&self, vertices: &[Vertex], primitives: &[Triangle<usize>], target: &mut B) {
        // Copy input data.
        let mut vertices = vertices.to_vec();
        let primitives = primitives.to_vec();

        // Vertex stage.
        for vertex in vertices.iter_mut() {
            self.vertex_processor(vertex);
        }

        for primitive in primitives {
            // Primitive stage.
            // Primitive assembly.
            let primitive = Triangle(
                vertices[primitive.0],
                vertices[primitive.1],
                vertices[primitive.2],
            );
            // TODO

            // Raster primitive.
            self.rasterizer.draw_triangle(primitive, target);
        }
    }

    fn vertex_processor(&self, vertex: &mut Vertex) {
        // Apply the World-View-Projection to the vertex position.
        vertex.position = self.worldviewproj * vertex.position;
    }

    fn line_processor(&self, line: &mut Line<Vertex>, screen: (u32, u32)) -> bool {
        // Clip lines outside the window.
        if let Some(cline) = clip_line(*line) {
            *line = cline;
        } else {
            return false;
        }

        // Perspective divide.
        line.0.position /= line.0.position.w;
        line.1.position /= line.1.position.w;

        // Screen mapping phase.
        let (sw, sh) = (screen.0 as f32, screen.1 as f32);
        let transform = Matrix4::from_nonuniform_scale(sw / 2.0, -sh / 2.0, 1.0)
            * Matrix4::from_translation(Vector3::new(1.0, -1.0, 0.0));
        line.0.position = transform * line.0.position;
        line.1.position = transform * line.1.position;

        return true;
    }
}
