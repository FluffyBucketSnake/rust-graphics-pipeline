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

use crate::framework::Framework;
use crate::models::{Cube, Plane};
use crate::scenes::{BasicScene, Model};

fn main() {
    let framework = Framework::init();

    let mut app = App::init(&framework);

    let cube = Cube::new(2.0);

    let scene0 = BasicScene::new(app.window(), Model::IndexedLineList((&cube).into()));
    app.add_scene(scene0);
    let scene1 = BasicScene::new(app.window(), Model::IndexedTriangleList((&cube).into()));
    app.add_scene(scene1);

    let plane = Plane::new(Vector3::unit_x(), Vector3::unit_y());

    let scene2 = BasicScene::new(app.window(), Model::IndexedLineList((&plane).into()));
    app.add_scene(scene2);
    let scene3 = BasicScene::new(app.window(), Model::IndexedTriangleList((&plane).into()));
    app.add_scene(scene3);

    app.run();
}
