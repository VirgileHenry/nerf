

pub struct Canvas<'a> {
    buffer: softbuffer::Buffer<'a>,
    surface_width: u32,
}

/// A canvas is a buffer that can be drawn on.
impl<'a> Canvas<'a> {
    pub fn new(surface: &'a mut softbuffer::Surface, surface_width: u32) -> Canvas<'a> {
        Canvas {
            buffer: surface.buffer_mut().unwrap(),
            surface_width,
        }
    }

    pub fn buffer(self) -> softbuffer::Buffer<'a> {
        self.buffer
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, color: u32){
        self.buffer[(y * self.surface_width + x) as usize] = color;
    }

    pub fn fill_rect(&mut self, rect: softbuffer::Rect, color: u32) {
        for y in rect.y..rect.y + rect.height.get() {
            self.buffer[(y * self.surface_width + rect.x) as usize..(y * self.surface_width + rect.x + rect.width.get()) as usize].fill(color);
        }
    }

}