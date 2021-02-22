use sdl2::{Sdl, VideoSubsystem};
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

const WINDOW_TITLE: &str = "Dummy Graphics Pipeline";
const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 640;

pub struct BitmapOutput {
    canvas: WindowCanvas,
}

impl BitmapOutput {
    pub fn new(canvas: WindowCanvas) -> BitmapOutput {
        BitmapOutput {
            canvas,
        }
    }

    pub fn size(&self) -> (u32, u32) {
        self.canvas.window().size()
    }

    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    pub fn put_pixel(&mut self, x:i32, y:i32, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.draw_point((x, y)).ok();
    }

    pub fn present(&mut self) {
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

    pub fn create_video_output(&self) -> BitmapOutput {
        let window = self.video_subsystem.window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        BitmapOutput::new(window.into_canvas().build().unwrap())
    }

    pub fn run<R: FnMut() -> ()>(&self, mut render: R) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'running: loop {
            render();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyUp { keycode: Some(Keycode::Escape),.. } => {
                        break 'running;
                    },
                    _ => {}
                }
            }

            ::std::thread::sleep(Duration::from_nanos(1_000_000_000u64 / 60));
        }
    }
}
