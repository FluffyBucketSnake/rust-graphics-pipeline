use crate::graphics::Line;
use crate::math::{Vec2f, Vec3f};
use crate::vertex::Vertex;
use sdl2::pixels::Color;

pub fn build_line_circle(dtheta: f32, colors: &(Color, Color)) -> Vec<Line<Vertex>> {
    let mut result = Vec::new();
    let mut theta = 0.0f32;
    while theta < 2.0 * std::f32::consts::PI {
        result.push(Line(Vertex::new(Vec3f::zero(), colors.0),
                         Vertex::new(Vec3f::from_vec2(Vec2f::from_direction(theta), 1.0), colors.1)));
        theta += dtheta;
    }
    result
}

pub fn build_line_cube() -> (Vec<Vertex>, Vec<Line<usize>>) {
    let mut vertices = Vec::new();

    vertices.push(Vertex::new(Vec3f::from_uniform(-0.5), Color::WHITE));
    vertices.push(Vertex::new(Vec3f::new(-0.5, -0.5, 0.5), Color::WHITE));
    vertices.push(Vertex::new(Vec3f::new(-0.5, 0.5, -0.5), Color::WHITE));
    vertices.push(Vertex::new(Vec3f::new(-0.5, 0.5, 0.5), Color::WHITE));
    vertices.push(Vertex::new(Vec3f::new(0.5, -0.5, -0.5), Color::WHITE));
    vertices.push(Vertex::new(Vec3f::new(0.5, -0.5, 0.5), Color::WHITE));
    vertices.push(Vertex::new(Vec3f::new(0.5, 0.5, -0.5), Color::WHITE));
    vertices.push(Vertex::new(Vec3f::from_uniform(0.5), Color::WHITE));

    let mut indexes = Vec::new();
    indexes.push(Line(0, 1));
    indexes.push(Line(1, 3));
    indexes.push(Line(3, 2));
    indexes.push(Line(2, 0));
    indexes.push(Line(4, 5));
    indexes.push(Line(5, 7));
    indexes.push(Line(7, 6));
    indexes.push(Line(6, 4));
    indexes.push(Line(0, 4));
    indexes.push(Line(1, 5));
    indexes.push(Line(2, 6));
    indexes.push(Line(3, 7));

    (vertices, indexes)
}
