use sdl2::{EventPump, Sdl, VideoSubsystem};
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::graphics::RenderTarget;

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

    /// Constructs a window with the given attributes.
    pub fn create_window(&self, title: &str, width: u32, height: u32) -> Window {
        let window = self.video_subsystem.window(title, width, height)
            .position_centered()
            .build()
            .unwrap();
        Window::from(window.into_canvas().build().unwrap())
    }

    // TODO: Abstract event queue.
    pub fn get_event_queue(&self) -> EventPump {
        self.sdl_context.event_pump().unwrap()
    }
}

pub struct Window {
    canvas: Rc<RefCell<WindowCanvas>>,
}

impl Window {
    pub fn get_rendertarget(&self) -> WindowTarget {
        WindowTarget::from(Rc::downgrade(&self.canvas))
    }
}

impl From<WindowCanvas> for Window {
    fn from(canvas: WindowCanvas) -> Window {
        Window {
            canvas: Rc::new(RefCell::new(canvas)),
        }
    }
}

pub struct WindowTarget {
    canvas: Weak<RefCell<WindowCanvas>>,
}

impl RenderTarget for WindowTarget {
    fn size(&self) -> (u32, u32) {
        self.canvas.upgrade().unwrap().borrow().window().size()
    }

    fn clear(&mut self, color: Color) {
        let rc = self.canvas.upgrade().unwrap();
        let mut canvas = rc.borrow_mut();
        canvas.set_draw_color(color);
        canvas.clear();
    }

    fn put_pixel(&mut self, position: (u32, u32), color: Color) {
        let rc = self.canvas.upgrade().unwrap();
        let mut canvas = rc.borrow_mut();
        canvas.set_draw_color(color);
        canvas.draw_point(sdl2::rect::Point::new(position.0 as i32, position.1 as i32)).ok();
    }

    fn present(&mut self) {
        let rc = self.canvas.upgrade().unwrap();
        let mut canvas = rc.borrow_mut();
        canvas.present();
    }
}

impl From<Weak<RefCell<WindowCanvas>>> for WindowTarget {
    fn from(canvas: Weak<RefCell<WindowCanvas>>) -> Self {
        Self { canvas }
    }
}