use cgmath::{perspective, point3, vec3, Matrix4, Point3, Rad, Vector3};
use sdl2::{
    event::Event,
    keyboard::{Keycode, Mod},
};

use crate::framework::{CanvasOutput, Framework};
use crate::graphics::{BitmapOutput, Pipeline};
use crate::models::{IndexedLineList, IndexedTriangleList, LineList, TriangleList};

use super::Scene;

#[allow(dead_code)]
pub enum Model {
    LineList(LineList),
    IndexedLineList(IndexedLineList),
    TriangleList(TriangleList),
    IndexedTriangleList(IndexedTriangleList),
}

/// A scene for showing a model under a basic pipeline.
pub struct BasicScene {
    output: CanvasOutput,
    pipeline: Pipeline,

    model: Model,

    eye: Point3<f32>,
    yaw: f32,
    pitch: f32,
}

impl BasicScene {
    pub fn new(framework: &Framework, model: Model) -> Self {
        Self {
            output: framework.create_video_output(),
            pipeline: Pipeline::new(),

            model,

            eye: point3(0.0, 0.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

impl Scene for BasicScene {
    fn draw(&mut self) {
        self.output.clear(sdl2::pixels::Color::BLACK);

        // Camera settings.
        let dir = Vector3::unit_z();
        let up = Vector3::unit_y();

        // Projection settings.
        let fov = Rad(std::f32::consts::FRAC_PI_2);
        let aspect = 1.0;

        // Calculate the World-View-Model matrix.
        let world = Matrix4::from_translation(vec3(0.0, 0.0, 4.0))
            * Matrix4::from_angle_x(Rad(self.pitch))
            * Matrix4::from_angle_y(Rad(self.yaw))
            * Matrix4::from_scale(0.5);
        let view = Matrix4::look_to_rh(self.eye, dir, up);
        let projection = perspective(fov, aspect, 0.1, 100.0);
        self.pipeline.worldviewproj = projection * view * world;

        // Draw onto the screen.
        match self.model {
            Model::LineList(ref model) => {
                self.pipeline.draw_lines(
                    &model.primitives[..],
                    &mut self.output,
                );
            }
            Model::IndexedLineList(ref model) => {
                self.pipeline.draw_indexed_lines(
                    &model.vertices[..],
                    &model.primitives[..],
                    &mut self.output,
                );
            }
            Model::TriangleList(ref model) => {

                self.pipeline.draw_triangles(
                    &model.primitives[..],
                    &mut self.output,
                );
            }
            Model::IndexedTriangleList(ref model) => {
                self.pipeline.draw_indexed_triangles(
                    &model.vertices[..],
                    &model.primitives[..],
                    &mut self.output,
                );
            }
        };
        self.output.present();
    }

    fn handle_event(&mut self, event: Event) {
        const CAMSPEED: f32 = 1.0 / 60.0;
        const ROTSPEED: f32 = (2.0 * std::f32::consts::PI) / (2.0 * 60.0);
        match event {
            // Camera controls.
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => {
                self.eye.z += CAMSPEED;
            }
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => {
                self.eye.x -= CAMSPEED;
            }
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => {
                self.eye.z -= CAMSPEED;
            }
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => {
                self.eye.x += CAMSPEED;
            }
            Event::KeyDown {
                keymod: Mod::LSHIFTMOD,
                ..
            } => {
                self.eye.y -= CAMSPEED;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => {
                self.eye.y += CAMSPEED;
            }
            // Rotation controls.
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => {
                self.pitch += ROTSPEED;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => {
                self.yaw += ROTSPEED;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => {
                self.pitch -= ROTSPEED;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => {
                self.yaw -= ROTSPEED;
            }
            _ => {}
        }
    }
}
