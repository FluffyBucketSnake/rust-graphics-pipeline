use crate::color::mix;
use crate::math::{Matrix, Vec2f, Vec3f};
use crate::vertex::Vertex;
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

impl<B> GPU<(Vertex, Vertex), B> for Rasterizer 
where B: BitmapOutput {
    fn draw(&self, line: (Vertex, Vertex), target: &mut B) {
        let (mut start, mut end) = line;
        
        // Screen mapping phase.
        let screen_size = target.size();
        let screen_size = Vec2f::new(screen_size.0 as f32, screen_size.1 as f32);
        let window_transform = Matrix::scale(screen_size.x / 2.0, -screen_size.y / 2.0, 1.0)
                                * Matrix::translate(1.0, -1.0, 0.0);
        start.position = window_transform * start.position;
        end.position = window_transform * end.position;

        // Line traversal.
        // Based on DDA algorithm.
        let delta = end.position - start.position;
        let step = f32::max(delta.x.abs(), delta.y.abs());  // Largest axis difference.
        let Vec3f { x: xi, y: yi, z: _ } = delta / step;    // Increment for each axis.
        let Vec3f { mut x, mut y, z: _ } = start.position;
        let mut i: f32 = 0.0;
        while i < step {
            target.put_pixel((x as u32, y as u32), mix(start.color, end.color, i / f32::max(step - 1.0, 1.0)));
            x += xi;
            y += yi;
            i += 1.0;
        }
    }
}
