extern crate sdl2;

mod color;
mod framework;
mod graphics;
mod math;
mod models;
mod pipeline;
mod vertex;

#[cfg(test)]
mod tests;

use crate::framework::Framework;
use crate::math::Matrix;
use crate::graphics::*;

fn main() {
    let framework = Framework::init();
    let mut output = framework.create_video_output();
    let mut pipeline = Pipeline::new();

    // Build model.
    let model = models::build_line_cube();
    let world = Matrix::translate(0.0, 0.0, 2.0);
    pipeline.set_worldviewproj(world);

    // Run simulation.
    framework.run(|| { 
        let output = &mut output;

        output.clear(sdl2::pixels::Color::BLACK);

        pipeline.draw((&model.0[..], &model.1[..]), output);
        
        output.present();
    });
}
