use cgmath::Vector4;

use std::{marker::PhantomData, mem::swap};

use super::clipping::{clip_line, clip_triangle};
use super::primitives::{Line, Primitive, Triangle, WindingOrder};
use super::{Effect, RenderTarget, Vertex};

#[allow(dead_code)]
pub enum FillMode {
    Solid,
    Wireframe,
}

/// A software implementation of a raster graphics processor pipeline.
///
/// It accepts a collection of primitives as input, while output a raster render of the scene
/// into the specified `BitmapOutput`.
pub struct Pipeline<V, E>
where
    E: Effect<V>,
    V: Vertex,
{
    pub fill_mode: FillMode,
    pub front_face: WindingOrder,
    pub effect: E,
    pub vertex: PhantomData<V>,
}

impl<V: Vertex, E: Effect<V>> Pipeline<V, E> {
    /// Constructs a new `Pipeline`.
    pub fn new(effect: E) -> Self {
        Self {
            fill_mode: FillMode::Solid,
            front_face: WindingOrder::CounterClockwise,
            effect,
            vertex: PhantomData,
        }
    }

    /// Draws multiple lines onto the render target.
    #[allow(dead_code)]
    pub fn draw_lines<B: RenderTarget>(&self, primitives: &[Line<V>], target: &mut B) {
        // Copy input.
        let primitives = primitives.to_vec();

        // Render each primitive
        for mut primitive in primitives {
            // Vertex stage.
            self.vertex_processor(&mut primitive.0);
            self.vertex_processor(&mut primitive.1);

            // Primitive stage.
            // Triangle has already been assembled.
            self.line_processor(primitive, target);
        }
    }

    /// Draws multiple lines onto the render target. Allows vertex indexing.
    #[allow(dead_code)]
    pub fn draw_indexed_lines<B: RenderTarget>(
        &self,
        vertices: &[V],
        primitives: &[Line<usize>],
        target: &mut B,
    ) {
        // Copy input data.
        let mut vertices = vertices.to_vec();
        let primitives = primitives.to_vec();

        // Vertex stage.
        for vertex in vertices.iter_mut() {
            self.vertex_processor(vertex);
        }

        for primitive in primitives {
            // Primitive stage.
            let line = Line(vertices[primitive.0], vertices[primitive.1]); // Primitive assembly
            self.line_processor(line, target);
        }
    }

    /// Draws multiple triangles onto the render target.
    #[allow(dead_code)]
    pub fn draw_triangles<B: RenderTarget>(&self, primitives: &[Triangle<V>], target: &mut B) {
        // Copy input buffer.
        let primitives = primitives.to_vec();

        for mut primitive in primitives {
            // Vertex stage.
            self.vertex_processor(&mut primitive.0);
            self.vertex_processor(&mut primitive.1);
            self.vertex_processor(&mut primitive.2);

            // Primitive stage.
            self.triangle_processor(primitive, target);
        }
    }

    /// Draws multiple triangles onto the render target. Allows vertex indexing.
    #[allow(dead_code)]
    pub fn draw_indexed_triangles<B: RenderTarget>(
        &self,
        vertices: &[V],
        primitives: &[Triangle<usize>],
        target: &mut B,
    ) {
        // Copy input data.
        let mut vertices = vertices.to_vec();
        let primitives = primitives.to_vec();

        // Vertex stage.
        for vertex in vertices.iter_mut() {
            self.vertex_processor(vertex);
        }

        for primitive in primitives {
            // Primitive stage.
            let triangle = Triangle(
                vertices[primitive.0],
                vertices[primitive.1],
                vertices[primitive.2],
            ); // Primitive assembly.
            self.triangle_processor(triangle, target);
        }
    }

    /// Executes the vertex processor onto the input vertex.
    fn vertex_processor(&self, vertex: &mut V) {
        *vertex = self.effect.vs(vertex);
    }

