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
use graphics::{BasicEffect, ColorTextureVertex, ColorVertex, Line, Triangle};
use models::{IndexedLineList, IndexedTriangleList, ModelBuilder};
use sdl2::image::LoadSurface;
use sdl2::pixels::Color;
use sdl2::surface::Surface;

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

fn texturize_model<'a, P: 'a, B>(
    builder: &'a mut B,
) -> impl ModelBuilder<P, ColorTextureVertex> + 'a
where
    B: ModelBuilder<P, Vector3<f32>>,
{
    let uvs = [
        (0.0f32, 0.0f32),
        (1.0f32, 0.0f32),
        (0.0f32, 1.0f32),
        (1.0f32, 1.0f32),
    ];

    let mut i = 0;
    builder.transform(move |v| {
        let uv = &uvs[i % 4];
        let color = Color::WHITE; // colors[i];
        i += 1;
        ColorTextureVertex::new(v.x, v.y, v.z, color, uv.0, uv.1)
    })
}

fn create_scenes_from_model<'a, 'f, 's: 'a, B>(app: &mut App<'f, 'a>, builder: &mut B, texture: Surface<'s>)
where
    B: ModelBuilder<Line<usize>, Vector3<f32>> + ModelBuilder<Triangle<usize>, Vector3<f32>>,
{
    let texturized = texturize_model(builder);
    let mut effect = BasicEffect::default();
    effect.texture = Some(texture);
    let textured_scene = BasicScene::with_effect(
        app.window(),
        Model::IndexedTriangleList(IndexedTriangleList::from_builder(texturized)),
        effect,
    );
    app.add_scene(textured_scene);

    let colorized_solid = colorize_primitive(builder);
    let solid_color_scene = BasicScene::new(
        app.window(),
        Model::IndexedTriangleList(IndexedTriangleList::from_builder(colorized_solid)),
    );
    app.add_scene(solid_color_scene);

    let colorized_line = colorize_primitive(builder);
    let wireframe_scene = BasicScene::new(
        app.window(),
        Model::IndexedLineList(IndexedLineList::from_builder(colorized_line)),
    );
    app.add_scene(wireframe_scene);
}

fn main() {
    let framework = Framework::init();

    let mut app = App::init(&framework);

    let texture1 = Surface::from_file("textures/temp.png").unwrap();
    let texture2 = Surface::from_file("textures/temp.png").unwrap();

    let mut cube = Cube::new(2.0);
    create_scenes_from_model(&mut app, &mut cube, texture1);

    let mut plane = Plane::new(Vector3::unit_x(), Vector3::unit_y());
    create_scenes_from_model(&mut app, &mut plane, texture2);

    app.run();
}
