// Re-export the Color struct.
pub use sdl2::pixels::Color;

/// Linearly interpolates two colors.
pub fn mix(color1: Color, color2: Color, amount: f32) -> Color {
    let (r1, g1, b1, a1) = color1.rgba();
    let (r2, g2, b2, a2) = color2.rgba();
    let amount = amount.max(0.0).min(1.0);
    Color::RGBA(
            lerp(r1, r2, amount),
            lerp(g1, g2, amount),
            lerp(b1, b2, amount),
            lerp(a1, a2, amount),
        )
}

/// Linear interpolation.
fn lerp(value1: u8, value2: u8, amount: f32) -> u8 {
    (((value2 as f32) * amount) + ((value1 as f32) * (1.0 - amount))) as u8
}