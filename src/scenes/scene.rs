/// View scenes interface.
///
/// This traits offers methods for defining the behavior of the scene.
pub trait Scene {
    fn initialize(&mut self);

    fn draw(&mut self);

    fn update(&mut self);
}