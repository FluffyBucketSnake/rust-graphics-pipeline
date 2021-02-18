use crate::color::mix;
use crate::framework::BitmapOutput;
use crate::math::Vec2f;
use sdl2::pixels::Color;

pub struct Rasterizer {
}

impl Rasterizer {
    pub fn new() -> Self {
        Rasterizer {
        }
    }

    pub fn draw_line(&self, target: &mut BitmapOutput, start: (Vec2f, Color), end: (Vec2f, Color)) {
        // Based on DDA algorithm.
        let delta = end.0 - start.0;
        let step = f32::max(delta.x.abs(), delta.y.abs());
        let Vec2f { x: xi, y: yi } = delta / step;

        let Vec2f { mut x, mut y } = start.0;
        let mut i: f32 = 0.0;
        while i < step {
            target.put_pixel(x as i32, y as i32, mix(start.1, end.1, i / f32::max(step - 1.0, 1.0)));
            x += xi;
            y += yi;
            i += 1.0;
        }
    }
}

pub struct Pipeline {
    rasterizer: Rasterizer,
}
