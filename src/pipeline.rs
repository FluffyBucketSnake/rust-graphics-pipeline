use crate::framework::BitmapOutput;
use crate::vec::Vec2f;
use sdl2::pixels::Color;

pub fn mix_channel(value1: u8, value2: u8, amount: f32) -> u8 {
    let amount = amount.max(0.0).min(1.0);
    (((value1 as f32) * amount) + ((value2 as f32) * (1.0 - amount))) as u8
}

pub fn mix(color1: Color, color2: Color, amount: f32) -> Color {
    let (r1, g1, b1, a1) = color1.rgba();
    let (r2, g2, b2, a2) = color2.rgba();
    Color::RGBA(
            mix_channel(r1, r2, amount),
            mix_channel(g1, g2, amount),
            mix_channel(b1, b2, amount),
            mix_channel(a1, a2, amount),
        )
}

pub fn draw_line(target: &mut BitmapOutput, start: (Vec2f, Color), end: (Vec2f, Color)) {
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
