
use cgmath::Vector4;
use sdl2::surface::Surface;
use sdl2::sys::{SDL_BYTEORDER, SDL_BIG_ENDIAN};
use sdl2::pixels::Color;

pub fn sample_texture(texture: &Surface, x: f32, y: f32) -> Vector4<f32> {
    // Source: https://stackoverflow.com/questions/53033971/how-to-get-the-color-of-a-specific-pixel-from-sdl-surface
    let raw = unsafe {
        let raw_surface = texture.raw();
        let bpp = (*(*raw_surface).format).BytesPerPixel as isize;
        let pitch = (*raw_surface).pitch as isize;
        let x = x.max(0.0).min(1.0);
        let y = y.max(0.0).min(1.0);
        let x = (x * (texture.width() as f32)) as isize;
        let y = (y * (texture.height() as f32)) as isize;
        let p = ((*texture.raw()).pixels as *mut u8).offset((y * pitch) + (x * bpp));
        match bpp {
            1 => *p as u32,
            2 => *(p as *mut u16) as u32,
            3 => {
                if SDL_BYTEORDER == SDL_BIG_ENDIAN {
                    (*p as u32) << 16 | (*p.offset(1) as u32) << 8 | *p.offset(2) as u32
                } else {
                    (*p as u32) | (*p.offset(1) as u32) << 8 | (*p.offset(2) as u32) << 16
                }
            }
            4 => *(p as *mut u32),
            _ => unreachable!(),
        }
    };
    crate::math::color_to_vector4(&Color::from_u32(&texture.pixel_format(), raw))
}