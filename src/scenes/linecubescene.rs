use cgmath::{perspective, point3, vec3, Matrix4, Quaternion, Rad, Vector3};

use crate::framework::{CanvasOutput, Framework};
use crate::graphics::{BitmapOutput, Line, Pipeline, Vertex};

use super::Scene;

pub struct LineCubeScene {
    output: CanvasOutput,
    pipeline: Pipeline,

    viewproj: Matrix4<f32>,
    model: (Vec<Vertex>, Vec<Line<usize>>),

    theta1: f32,
    theta2: f32,
}

impl LineCubeScene {
    pub fn new(framework: &Framework) -> Self {
        // Camera settings.
        let eye = point3(0.0, 0.0, 0.0);
        let dir = Vector3::unit_z();
        let up = Vector3::unit_y();

        // Projection settings.
        let fov = Rad(std::f32::consts::FRAC_PI_2);
        let aspect = 1.0;

        Self {
            output: framework.create_video_output(),
            pipeline: Pipeline::new(),

            viewproj: perspective(fov, aspect, 0.1, 100.0) * Matrix4::look_to_rh(eye, dir, up),
            model: crate::models::build_line_cube(),

            theta1: 0.0,
            theta2: 0.0,
        }
    }
}

impl Scene for LineCubeScene {
    fn draw(&mut self) {
        self.output.clear(sdl2::pixels::Color::BLACK);

        let q = Quaternion::from_sv(
            f32::cos(self.theta1),
            f32::sin(self.theta1) * Vector3::unit_x(),
        );
        let r = Quaternion::from_sv(
            f32::cos(self.theta1),
            f32::sin(self.theta1) * Vector3::unit_y(),
        );
        let rotation: Matrix4<f32> = q.slerp(r, f32::cos(self.theta2)).into();

        let translation =
            Matrix4::from_translation(vec3(0.0, 0.0, 4.0 - 8.0 * f32::sin(self.theta1)));

        let scale = Matrix4::from_scale(2.0);

        self.pipeline.worldviewproj = self.viewproj * translation * rotation * scale;

        self.pipeline
            .draw_indexed_lines(&self.model.0[..], &self.model.1[..], &mut self.output);

        self.output.present();

        self.theta1 += 0.0002 * std::f32::consts::PI;
        self.theta2 += 0.003 * std::f32::consts::PI;
    }

    fn update(&mut self) {
    }
}
