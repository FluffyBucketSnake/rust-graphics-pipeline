extern crate sdl2;
mod framework;
mod pipeline;
mod vec;

fn main() {
    let mut framework = framework::Framework::init();

    framework.run()
}
