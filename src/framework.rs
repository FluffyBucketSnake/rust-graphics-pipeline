use sdl2::{EventPump, Sdl, VideoSubsystem};
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;

use crate::graphics::BitmapOutput;

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
/// The backend of the application.
///
/// Responsible for setting up the low-level libraries and frameworks used for running the 
/// application.
pub struct Framework {
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
}

// TODO: Introduce proper error handling
impl Framework {
    /// Initializes the framework's internals systems.
    pub fn init() -> Framework {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        Framework {
            sdl_context,
            video_subsystem,
        }
    }

    // TODO: Instead of returning the bitmap rendertarget itself, the framework should only 
    // return a handle to the window, so the App shall be responsible for managing the 
    // distribution of the rendertarget.
    pub fn create_video_output(&self) -> CanvasOutput {
        let window = self.video_subsystem.window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        CanvasOutput::from(window.into_canvas().build().unwrap())
    }

    // TODO: Abstract event queue.
    pub fn get_event_queue(&self) -> EventPump {
        self.sdl_context.event_pump().unwrap()
    }
}
