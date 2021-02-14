use sdl2::{Sdl, VideoSubsystem};
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use super::vec::Vec2f;

const WINDOW_TITLE: &str = "Dummy Graphics Pipeline";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

// Represents the application proper. Responsible for handling backend stuff for the pipeline program, 
// such as setting up the backend libraries and systems, as well as creating and handling the windows.
pub struct Framework {
    sdl_context: Sdl,
    canvas: WindowCanvas,
}

// TODO: Introduce proper error handling
impl Framework {
    pub fn init() -> Framework {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        Framework {
            sdl_context,
            canvas,
        }
    }

    pub fn run(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'running: loop {
            self.render();

            for event in event_pump.poll_iter() {
                if let Err(()) = self.handle_event(event) {
                    break 'running;
                }
            }

            ::std::thread::sleep(Duration::from_nanos(1_000_000_000u64 / 60));
        }
    }

    // TODO: Properly handle events, instead of sticking in Result
    fn handle_event(&mut self, event: Event) -> Result<(),()> {
        match event {
            Event::Quit {..} |
            Event::KeyUp { keycode: Some(Keycode::Escape),.. } => {
                Err(())
            },
            _ => Ok(()) 
        }
    }

    fn render(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.draw_line(Vec2f::zero(), Vec2f::from_uniform(64.0)).unwrap();
        self.canvas.draw_line(Vec2f::new(64.0, 0.0), Vec2f::new(64.0, 0.0) + Vec2f::from_uniform(64.0)).unwrap();

        self.canvas.present();
    }
}
