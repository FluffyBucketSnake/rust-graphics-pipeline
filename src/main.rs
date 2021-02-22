extern crate sdl2;

mod color;
mod framework;
mod graphics;
mod math;
mod models;
mod pipeline;
mod vertex;

use crate::framework::Framework;
use crate::math::Matrix;
use crate::graphics::*;

fn main() {
    let framework = Framework::init();
    let mut output = framework.create_video_output();
    let pipeline = Pipeline::new();

    // Rotate the model's x-axis by theta.
    let mut theta = 0.0;

    // Build model.
    let model = models::build_line_cube();
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

        pipeline.draw(&model_t.as_slice(), output);

        theta += 0.01;
        theta = theta % (2.0 * std::f32::consts::PI);
        
        output.present();
    });
}
