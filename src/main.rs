extern crate bitflags;
extern crate cgmath;
extern crate sdl2;

mod color;
mod framework;
mod graphics;
mod models;
mod pipeline;
mod vertex;

#[cfg(test)]
mod tests;

use cgmath::{Quaternion, Matrix4, perspective, point3, Rad, vec3, Vector3};
use crate::framework::Framework;
use crate::graphics::*;

fn main() {
    let framework = Framework::init();
    let mut output = framework.create_video_output();
    let mut pipeline = Pipeline::new();
    pipeline.set_front_face(crate::graphics::WindingOrder::CounterClockwise);

    // Build model.
    let model = models::build_triangle_cube();
    let worldviewproj = perspective(Rad(std::f32::consts::FRAC_PI_2), 1.0, 0.1, 100.0)
                        * Matrix4::look_to_rh(point3(0.0, 0.0, 0.0), Vector3::unit_z(), Vector3::unit_y())
                        * Matrix4::from_translation(vec3(0.0, 0.0, 2.0)); 

    // Keep track of rotation.
    let mut theta1 = 0.0f32;
    let mut theta2 = 0.0f32;

    // Run simulation.
    framework.run(|| { 
        let output = &mut output;

        output.clear(sdl2::pixels::Color::BLACK);

        let q = Quaternion::from_sv(f32::cos(theta1), f32::sin(theta1) * Vector3::unit_x());
        let r = Quaternion::from_sv(f32::cos(theta1), f32::sin(theta1) * Vector3::unit_y());
        let rotation: Matrix4<f32> = q.slerp(r, f32::cos(theta2)).into();
        pipeline.set_worldviewproj(worldviewproj * rotation);

        pipeline.draw((&model.0[..], &model.1[..]), output);
        
        output.present();

        theta1 += 0.002 * std::f32::consts::PI;
        theta2 += 0.003 * std::f32::consts::PI;
    });
}
