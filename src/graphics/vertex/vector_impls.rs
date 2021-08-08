use cgmath::Vector4;

use crate::graphics::Vertex;

impl Vertex for Vector4<f32> {
    fn position(&self) -> Vector4<f32> {
        *self
    }

    fn to_screen_coords(&mut self, width: f32, height: f32) {
        *self /= self.w;

        self.x += 1.0;
        self.x *= width / 2.0;
        self.y -= 1.0;
        self.y *= -height / 2.0;
    }
    
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn vector4_to_screen_coords() {
        
    }
}