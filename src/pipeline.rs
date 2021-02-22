use crate::color::mix;
use crate::framework::BitmapOutput;
use crate::math::{Matrix, Vec3f};
use crate::vertex::Vertex;

pub struct Rasterizer {
}

impl Rasterizer {
    pub fn new() -> Self {
        Rasterizer {
        }
    }

    pub fn draw_line(&self, target: &mut BitmapOutput, start: &Vertex, end: &Vertex) {
        // Based on DDA algorithm.
        let delta = end.position - start.position;
        let step = f32::max(delta.x.abs(), delta.y.abs());
        let Vec3f { x: xi, y: yi, z: _ } = delta / step;

        let Vec3f { mut x, mut y, z: _ } = start.position;
        let mut i: f32 = 0.0;
        while i < step {
            target.put_pixel(x as i32, y as i32, mix(start.color, end.color, i / f32::max(step - 1.0, 1.0)));
            x += xi;
            y += yi;
            i += 1.0;
        }
    }
}

pub struct Pipeline {
    rasterizer: Rasterizer,
    screen_size: (f32, f32),
}

impl Pipeline {
    pub fn new(screen_size: (f32, f32)) -> Self {
        Self {
            rasterizer: Rasterizer::new(),
            screen_size,
        }
    }

    pub fn draw_primitives(&self, target: &mut BitmapOutput, primitives: &[(Vertex, Vertex)]) {
        for primitive in primitives {
            let (mut start, mut end) = primitive;
            // Rasterization Begin
            // Convert coordinates to clip space.
            start.position /= start.position.z;
            end.position /= end.position.z;
            // Convert coordinates to device coordinates.
            let window_transform = Matrix::scale(self.screen_size.0 / 2.0, self.screen_size.1 / 2.0, 1.0)
                                    * Matrix::translate(1.0, 1.0, 0.0);
            start.position = window_transform * start.position;
            end.position = window_transform * end.position;
            // Render primitive.
            self.rasterizer.draw_line(target, &start, &end);
            // Rasterization End
        }
    }
}
