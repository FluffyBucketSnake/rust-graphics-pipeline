use cgmath::prelude::*;
use cgmath::{Matrix4, Point3, Vector3};
use std::mem::swap;
use crate::color::mix;
use crate::vertex::Vertex;
use super::clipping::clip_line;
use super::primitives::{Line, Triangle};
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
}

impl Rasterizer {
    pub fn new() -> Self {
        Rasterizer {
            fill_mode: FillMode::Solid,
        }
    }
}

impl Rasterizer {
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

impl<B> GPU<Line<Vertex>, B> for Rasterizer 
where B: BitmapOutput {
    fn draw(&self, line: Line<Vertex>, target: &mut B) {
        let Line(mut start, mut end) = line;
        
        // Clip lines out side the window.
        let line_xyz = (start.position, end.position);
        if let Some(line_xyz) = clip_line(line_xyz) {
            start.position.x = line_xyz.0.x;
            start.position.y = line_xyz.0.y;
            start.position.z = line_xyz.0.z;
            end.position.x = line_xyz.1.x;
            end.position.y = line_xyz.1.y;
            end.position.z = line_xyz.1.z;
        }
        else {
            // Line has been completely clipped out.
            return;
        }
        
        // Screen mapping phase.
        let (sw, sh) = target.size();
        let (sw, sh) = (sw as f32, sh as f32);
        let transform = Matrix4::from_nonuniform_scale(sw / 2.0, -sh / 2.0, 1.0)
                                 * Matrix4::from_translation(Vector3::new(1.0, -1.0, 0.0));
        start.position = transform.transform_point(start.position);
        end.position = transform.transform_point(end.position);
        
        // Line traversal.
        // Based on DDA algorithm.
        let delta = end.position - start.position;
        let step = f32::max(delta.x.abs(), delta.y.abs());  // Largest axis difference.
        let Vector3 { x: xi, y: yi, z: _ } = delta / step;    // Increment for each axis.
        let Point3 { mut x, mut y, z: _ } = start.position;
        let mut i: f32 = 0.0;
        while i < step {
            target.put_pixel((x as u32, y as u32), mix(start.color, end.color, i / f32::max(step - 1.0, 1.0)));
            x += xi;
            y += yi;
            i += 1.0;
        }
    }
}

impl<B: BitmapOutput> GPU<Triangle<Vertex>, B> for Rasterizer {
    fn draw(&self, mut triangle: Triangle<Vertex>, target: &mut B) {
        match self.fill_mode {
            FillMode::Wireframe => {
                self.draw(Line(triangle.0, triangle.1), target);
                self.draw(Line(triangle.1, triangle.2), target);
                self.draw(Line(triangle.2, triangle.0), target);
            },
            FillMode::Solid => {
                // Use references for easy swapping.
                let (mut v0, mut v1, mut v2) = (&mut triangle.0, &mut triangle.1, &mut triangle.2);

                // Screen mapping phase.
                // TODO: Use viewport instead of screen.
                let (sw, sh) = target.size();
                let (sw, sh) = (sw as f32, sh as f32);
                let transform = Matrix4::from_nonuniform_scale(sw / 2.0, -sh / 2.0, 1.0)
                                         * Matrix4::from_translation(Vector3::new(1.0, -1.0, 0.0));
                v0.position = transform.transform_point(v0.position);
                v1.position = transform.transform_point(v1.position);
                v2.position = transform.transform_point(v2.position);

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
}
