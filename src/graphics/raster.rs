use cgmath::prelude::*;
use cgmath::{Matrix4, Point3, Vector3};
use crate::color::mix;
use crate::vertex::Vertex;
use super::clipping::clip_line;
use super::primitives::Line;
use super::{BitmapOutput, GPU};

/// The graphics processing component responsible for rasterizing primitives into the screen.
///
/// The rasterizer first coverts the normalized device coordinates into screen coordinates,
/// then setup the primitives, traverses the screen, rendering each pixel.
pub struct Rasterizer {
}

impl Rasterizer {
    pub fn new() -> Self {
        Rasterizer {
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
