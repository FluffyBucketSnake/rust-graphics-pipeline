use cgmath::{perspective, point3, vec3, Matrix4, Point3, Rad, Vector3};
use sdl2::keyboard::{KeyboardState, Scancode};
use sdl2::mouse::MouseState;

use crate::framework::{Window, WindowTarget};
use crate::graphics::{BasicEffect, Effect, Pipeline, RenderTarget, Vertex};
use crate::models::{IndexedLineList, IndexedTriangleList, LineList, TriangleList};

use super::Scene;

#[allow(dead_code)]
pub enum Model<V: Vertex> {
    LineList(LineList<V>),
    IndexedLineList(IndexedLineList<V>),
    TriangleList(TriangleList<V>),
    IndexedTriangleList(IndexedTriangleList<V>),
}

/// A scene for showing a model under a basic pipeline.
pub struct BasicScene<'a, V: Vertex>
where
    BasicEffect<'a>: Effect<V>,
{
    output: WindowTarget,
    pipeline: Pipeline<V, BasicEffect<'a>>,

    model: Model<V>,

    eye: Point3<f32>,
    yaw: f32,
    pitch: f32,
}

impl<'a, V: Vertex> BasicScene<'a, V>
where
    BasicEffect<'a>: Effect<V>,
{
    pub fn new(window: &Window, model: Model<V>) -> Self {
        Self::with_effect(window, model, BasicEffect::default())
    }

    pub fn with_effect(window: &Window, model: Model<V>, effect: BasicEffect<'a>) -> Self {
        Self {
            output: window.get_rendertarget(),
            pipeline: Pipeline::new(effect),

            model,

            eye: point3(0.0, 0.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

impl<'a, V: Vertex> Scene for BasicScene<'a, V>
where
    BasicEffect<'a>: Effect<V>,
{
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
        self.pipeline.effect.worldviewproj = projection * view * world;

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

    fn handle_input(&mut self, keyboard: KeyboardState, _: MouseState) {
        const CAMSPEED: f32 = 1.0 / 60.0;
        const ROTSPEED: f32 = (2.0 * std::f32::consts::PI) / (2.0 * 60.0);

        // Camera controls
        if keyboard.is_scancode_pressed(Scancode::W) {
            self.eye.z += CAMSPEED;
        }
        if keyboard.is_scancode_pressed(Scancode::A) {
            self.eye.x -= CAMSPEED;
        }
        if keyboard.is_scancode_pressed(Scancode::S) {
            self.eye.z -= CAMSPEED;
        }
        if keyboard.is_scancode_pressed(Scancode::D) {
            self.eye.x += CAMSPEED;
        }
        if keyboard.is_scancode_pressed(Scancode::LShift) {
            self.eye.y -= CAMSPEED;
        }
        if keyboard.is_scancode_pressed(Scancode::Space) {
            self.eye.y += CAMSPEED;
        }

        // Rotation controls
        if keyboard.is_scancode_pressed(Scancode::Up) {
            self.pitch += ROTSPEED;
        }
        if keyboard.is_scancode_pressed(Scancode::Left) {
            self.yaw += ROTSPEED;
        }
        if keyboard.is_scancode_pressed(Scancode::Down) {
            self.pitch -= ROTSPEED;
        }
        if keyboard.is_scancode_pressed(Scancode::Right) {
            self.yaw -= ROTSPEED;
        }
    }
}
