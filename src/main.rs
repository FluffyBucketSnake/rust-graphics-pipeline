extern crate sdl2;

mod color;
mod framework;
mod pipeline;
mod math;

use crate::framework::Framework;
use crate::pipeline::*;
use crate::math::Vec2f;
use sdl2::pixels::Color;

fn main() {
    let framework = Framework::init();
    let mut output = framework.create_video_output();
    let rasterizer = Rasterizer::new();

    framework.run(|| { 
        let output = &mut output;

        output.clear(sdl2::pixels::Color::BLACK);

        // draw_line(&mut output, Vec2f::new(78.0, 480.0), Vec2f::new(44.0, 357.0));
        const LENGTH: f32 = 64.0;
        const DTHETA: f32 = std::f32::consts::FRAC_PI_8;
        let mut theta = 0.0f32;
        while theta < 2.0 * std::f32::consts::PI {
            let origin = Vec2f::new(400.0, 300.0);
            rasterizer.draw_line(output, (origin, Color::BLUE), (origin + (LENGTH * Vec2f::from_direction(theta)), Color::RED));
            theta += DTHETA;
        }
        
        output.present();
    });
}
