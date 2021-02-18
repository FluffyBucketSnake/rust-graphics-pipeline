use crate::framework::BitmapOutput;
use crate::vec::Vec2f;
use sdl2::pixels::Color;

pub fn draw_line(target: &mut BitmapOutput, start: Vec2f, end: Vec2f, color: Color) {
    // Based on DDA algorithm.
    let delta = end - start;
    let step = f32::max(delta.x.abs(), delta.y.abs());
    let Vec2f { x: xi, y: yi } = delta / step;

    let Vec2f { mut x, mut y } = start;
    let mut i: f32 = 1.0;
    while i <= step {
        target.put_pixel(x as i32, y as i32, color);
        x += xi;
        y += yi;
        i += 1.0;
    }
}
