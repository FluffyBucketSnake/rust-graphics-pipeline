use super::clipping::{clip_line, clip_triangle};
use super::primitives::{Line, Triangle, WindingOrder};
use super::vertex::Vertex;
use super::BitmapOutput;
use crate::math::mix;
use cgmath::{Matrix4, Vector3, Vector4};
use std::mem::swap;

#[allow(dead_code)]
pub enum FillMode {
    Solid,
    Wireframe,
}

/// A software implementation of a raster graphics processor pipeline.
///
/// It accepts a collection of primitives as input, while output a raster render of the scene
/// into the specified `BitmapOutput`.
pub struct Pipeline {
    pub fill_mode: FillMode,
    pub front_face: WindingOrder,
    pub worldviewproj: Matrix4<f32>,
}

impl Pipeline {
    /// Constructs a new `Pipeline`.
    pub fn new() -> Self {
        Self {
            fill_mode: FillMode::Solid,
            front_face: WindingOrder::CounterClockwise,
            worldviewproj: Matrix4::from_scale(1.0),
        }
    }

    /// Draws multiple lines onto the render target.
    #[allow(dead_code)]
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
            self.render_line(*primitive, target);
        }
    }

    /// Draws multiple lines onto the render target. Allows vertex indexing.
    #[allow(dead_code)]
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
            self.render_line(line, target);
        }
    }
    
    /// Draws multiple triangles onto the render target.
    #[allow(dead_code)]
    pub fn draw_triangles<B: BitmapOutput>(&self, primitives: &[Triangle<Vertex>], target: &mut B) {
        // Copy input buffer.
        let primitives = primitives.to_vec();

        for mut primitive in primitives {
            // Vertex stage.
            self.vertex_processor(&mut primitive.0);
            self.vertex_processor(&mut primitive.1);
            self.vertex_processor(&mut primitive.2);

            // Primitive stage.
            if !self.triangle_processor(&mut primitive, target.size()) {
                // Primitive has been discarded.
                continue;
            }

            // Raster primitive.
            self.render_triangle(primitive, target);
        }
    }

    /// Draws multiple triangles onto the render target. Allows vertex indexing.
    #[allow(dead_code)]
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

            match clip_triangle(primitive) {
                super::clipping::ClippedTriangle::Empty => {}
                super::clipping::ClippedTriangle::One(mut t) => {
                    if self.triangle_processor(&mut t, target.size()) {
                        self.render_triangle(t, target);
                    }
                }
                super::clipping::ClippedTriangle::Two(mut t1, mut t2) => {
                    if self.triangle_processor(&mut t1, target.size()) {
                        self.render_triangle(t1, target);
                    }
                    if self.triangle_processor(&mut t2, target.size()) {
                        self.render_triangle(t2, target);
                    }
                }
            }
            // if self.triangle_processor(&mut primitive, target.size()) {
            //     self.render_triangle(primitive, target);
            // }
        }
    }

    /// Executes the vertex processor onto the input vertex. To be replaced by the vertex shader.
    fn vertex_processor(&self, vertex: &mut Vertex) {
        // Apply the World-View-Projection to the vertex position.
        vertex.position = self.worldviewproj * vertex.position;
    }

    /// Applies the primitive processing stage onto the input line.
    /// Since lines can't go through front face culling, this method only clips the line.
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
        // TODO: Use viewport instead of screen.
        let (sw, sh) = (screen.0 as f32, screen.1 as f32);
        let transform = Matrix4::from_nonuniform_scale(sw / 2.0, -sh / 2.0, 1.0)
            * Matrix4::from_translation(Vector3::new(1.0, -1.0, 0.0));
        line.0.position = transform * line.0.position;
        line.1.position = transform * line.1.position;

        return true;
    }

    /// Applies the primitive processing stage onto the input triangle.
    /// ie. Clipping and front face culling.
    fn triangle_processor(&self, triangle: &mut Triangle<Vertex>, screen: (u32, u32)) -> bool {
        // Perspective divide.
        triangle.0.position /= triangle.0.position.w;
        triangle.1.position /= triangle.1.position.w;
        triangle.2.position /= triangle.2.position.w;

        // Screen mapping phase.
        // TODO: Use viewport instead of screen.
        let (sw, sh) = (screen.0 as f32, screen.1 as f32);
        let transform = Matrix4::from_nonuniform_scale(sw / 2.0, -sh / 2.0, 1.0)
                                 * Matrix4::from_translation(Vector3::new(1.0, -1.0, 0.0));
        triangle.0.position = transform * triangle.0.position;
        triangle.1.position = transform * triangle.1.position;
        triangle.2.position = transform * triangle.2.position;

        // Front-face culling.
        match triangle.order() {
            WindingOrder::Clockwise => {
                if self.front_face == WindingOrder::CounterClockwise { return false }
            },
            WindingOrder::CounterClockwise => {
                if self.front_face == WindingOrder::Clockwise { return false }
            },
            WindingOrder::Both => {},
        };

        true
    }

    /// Renders the line onto the render target. Uses the DDA algorithm.
    fn render_line<B: BitmapOutput>(&self, line: Line<Vertex>, target: &mut B) {
        // Line traversal.
        let delta = line.1 - line.0;
        let step = f32::max(delta.position.x.abs(), delta.position.y.abs());  // Largest axis difference.
        let derivant = delta / step;    // Increment for each axis.
        let mut integrant = line.0;
        let mut i: f32 = 0.0;
        while i < step {
            let Vertex { position: Vector4 { x, y, z: _, w: _}, color: _} = integrant;
            target.put_pixel((x as u32, y as u32), mix(line.0.color, line.1.color, i / f32::max(step - 1.0, 1.0)));
            integrant += derivant;
            i += 1.0;
        }
    }

    /// Renders the line onto the render target.
    fn render_triangle<B: BitmapOutput>(&self, triangle: Triangle<Vertex>, target: &mut B) {
        match self.fill_mode {
            FillMode::Wireframe => {
                self.render_line(Line(triangle.0, triangle.1), target);
                self.render_line(Line(triangle.1, triangle.2), target);
                self.render_line(Line(triangle.2, triangle.0), target);
            },
            FillMode::Solid => {
                // Use references for easy swapping.
                let (mut v0, mut v1, mut v2) = (&triangle.0, &triangle.1, &triangle.2);

                // Sort vertices by y-value. v0.y < v1.y < v2.y
                if v0.position.y > v1.position.y { swap(&mut v0, &mut v1); }
                if v1.position.y > v2.position.y { swap(&mut v1, &mut v2); }
                if v0.position.y > v1.position.y { swap(&mut v0, &mut v1); }

                if v0.position.y == v1.position.y {
                    // Sort top vertices by x. v0.x < v1.x
                    if v0.position.x > v1.position.x { swap(&mut v0, &mut v1) }

                    // Natural flat top.
                    self.draw_flattop_triangle(Triangle(*v0, *v1, *v2), target);
                }
                else if v1.position.y == v2.position.y {
                    // Sort bottom vertices by x. v1.x < v2.x
                    if v1.position.x > v2.position.x { swap(&mut v1, &mut v2) }

                    // Natural bottom top.
                    self.draw_flatbottom_triangle(Triangle(*v0, *v1, *v2), target);
                }
                else {
                    let a = (v1.position - v0.position).y /
                            (v2.position - v0.position).y;

                    // TODO: Create a dedicated interpolation function/trait.
                    let vi = Vertex { 
                        position: v0.position + ((v2.position - v0.position) * a),
                        color: mix(v0.color, v2.color, a),
                    };

                    if v1.position.x > vi.position.x {
                        // Major left
                        self.draw_flatbottom_triangle(Triangle(*v0, vi, *v1), target);
                        self.draw_flattop_triangle(Triangle(vi, *v1, *v2), target);
                    }
                    else {
                        // Major right
                        self.draw_flatbottom_triangle(Triangle(*v0, *v1, vi), target);
                        self.draw_flattop_triangle(Triangle(*v1, vi, *v2), target);
                    }
                }
            },
        }
    }

    /// Renders a flat top triangle onto the screen.
    /// TODO: refactor this and `draw_flatbottom_triangle` functions.
    fn draw_flattop_triangle<B: BitmapOutput>(&self, triangle: Triangle<Vertex>, target: &mut B) {
        let Triangle(v0, v1, v2) = triangle;

        // Parameter validation.
        assert!(v0.position.y == v1.position.y);
        assert!(v0.position.y < v2.position.y);
        assert!(v0.position.x < v1.position.x);

        // Calculate line slopes.
        let m1 = (v2.position.x - v0.position.x) / (v2.position.y - v0.position.y);
        let m2 = (v2.position.x - v1.position.x) / (v2.position.y - v1.position.y);

        // Iterate over each horizontal line.
        let y_start = f32::ceil(v0.position.y - 0.5);
        let y_end = f32::ceil(v2.position.y - 0.5);
        let mut y = y_start;
        
        while y < y_end {
            // Calculate the beginning and end of this line.
            let lx1 = m1 * (y + 0.5 - v0.position.y) + v0.position.x;
            let lx2 = m2 * (y + 0.5 - v0.position.y) + v1.position.x;
            let lc1 = mix(v0.color, v2.color, (y + 0.5 - y_start) / (y_end - y_start));
            let lc2 = mix(v1.color, v2.color, (y + 0.5 - y_start) / (y_end - y_start));

            // Calculate the pixel indices.
            let x_start = f32::ceil(lx1 - 0.5);
            let x_end = f32::ceil(lx2 - 0.5);
            let mut x = x_start;

            while x < x_end {
                let c = mix(lc1, lc2, (x + 0.5 - x_start) / (x_end - x_start));
                target.put_pixel((x as u32, y as u32), c);
                x += 1.0;
            }

            // Go to the next line.
            y += 1.0;
        }
    }

    /// Renders a flat bottom triangle onto the screen.
    /// TODO: refactor this and `draw_flattop_triangle` functions.
    fn draw_flatbottom_triangle<B: BitmapOutput>(&self, triangle: Triangle<Vertex>, target: &mut B) {
        let Triangle(v0, v1, v2) = triangle;

        // Parameter validation.
        assert!(v0.position.y < v1.position.y);
        assert!(v1.position.y == v2.position.y);
        assert!(v1.position.x < v2.position.x);

        // Calculate line slopes.
        let m1 = (v1.position.x - v0.position.x) / (v1.position.y - v0.position.y);
        let m2 = (v2.position.x - v0.position.x) / (v2.position.y - v0.position.y);

        // Iterate over each horizontal line.
        let y_start = f32::ceil(v0.position.y - 0.5);
        let y_end = f32::ceil(v2.position.y - 0.5);
        let mut y = y_start;

        while y < y_end {
            // Calculate the beginning and end of this line.
            let lx1 = m1 * (y + 0.5 - v0.position.y) + v0.position.x;
            let lx2 = m2 * (y + 0.5 - v0.position.y) + v0.position.x;
            let lc1 = mix(v0.color, v1.color, (y + 0.5 - y_start) / (y_end - y_start));
            let lc2 = mix(v0.color, v2.color, (y + 0.5 - y_start) / (y_end - y_start));

            // Calculate the pixel indices.
            let x_start = f32::ceil(lx1 - 0.5);
            let x_end = f32::ceil(lx2 - 0.5);
            let mut x = x_start;

            while x < x_end {
                let c = mix(lc1, lc2, (x + 0.5 - x_start) / (x_end - x_start));
                target.put_pixel((x as u32, y as u32), c);
                x += 1.0;
            }

            // Go to the next line.
            y += 1.0;
        }
    }
}
