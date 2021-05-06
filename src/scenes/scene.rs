use sdl2::event::Event;

/// View scenes interface.
///
/// This traits offers methods for defining the behavior of the scene.
pub trait Scene {
    fn draw(&mut self);

    fn handle_event(&mut self, event: Event);
}