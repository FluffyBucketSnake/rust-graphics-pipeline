use sdl2::pixels::Color;

// pub trait Bitmap {
//     fn data_at() -> Color;
//     fn set_data_at(x: u32, y: u32, data: Color);
// }

pub struct Bitmap {
    buffer: Vec<Color>,
    width: usize,
    height: usize,
}

impl Bitmap {
    pub fn from_color(width: usize, height: usize, color: Color) -> Self {
        let buffer = vec![color; width * height];
        Self {
            buffer,
            width,
            height,
        }
    }

    pub fn from_function<F>(width: usize, height: usize, function: F) -> Self
    where
        F: Fn(usize, usize) -> Color,
    {
        let mut result = Self::from_color(width, height, Color::BLACK);
        for y in 0..height {
            for x in 0..width {
                *result.data_at_mut(x, y) = function(x, y);
            }
        }
        result
    }

    pub fn data(&self) -> &[Color] {
        &self.buffer
    }

    pub fn data_mut(&mut self) -> &mut [Color] {
        &mut self.buffer
    }

    pub fn data_at(&self, x: usize, y: usize) -> &Color {
        &self.buffer[(y * self.width) + x]
    }

    pub fn data_at_mut(&mut self, x: usize, y: usize) -> &mut Color {
        &mut self.buffer[(y * self.width) + x]
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitmap_from_color() {
        // Given
        let width = 32;
        let height = 16;
        let color = Color::WHITE;

        // When
        let bitmap = Bitmap::from_color(width, height, color);

        // Then
        assert_dimensions(width, height, &bitmap);
        for i in bitmap.data().iter() {
            assert_eq!(color, *i);
        }
    }

    #[test]
    fn bitmap_from_function() {
        // Given
        let width = 32;
        let height = 16;
        let color_function = |x: usize, y: usize| { 
            if (x + y) % 2 == 1 { Color::WHITE } else { Color::BLACK }
        };

        // When
        let bitmap = Bitmap::from_function(width, height, color_function);

        // Then
        assert_dimensions(width, height, &bitmap);
        for x in 0..width {
            for y in 0..height {
                assert_eq!(color_function(x, y), *bitmap.data_at(x, y));
            }
        }
    }

    fn assert_dimensions(width: usize, height: usize, bitmap: &Bitmap) {
        assert_eq!(width, bitmap.width());
        assert_eq!(height, bitmap.height());
    }
}