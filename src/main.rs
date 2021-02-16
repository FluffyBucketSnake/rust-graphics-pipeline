extern crate sdl2;

mod framework;
mod vec;

use framework::Framework;

fn main() {
    let framework = Framework::init();

    framework.run(|| {
    });
}
