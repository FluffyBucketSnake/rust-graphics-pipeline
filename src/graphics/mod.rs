mod clipping;
mod pipeline;
mod primitives;

pub use self::pipeline::*;
pub use self::primitives::*;

use crate::color::Color;

pub trait BitmapOutput {
    fn size(&self) -> (u32, u32);
    fn clear(&mut self, color: Color);
    fn put_pixel(&mut self, position: (u32, u32), color: Color);
    fn present(&mut self);
}

pub trait GPU<InputData, OutputTarget> {
    fn draw(&self, input: InputData, output: &mut OutputTarget);
}
