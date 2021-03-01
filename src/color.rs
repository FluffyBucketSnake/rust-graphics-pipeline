pub use sdl2::pixels::Color;

pub fn mix(color1: Color, color2: Color, amount: f32) -> Color {
    let (r1, g1, b1, a1) = color1.rgba();
    let (r2, g2, b2, a2) = color2.rgba();
    let amount = amount.max(0.0).min(1.0);
    Color::RGBA(
            mix_channel(r1, r2, amount),
            mix_channel(g1, g2, amount),
            mix_channel(b1, b2, amount),
            mix_channel(a1, a2, amount),
        )
}

fn mix_channel(value1: u8, value2: u8, amount: f32) -> u8 {
    (((value2 as f32) * amount) + ((value1 as f32) * (1.0 - amount))) as u8
}
