use crate::color::mix;
use crate::framework::BitmapOutput;
use crate::math::Vec3f;
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
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            rasterizer: Rasterizer::new(),
        }
    }

    pub fn draw_primitives(&self, target: &mut BitmapOutput, primitives: &[(Vertex, Vertex)]) {
        for primitive in primitives {
            let (start, end) = primitive;
            self.rasterizer.draw_line(target, start, end);
        }
    }
}
