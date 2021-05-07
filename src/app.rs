use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::framework::Framework;
use crate::scenes::Scene;

/// The main application class.
///
/// Responsible for the management of execution flow, event loop, the main window, 
/// scene management and shared resources.
pub struct App<'f> {
    current: usize,
    scenes: Vec<Box<dyn Scene>>,
    framework: &'f Framework,
}

impl<'f> App<'f> {
    /// Initializes the application using the specified framework.
    pub fn init(framework: &'f Framework) -> Self {
        Self {
            current : 0,
            scenes: Vec::new(),
            framework,
        }
    }

    /// Adds the scene into the application.
    pub fn add_scene<S: Scene + 'static>(&mut self, scene: S) {
        self.scenes.push(Box::new(scene));
    }

    /// Begins the application's main loop.
    ///
    /// The application runs at a predeterminate framerate, with the updating and drawing 
    /// routines running synchronically. First, the event queue is checked - in which events
    /// are first handled by the application, then forwarded to the scene, then the current
    /// scene is updated and drawn, in that order.
    pub fn run(mut self) {
        let mut event_pump = self.framework.get_event_queue();
        let length = self.scenes.len();
        
        'running: loop {
            // Fetch the current scene this frame.
            let scene = &mut self.scenes[self.current];

            // Check event queue.
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyUp { keycode: Some(Keycode::Escape),.. } => {
                        break 'running;
                    },
                    Event::KeyUp { keycode: Some(Keycode::LeftBracket), ..} => {
                        self.current = match self.current.checked_sub(1) {
                            Some(v) => v,
                            None => length - 1,
                        }
                    },
                    Event::KeyUp { keycode: Some(Keycode::RightBracket), ..} => {
                        self.current = (self.current + 1) % length;
                    },
                    _ => {}
                }
                scene.handle_event(event);
            }

            scene.draw();

            ::std::thread::sleep(Duration::from_nanos(1_000_000_000u64 / 60));
        }
    }
}