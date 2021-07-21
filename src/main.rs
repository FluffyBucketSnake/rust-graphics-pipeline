extern crate bitflags;
extern crate cgmath;
extern crate sdl2;

mod app;
mod framework;
mod graphics;
mod math;
mod models;
mod scenes;

#[cfg(test)]
mod tests;

use app::App;
use cgmath::Vector3;
use graphics::{ColorVertex, Line, Triangle};
use models::{IndexedLineList, IndexedTriangleList, ModelBuilder};
use sdl2::pixels::Color;

use crate::framework::Framework;
use crate::models::{Cube, Plane};
use crate::scenes::{BasicScene, Model};

fn colorize_primitive<'a, P: 'a, B>(builder: &'a mut B) -> impl ModelBuilder<P, ColorVertex> + 'a
where
    B: ModelBuilder<P, Vector3<f32>>,
{
    let colors = [
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::CYAN,
        Color::MAGENTA,
        Color::YELLOW,
        Color::WHITE,
        Color::GRAY,
    ];

    let mut i = 0;
    builder.transform(move |v| {
        i += 1;
        ColorVertex::new(v.x, v.y, v.z, colors[i - 1])
    })
}

fn create_scenes_from_model<B>(app: &mut App, builder: &mut B)
where
    B: ModelBuilder<Line<usize>, Vector3<f32>> + ModelBuilder<Triangle<usize>, Vector3<f32>>,
{
    let colorized_line = colorize_primitive(builder);
    let wireframe_scene = BasicScene::new(
        app.window(),
        Model::IndexedLineList(IndexedLineList::from_builder(colorized_line)),
    );
    app.add_scene(wireframe_scene);

    let colorized_solid = colorize_primitive(builder);
    let solid_color_scene = BasicScene::new(
        app.window(),
        Model::IndexedTriangleList(IndexedTriangleList::from_builder(colorized_solid)),
    );
    app.add_scene(solid_color_scene);
}

fn main() {
    let framework = Framework::init();

    let mut app = App::init(&framework);

    let mut cube = Cube::new(2.0);
    create_scenes_from_model(&mut app, &mut cube);

    let mut plane = Plane::new(Vector3::unit_x(), Vector3::unit_y());
    create_scenes_from_model(&mut app, &mut plane);

    app.run();
}
