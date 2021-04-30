use cgmath::prelude::*;
use cgmath::{Matrix4, Vector3, Vector4};
use std::mem::swap;
use crate::color::mix;
use crate::vertex::Vertex;
use super::clipping::clip_line;
use super::primitives::{Line, Triangle, WindingOrder};
use super::{BitmapOutput, GPU};

pub enum FillMode {
    Solid,
    Wireframe,
}

/// The graphics processing component responsible for rasterizing primitives into the screen.
///
/// The rasterizer first coverts the normalized device coordinates into screen coordinates,
/// then setup the primitives, traverses the screen, rendering each pixel.
pub struct Rasterizer {
    pub fill_mode: FillMode,
    pub front_face: WindingOrder,
}

impl Rasterizer {
    pub fn new() -> Self {
        Rasterizer {
            fill_mode: FillMode::Solid,
            front_face: WindingOrder::Both,
        }
    }
}

impl Rasterizer {
    pub fn draw_line<B: BitmapOutput>(&self, line: Line<Vertex>, target: &mut B) {
        // Line traversal.
        // Based on DDA algorithm.
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

    pub fn draw_triangle<B: BitmapOutput>(&self, mut triangle: Triangle<Vertex>, target: &mut B) {
        match self.fill_mode {
            FillMode::Wireframe => {
                self.draw_line(Line(triangle.0, triangle.1), target);
                self.draw_line(Line(triangle.1, triangle.2), target);
                self.draw_line(Line(triangle.2, triangle.0), target);
            },
            FillMode::Solid => {

                // Screen mapping phase.
                // TODO: Use viewport instead of screen.
                let (sw, sh) = target.size();
                let (sw, sh) = (sw as f32, sh as f32);
                let transform = Matrix4::from_nonuniform_scale(sw / 2.0, -sh / 2.0, 1.0)
                                         * Matrix4::from_translation(Vector3::new(1.0, -1.0, 0.0));
                triangle.0.position = transform * triangle.0.position;
                triangle.1.position = transform * triangle.1.position;
                triangle.2.position = transform * triangle.2.position;

                // Front-face culling.
                match triangle.order() {
                    WindingOrder::Clockwise => {
                        if self.front_face == WindingOrder::CounterClockwise { return }
                    },
                    WindingOrder::CounterClockwise => {
                        if self.front_face == WindingOrder::Clockwise { return }
                    },
                    WindingOrder::Both => {},
                }

                // Use references for easy swapping.
                let (mut v0, mut v1, mut v2) = (&mut triangle.0, &mut triangle.1, &mut triangle.2);

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
            // Caculate the beginning and end of this line.
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
            // Caculate the beginning and end of this line.
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
