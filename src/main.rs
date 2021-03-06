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
use crate::math::{Quaternion, Matrix};
use crate::graphics::*;

fn main() {
    let framework = Framework::init();
    let mut output = framework.create_video_output();
    let mut pipeline = Pipeline::new();
    pipeline.set_front_face(crate::graphics::WindingOrder::CounterClockwise);

    // Build model.
    let model = models::build_triangle_cube();
    let world = Matrix::translate(0.0, 0.0, 2.0);

    // Keep track of rotation.
    let mut theta1 = 0.0f32;
    let mut theta2 = 0.0f32;

    // Run simulation.
    framework.run(|| { 
        let output = &mut output;

        output.clear(sdl2::pixels::Color::BLACK);

        let rotation: Matrix = Quaternion::rotation(crate::math::Vec3f::positive_x(), theta1).into();
        pipeline.set_worldviewproj(world * rotation);

        pipeline.draw((&model.0[..], &model.1[..]), output);
        
        output.present();

        theta1 += 0.002 * std::f32::consts::PI;
        theta2 += 0.003 * std::f32::consts::PI;
    });
}
