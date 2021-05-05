extern crate bitflags;
extern crate cgmath;
extern crate sdl2;

mod framework;
mod graphics;
mod math;
mod models;
mod scenes;

#[cfg(test)]
mod tests;

use crate::framework::Framework;
use crate::scenes::LineCubeScene;

fn main() {
    let framework = Framework::init();

    let scene = LineCubeScene::new(&framework);

    framework.run(scene);
}
