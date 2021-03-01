use crate::math::{Matrix, Transform, Vec4f};
use crate::vertex::Vertex;
use super::{BitmapOutput, GPU, primitives::WindingOrder};
use super::primitives::{Line, Triangle};
use super::raster::Rasterizer;

/// A software implementation of a raster graphics processor pipeline.
/// 
/// It accepts a collection of primitives as input, while output a raster render of the scene
/// into the specified `BitmapOutput`.
pub struct Pipeline {
    front_face: WindingOrder,
    rasterizer: Rasterizer,
    worldviewproj: Matrix,
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            front_face: WindingOrder::Both,
            rasterizer: Rasterizer::new(),
            worldviewproj: Matrix::identity(),
        }
    }

    pub fn front_face(&self) -> WindingOrder {
        self.front_face
    }

    pub fn set_front_face(&mut self, order: WindingOrder) {
        self.front_face = order;
    }

    pub fn worldviewproj(&self) -> Matrix {
        self.worldviewproj
    }

    pub fn set_worldviewproj(&mut self, value: Matrix) {
        self.worldviewproj = value;
    }

    fn vertex_processor(&self, vertex: &mut Vertex) {
        // Convert position into a Vec4f.
        let Vertex { position, .. } = vertex;
        let mut position = Vec4f::from_point(*position);

        // Convert world coordinates to clip space.
        position.transform_self(&self.worldviewproj);

        // TODO: Create a proper perspective matrix function.
        // Apply a perspective transformation.
        let proj = Matrix::new(1.0, 0.0,  0.0, 0.0,
                               0.0, 1.0,  0.0, 0.0,
                               0.0, 0.0, -1.0, 0.0,
                               0.0, 0.0, -1.0, 0.0);
        position.transform_self(&proj);

        // Perspective division.
        position.homogenize_self();
        
        // Extract the xyz coordinates back into the vertex data.
        vertex.position = position.xyz();
    }

    fn cull_face(&self, face: &Triangle<Vertex>) -> bool {
        match face.order() {
            WindingOrder::Clockwise => {
                if self.front_face == WindingOrder::CounterClockwise { true }
                else { false }
            },
            WindingOrder::CounterClockwise => {
                if self.front_face == WindingOrder::Clockwise { true }
                else { false }
            },
            WindingOrder::Both => { false },
        }
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
        let mut i = 0usize;
        for primitive in primitives {
            // Dereference triangle vertices.
            let primitive = Triangle(vertices[primitive.0], vertices[primitive.1], vertices[primitive.2]);

            // Back-face culling.
            if self.cull_face(&primitive) {
                // This primitive has beem culled.
                continue;
            }

            // Raster primitive.
            // Only wireframe mode is active currently.
            self.rasterizer.draw(Line(primitive.0, primitive.1), target);
            self.rasterizer.draw(Line(primitive.1, primitive.2), target);
            self.rasterizer.draw(Line(primitive.2, primitive.0), target);
            i += 1;
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
            if self.cull_face(&primitive) {
                // This primitive has beem culled.
                continue;
            }

            // Raster primitive.
            self.rasterizer.draw(Line(primitive.0, primitive.1), target);
            self.rasterizer.draw(Line(primitive.1, primitive.2), target);
            self.rasterizer.draw(Line(primitive.2, primitive.0), target);
        }
    }
}
