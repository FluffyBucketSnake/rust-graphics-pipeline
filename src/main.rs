extern crate sdl2;

mod color;
mod framework;
mod pipeline;
mod math;
mod vertex;

use crate::framework::Framework;
use crate::pipeline::*;
use crate::math::Vec2f;
use crate::vertex::Vertex;
use sdl2::pixels::Color;

fn build_line_circle(origin: Vec2f, length: f32, dtheta: f32, colors: &(Color, Color)) -> Vec<(Vertex, Vertex)> {
    let mut result = Vec::new();
    let mut theta = 0.0f32;
    while theta < 2.0 * std::f32::consts::PI {
        result.push((Vertex::new(origin, colors.0),
                     Vertex::new(origin + length * Vec2f::from_direction(theta), colors.1)));
        theta += dtheta;
    }
    result
}

fn main() {
    let framework = Framework::init();
    let mut output = framework.create_video_output();
    let pipeline = Pipeline::new();
    let model = build_line_circle(Vec2f::new(400.0, 300.0), 64.0, std::f32::consts::FRAC_PI_8, &(Color::BLUE, Color::RED));

    framework.run(|| { 
        let output = &mut output;

        output.clear(sdl2::pixels::Color::BLACK);

        pipeline.draw_primitives(output, &model);
        
        output.present();
    });
}
