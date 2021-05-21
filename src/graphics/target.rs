use crate::math::Color;

/// Interface for output bitmap buffer.
pub trait RenderTarget {
    /// Returns the dimensions of this buffer.
    fn size(&self) -> (u32, u32);

    /// Fills the buffer with the color.
    fn clear(&mut self, color: Color);

    /// Sets the color value at the pixel coordinates.
    fn put_pixel(&mut self, position: (u32, u32), color: Color);

    /// Tests and sets an value on the depth buffer if the value is smaller than what is 
    /// currently stored.
    fn test_and_set_depth(&mut self, position: (u32, u32), depth: f32) -> bool;
    
    /// Applies any rendering operations since last call.
    /// 
    /// Used in bitmap outputs that use a backbuffer.
    fn present(&mut self);
}