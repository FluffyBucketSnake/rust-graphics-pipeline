extern crate sdl2;

mod color;
mod framework;
mod pipeline;
mod math;
mod vertex;

use std::ops::Rem;

use crate::framework::Framework;
use crate::pipeline::*;
use crate::math::{Matrix, Vec2f, Vec3f};
use crate::vertex::Vertex;
use sdl2::pixels::Color;

fn build_line_circle(origin: Vec3f, length: f32, dtheta: f32, colors: &(Color, Color)) -> Vec<(Vertex, Vertex)> {
    let mut result = Vec::new();
    let mut theta = 0.0f32;
    while theta < 2.0 * std::f32::consts::PI {
        result.push((Vertex::new(origin, colors.0),
                     Vertex::new(origin + (length * Vec3f::from_vec2(Vec2f::from_direction(theta), 1.0)), colors.1)));
        theta += dtheta;
    }
    result
}

fn build_line_cube() -> Vec<(Vertex, Vertex)> {
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

fn main() {
    let framework = Framework::init();
    let mut output = framework.create_video_output();
    let pipeline = Pipeline::new((640.0, 640.0));

    // Rotate the model's x-axis by theta.
    let mut theta = 0.0;

    // Build model.
    let model = build_line_cube();
    let translation = Matrix::translate(0.0, 0.0, 2.0);

    // Run simulation.
    framework.run(|| { 
        let output = &mut output;

        let world = Matrix::rotate_x(theta) * translation;
        let model_t: Vec<_> = model.iter().map( |line| {
            let (mut start, mut end) = line.clone();
            start.position = world * start.position;
            end.position = world * end.position;

            (start, end)
        }).collect();

        output.clear(sdl2::pixels::Color::BLACK);

        pipeline.draw_primitives(output, &model_t);

        theta += 0.01;
        theta = theta.rem(std::f32::consts::PI * 2.0);
        
        output.present();
    });
}
