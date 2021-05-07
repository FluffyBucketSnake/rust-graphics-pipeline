use sdl2::keyboard::KeyboardState;
use sdl2::mouse::MouseState;

/// View scenes interface.
///
/// This traits offers methods for defining the behavior of the scene.
pub trait Scene {
    fn draw(&mut self);

    // TODO: Create Keyboard and Mouse abstractions.
    fn handle_input(&mut self, keyboard: KeyboardState, mouse: MouseState);
}