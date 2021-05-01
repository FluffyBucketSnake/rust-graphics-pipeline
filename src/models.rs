use crate::graphics::{Line, Triangle};
use crate::graphics::Vertex;
use sdl2::pixels::Color;

#[allow(dead_code)]
pub fn build_line_circle(dtheta: f32, colors: &(Color, Color)) -> Vec<Line<Vertex>> {
    let mut result = Vec::new();
    let mut theta = 0.0f32;
    while theta < 2.0 * std::f32::consts::PI {
        result.push(Line(Vertex::new(0.0, 0.0, 0.0, colors.0),
                         Vertex::new(f32::cos(theta), f32::sin(theta), 0.0, colors.1)));
        theta += dtheta;
    }
    result
}

#[allow(dead_code)]
pub fn build_line_cube() -> (Vec<Vertex>, Vec<Line<usize>>) {
    let mut vertices = Vec::new();

    // Front
    vertices.push(Vertex::new(-0.5, -0.5, -0.5, Color::RED));
    vertices.push(Vertex::new( 0.5, -0.5, -0.5, Color::BLUE));
    vertices.push(Vertex::new( 0.5,  0.5, -0.5, Color::GREEN));
    vertices.push(Vertex::new(-0.5,  0.5, -0.5, Color::CYAN));
    // Back
    vertices.push(Vertex::new(-0.5, -0.5,  0.5, Color::YELLOW));
    vertices.push(Vertex::new( 0.5, -0.5,  0.5, Color::MAGENTA));
    vertices.push(Vertex::new( 0.5,  0.5,  0.5, Color::GREY));
    vertices.push(Vertex::new(-0.5,  0.5,  0.5, Color::WHITE));

    let mut indexes = Vec::new();
    indexes.push(Line(0, 1));
    indexes.push(Line(0, 3));
    indexes.push(Line(0, 4));
    indexes.push(Line(1, 2));
    indexes.push(Line(1, 5));
    indexes.push(Line(2, 3));
    indexes.push(Line(2, 6));
    indexes.push(Line(3, 7));
    indexes.push(Line(4, 5));
    indexes.push(Line(4, 7));
    indexes.push(Line(5, 6));
    indexes.push(Line(6, 7));

    (vertices, indexes)
}

#[allow(dead_code)]
pub fn build_triangle_cube() -> (Vec<Vertex>, Vec<Triangle<usize>>) {
    let mut vertices = Vec::new();

    vertices.push(Vertex::new(-0.5, -0.5, -0.5, Color::RED));
    vertices.push(Vertex::new( 0.5, -0.5, -0.5, Color::GREEN));
    vertices.push(Vertex::new(-0.5,  0.5, -0.5, Color::BLUE));
    vertices.push(Vertex::new( 0.5,  0.5, -0.5, Color::YELLOW));
    vertices.push(Vertex::new(-0.5, -0.5,  0.5, Color::CYAN));
    vertices.push(Vertex::new( 0.5, -0.5,  0.5, Color::MAGENTA));
    vertices.push(Vertex::new(-0.5,  0.5,  0.5, Color::WHITE));
    vertices.push(Vertex::new( 0.5,  0.5,  0.5, Color::BLACK));

    let mut indexes = Vec::new();
    indexes.push(Triangle(0, 2, 1));
    indexes.push(Triangle(2, 3, 1));
    indexes.push(Triangle(1, 3, 5));
    indexes.push(Triangle(3, 7, 5));
    indexes.push(Triangle(2, 6, 3));
    indexes.push(Triangle(3, 6, 7));
    indexes.push(Triangle(4, 5, 7));
    indexes.push(Triangle(4, 7, 6));
    indexes.push(Triangle(0, 4, 2));
    indexes.push(Triangle(2, 4, 6));
    indexes.push(Triangle(0, 1, 4));
    indexes.push(Triangle(1, 5, 4));

    (vertices, indexes)
}
