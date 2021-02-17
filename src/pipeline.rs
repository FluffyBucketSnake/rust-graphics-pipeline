use std::cmp::max;

use crate::framework::BitmapOutput;
use crate::vec::Vec2f;
use sdl2::pixels::Color;

pub fn draw_line(target: &mut BitmapOutput, start: Vec2f, end: Vec2f) {
    // Based on DDA algorithm.
    let Vec2f { x: mut dx, y: mut dy } = end - start;
    let step = f32::max(dx, dy);
    dx /= step;
    dy /= step;
    let Vec2f { mut x, mut y } = start;
    let mut i: f32 = 1.0;
    while i <= step {
        target.put_pixel(x as i32, y as i32, Color::WHITE);
        x += dx;
        y += dy;
        i += 1.0;
    }
}