    /// Applies the primitive processing stage onto the input line.
    /// Since lines can't go through front face culling, this method only clips the line.
    fn line_processor<B: RenderTarget>(&self, mut line: Line<V>, target: &mut B) {
        // Clip lines outside the window.
        if let Some(cline) = clip_line(line) {
            line = cline;
        } else {
            return;
        }

        // Convert into screen space.
        // TODO: Use viewport instead of screen.
        let screen = target.size();
        let (width, height) = (screen.0 as f32, screen.1 as f32);
        line.0.to_screen_coords(width, height);
        line.1.to_screen_coords(width, height);

        self.render_line(line, target);
    }

    /// Executes the first steps of the primitive processing stage over the input triangle.
    ///
    /// First, the triangle is culled, then it's clipped against the view frustum, then each
    /// resulting triangle is sent for postprocessing.
    fn triangle_processor<B: RenderTarget>(&self, triangle: Triangle<V>, target: &mut B) {
        // Face culling.
        match (self.front_face, triangle.get_winding()) {
            (WindingOrder::CounterClockwise, WindingOrder::Clockwise) => {
                return;
            }
            (WindingOrder::Clockwise, WindingOrder::CounterClockwise) => {
                return;
            }
            _ => {}
        };

        // Front plane clipping.
        match clip_triangle(triangle) {
            super::clipping::ClippedTriangle::Empty => {}
            super::clipping::ClippedTriangle::One(t) => {
                self.triangle_postprocessor(t, target);
            }
            super::clipping::ClippedTriangle::Two(t1, t2) => {
                self.triangle_postprocessor(t1, target);
                self.triangle_postprocessor(t2, target);
            }
        }
    }

    /// Executes the post-clipping processing stage over the input triangle
    fn triangle_postprocessor<B: RenderTarget>(&self, mut triangle: Triangle<V>, target: &mut B) {
        // Convert into screen space.
        // TODO: Use viewport instead of screen.
        let screen = target.size();
        let (width, height) = (screen.0 as f32, screen.1 as f32);
        triangle.0.to_screen_coords(width, height);
        triangle.1.to_screen_coords(width, height);
        triangle.2.to_screen_coords(width, height);

        // Rasterization stage.
        self.render_triangle(triangle, target);
    }

    /// Renders the line onto the render target. Uses the DDA algorithm.
    fn render_line<B: RenderTarget>(&self, line: Line<V>, target: &mut B) {
        // Line traversal.
        let delta = line.1 - line.0;
        let delta_pos = delta.position();
        let step = f32::max(delta_pos.x.abs(), delta_pos.y.abs()); // Largest axis difference.
        let dv = delta / step; // Increment for each axis.
        let mut it = line.0;
        let mut i: f32 = 0.0;
        while i < step {
            let Vector4 { x, y, z: _, w: _ } = it.position();
            self.draw_vertex((x as u32, y as u32), &it, target);
            it += dv;
            i += 1.0;
        }
    }

    /// Renders the line onto the render target.
    fn render_triangle<B: RenderTarget>(&self, triangle: Triangle<V>, target: &mut B) {
        match self.fill_mode {
            FillMode::Wireframe => {
                self.render_line(Line(triangle.0, triangle.1), target);
                self.render_line(Line(triangle.1, triangle.2), target);
                self.render_line(Line(triangle.2, triangle.0), target);
            }
            FillMode::Solid => {
                // Use references for easy swapping.
                let (mut v0, mut v1, mut v2) = (&triangle.0, &triangle.1, &triangle.2);

                // Sort vertices by y-value. v0.y < v1.y < v2.y
                if v0.position().y > v1.position().y {
                    swap(&mut v0, &mut v1);
                }
                if v1.position().y > v2.position().y {
                    swap(&mut v1, &mut v2);
                }
                if v0.position().y > v1.position().y {
                    swap(&mut v0, &mut v1);
                }

                if v0.position().y == v1.position().y {
                    // Sort top vertices by x. v0.x < v1.x
                    if v0.position().x > v1.position().x {
                        swap(&mut v0, &mut v1)
                    }

                    // Natural flat top.
                    self.draw_flattop_triangle(Triangle(*v0, *v1, *v2), target);
                } else if v1.position().y == v2.position().y {
                    // Sort bottom vertices by x. v1.x < v2.x
                    if v1.position().x > v2.position().x {
                        swap(&mut v1, &mut v2)
                    }

                    // Natural bottom top.
                    self.draw_flatbottom_triangle(Triangle(*v0, *v1, *v2), target);
                } else {
                    let a = (v1.position() - v0.position()).y / (v2.position() - v0.position()).y;

                    // TODO: Create a dedicated interpolation function/trait.
                    let vi = V::interpolate(v0, v2, a);

                    if v1.position().x > vi.position().x {
                        // Major left
                        self.draw_flatbottom_triangle(Triangle(*v0, vi, *v1), target);
                        self.draw_flattop_triangle(Triangle(vi, *v1, *v2), target);
                    } else {
                        // Major right
                        self.draw_flatbottom_triangle(Triangle(*v0, *v1, vi), target);
                        self.draw_flattop_triangle(Triangle(*v1, vi, *v2), target);
                    }
                }
            }
        }
    }

