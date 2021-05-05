use sdl2::{Sdl, VideoSubsystem};
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use crate::graphics::BitmapOutput;
use crate::scenes::Scene;

const WINDOW_TITLE: &str = "Dummy Graphics Pipeline";
const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 640;

pub struct CanvasOutput {
    canvas: WindowCanvas,
}

impl From<WindowCanvas> for CanvasOutput {
    fn from(canvas: WindowCanvas) -> CanvasOutput {
        CanvasOutput {
            canvas,
        }
    }

}

impl BitmapOutput for CanvasOutput {
    fn size(&self) -> (u32, u32) {
        self.canvas.window().size()
    }

    fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn put_pixel(&mut self, position: (u32, u32), color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.draw_point(sdl2::rect::Point::new(position.0 as i32, position.1 as i32)).ok();
    }

    fn present(&mut self) {
        self.canvas.present();
    }
}

// Represents the application proper. Responsible for handling backend stuff for the pipeline program, 
// such as setting up the backend libraries and systems, as well as creating and handling the windows.
pub struct Framework {
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
}

// TODO: Introduce proper error handling
impl Framework {
    pub fn init() -> Framework {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        Framework {
            sdl_context,
            video_subsystem,
        }
    }

    pub fn create_video_output(&self) -> CanvasOutput {
        let window = self.video_subsystem.window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        CanvasOutput::from(window.into_canvas().build().unwrap())
    }

    pub fn run<S: Scene>(&self, mut scene: S) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyUp { keycode: Some(Keycode::Escape),.. } => {
                        break 'running;
                    },
                    _ => {}
                }
            }

            scene.update();
            scene.draw();

            ::std::thread::sleep(Duration::from_nanos(1_000_000_000u64 / 60));
        }
    }
}
