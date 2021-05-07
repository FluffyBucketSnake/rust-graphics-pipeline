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

use crate::framework::Framework;
use crate::scenes::LineCubeScene;

fn main() {
    let framework = Framework::init();

    let mut app = App::init(&framework);
    
    let scene = LineCubeScene::new(&framework);
    app.add_scene(scene);

    app.run();
}
