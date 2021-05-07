use cgmath::Vector4;

// Re-export the Color struct.
pub use sdl2::pixels::Color;

/// Converts `sdl2::pixels::Color` into a `cgmath::Vector4<f32>`
#[inline]
pub fn color_to_vector4(color: &Color) -> Vector4<f32> {
    Vector4::new(
        (color.r as f32) / 255.0,
        (color.g as f32) / 255.0,
        (color.b as f32) / 255.0,
        (color.a as f32) / 255.0,
    )
}
/// Converts `cgmath::Vector4<f32>` into a `sdl2::pixels::Color`
#[inline]
pub fn vector4_to_color(vector: &Vector4<f32>) -> Color {
    Color::RGBA(
        clamp(0.0, vector.x * 255.0, 255.0) as u8,
        clamp(0.0, vector.y * 255.0, 255.0) as u8,
        clamp(0.0, vector.z * 255.0, 255.0) as u8,
        clamp(0.0, vector.w * 255.0, 255.0) as u8,
    )
}

// /// Linearly interpolates two colors.
// pub fn mix(color1: Color, color2: Color, amount: f32) -> Color {
//     let (r1, g1, b1, a1) = color1.rgba();
//     let (r2, g2, b2, a2) = color2.rgba();
//     let amount = amount.max(0.0).min(1.0);
//     Color::RGBA(
//         lerp(r1, r2, amount),
//         lerp(g1, g2, amount),
//         lerp(b1, b2, amount),
//         lerp(a1, a2, amount),
//     )
// }

// /// Linear interpolation.
// #[inline]
// fn lerp(value1: u8, value2: u8, amount: f32) -> u8 {
//     (((value2 as f32) * amount) + ((value1 as f32) * (1.0 - amount))) as u8
// }

/// Float value clamping.
#[inline]
fn clamp(min: f32, value: f32, max: f32) -> f32 {
    f32::min(f32::max(value, min), max)
}