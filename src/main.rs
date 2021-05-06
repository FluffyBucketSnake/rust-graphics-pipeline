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
use framework::Framework;
use models::Cube;
use scenes::{BasicScene, Model};

fn main() {
    let framework = Framework::init();

    let mut app = App::init(&framework);

    let cube = Cube::new(1.0);
    let scene = BasicScene::new(&framework, Model::IndexedLineList((&cube).into()));
    app.add_scene(scene);

    app.run();
}
