use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    text::Text,
};
use limine::framebuffer::Framebuffer;


/// Clears the screen with a given color.
pub fn clear_screen(fb: &Framebuffer, color: u32) {
    let addr = fb.addr() as *mut u32;
    let width = fb.width() as usize;
    let height = fb.height() as usize;
    let pitch = fb.pitch() as usize / 4;

    unsafe {
        for y in 0..height {
            for x in 0..width {
                *addr.add(y * pitch + x) = color;
            }
        }
    }
}

/// Draws text on the framebuffer at the given coordinates.
pub fn draw_text(fb: &Framebuffer, text: &str, x: i32, y: i32, color: u32) {
    let text_style = MonoTextStyle::new(&FONT_6X10, Rgb888::new((color >> 16) as u8, (color >> 8) as u8, color as u8));
    let mut framebuffer_drawer = FramebufferWrapper::new(fb);
    Text::new(text, Point::new(x, y), text_style).draw(&mut framebuffer_drawer).unwrap();
}

/// Simple wrapper to implement DrawTarget for the framebuffer.
struct FramebufferWrapper<'a> {
    fb: &'a Framebuffer<'a>,
}

impl<'a> FramebufferWrapper<'a> {
    fn new(fb: &'a Framebuffer) -> Self {
        Self { fb }
    }
}

impl<'a> OriginDimensions for FramebufferWrapper<'a> {
    fn size(&self) -> Size {
        Size::new(self.fb.width() as u32, self.fb.height() as u32)
    }
}

impl<'a> DrawTarget for FramebufferWrapper<'a> {
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let addr = self.fb.addr() as *mut u32;
        let pitch = self.fb.pitch() as usize / 4;

        for Pixel(Point { x, y }, color) in pixels {
            if x >= 0 && y >= 0 && (x as usize) < self.fb.width() as usize && (y as usize) < self.fb.height() as usize {
                let offset = (y as usize) * pitch + (x as usize);
                unsafe {
                    *addr.add(offset) = (0xFF << 24) | ((color.r() as u32) << 16) | ((color.g() as u32) << 8) | (color.b() as u32);
                }
            }
        }
        Ok(())
    }
}