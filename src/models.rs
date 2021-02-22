use crate::math::{Vec2f, Vec3f};
use crate::vertex::Vertex;
use sdl2::pixels::Color;

pub fn build_line_circle(dtheta: f32, colors: &(Color, Color)) -> Vec<(Vertex, Vertex)> {
    let mut result = Vec::new();
    let mut theta = 0.0f32;
    while theta < 2.0 * std::f32::consts::PI {
        result.push((Vertex::new(Vec3f::zero(), colors.0),
                     Vertex::new(Vec3f::from_vec2(Vec2f::from_direction(theta), 1.0), colors.1)));
        theta += dtheta;
    }
    result
}

pub fn build_line_cube() -> Vec<(Vertex, Vertex)> {
    let p1 = Vertex::new(Vec3f::from_uniform(-0.5), Color::WHITE);
    let p2 = Vertex::new(Vec3f::new(-0.5, -0.5, 0.5), Color::WHITE);
    let p3 = Vertex::new(Vec3f::new(-0.5, 0.5, -0.5), Color::WHITE);
    let p4 = Vertex::new(Vec3f::new(-0.5, 0.5, 0.5), Color::WHITE);
    let p5 = Vertex::new(Vec3f::new(0.5, -0.5, -0.5), Color::WHITE);
    let p6 = Vertex::new(Vec3f::new(0.5, -0.5, 0.5), Color::WHITE);
    let p7 = Vertex::new(Vec3f::new(0.5, 0.5, -0.5), Color::WHITE);
    let p8 = Vertex::new(Vec3f::from_uniform(0.5), Color::WHITE);

    let mut result = Vec::new();
    result.push((p1,p2));
    result.push((p2,p4));
    result.push((p4,p3));
    result.push((p3,p1));
    result.push((p5,p6));
    result.push((p6,p8));
    result.push((p8,p7));
    result.push((p7,p5));
    result.push((p1,p5));
    result.push((p2,p6));
    result.push((p3,p7));
    result.push((p4,p8));
    result
}