    /// Renders a flat top triangle onto the screen.
    ///
    /// This functions is a decorator for the `draw_flat_triangle` call.
    fn draw_flattop_triangle<B: RenderTarget>(&self, triangle: Triangle<V>, target: &mut B) {
        let dit0 = triangle.2 - triangle.0;
        let dit1 = triangle.2 - triangle.1;
        let dy = dit0.position().y;

        let dv0 = dit0 / dy;
        let dv1 = dit1 / dy;

        self.draw_flat_triangle(triangle.0, triangle.1, dv0, dv1, dy, target);
    }

    /// Renders a flat bottom triangle onto the screen.
    ///
    /// This functions is a decorator for the `draw_flat_triangle` call.
    fn draw_flatbottom_triangle<B: RenderTarget>(&self, triangle: Triangle<V>, target: &mut B) {
        let dit0 = triangle.1 - triangle.0;
        let dit1 = triangle.2 - triangle.0;
        let dy = dit0.position().y;

        let dv0 = dit0 / dy;
        let dv1 = dit1 / dy;

        self.draw_flat_triangle(triangle.0, triangle.0, dv0, dv1, dy, target);
    }

    fn draw_flat_triangle<B: RenderTarget>(
        &self,
        it0: V,
        it1: V,
        dv0: V,
        dv1: V,
        height: f32,
        target: &mut B,
    ) {
        // Calculate start and end scanlines.
        let y_start = f32::max(f32::ceil(it0.position().y - 0.5), 0.0);
        let y_end = f32::min(
            f32::ceil(it0.position().y + height - 0.5),
            target.size().1 as f32,
        );

        // Left and right edge interpolants
        let mut it_edge0 = it0 + dv0 * (y_start + 0.5 - it0.position().y);
        let mut it_edge1 = it1 + dv1 * (y_start + 0.5 - it1.position().y);

        // Calculate each scanline.
        let mut y = y_start;
        while y < y_end {
            // Calculate the start and end pixel positions.
            let x_start = f32::max(f32::ceil(it_edge0.position().x - 0.5), 0.0);
            let x_end = f32::min(
                f32::ceil(it_edge1.position().x - 0.5),
                target.size().0 as f32,
            );

            // Scanline interpolant
            let mut it = it_edge0;

            // Calculate scanline derivant.
            let dx = it_edge1.position().x - it.position().x;
            let dv = (it_edge1 - it) / dx;

            // Prestep interpolant.
            it += dv * (x_start + 0.5 - it.position().x);

            // Draw each pixel.
            let mut x = x_start;
            while x < x_end {
                self.draw_vertex((x as u32, y as u32), &it, target);
                x += 1.0;
                it += dv;
            }

            // Increment y and interpolants.
            y += 1.0;
            it_edge0 += dv0;
            it_edge1 += dv1;
        }
    }

    #[inline]
    fn draw_vertex<B: RenderTarget>(&self, position: (u32, u32), vertex: &V, target: &mut B) {
        if target.test_and_set_depth(position, vertex.position().z) {
            let z_inv = (1.0f64 / vertex.position().z as f64) as f32;
            let vertex = vertex.clone() * z_inv;
            target.put_pixel(position, self.effect.ps(&vertex));
        }
    }
}
