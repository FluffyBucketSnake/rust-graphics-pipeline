use crate::math::Color;

/// Interface for output bitmap buffer.
pub trait RenderTarget {
    /// Returns the dimensions of this buffer.
    fn size(&self) -> (u32, u32);

    /// Fills the buffer with the color.
    fn clear(&mut self, color: Color);

    /// Sets the color value at the pixel coordinates.
    fn put_pixel(&mut self, position: (u32, u32), color: Color);
    
    /// Applies any rendering operations since last call.
    /// 
    /// Used in bitmap outputs that use a backbuffer.
    fn present(&mut self);
}