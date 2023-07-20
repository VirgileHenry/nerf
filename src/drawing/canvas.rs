use crate::Color;



pub struct Canvas<'a> {
    buffer: softbuffer::Buffer<'a>,
    surface_width: u32,
    surface_height: u32,
}

/// A canvas is a buffer that can be drawn on.
impl<'a> Canvas<'a> {
    pub fn new(
        surface: &'a mut softbuffer::Surface,
        surface_width: u32,
        surface_height: u32,
    ) -> Canvas<'a> {
        let buffer = surface.buffer_mut().unwrap();
        Canvas {
            buffer,
            surface_width,
            surface_height,
        }
    }

    pub fn buffer(self) -> softbuffer::Buffer<'a> {
        self.buffer
    }


}


#[cfg(feature = "skia")]
impl<'a> Canvas<'a> {
    pub fn fill_rect(&mut self, rect: softbuffer::Rect, color: Color) {
        // todo : create a single skia pixmap with canvas creation, and reuse it.
        let slice = unsafe {
            std::slice::from_raw_parts_mut(
                self.buffer.as_mut_ptr() as *mut u8,
                (self.surface_height * self.surface_width * 4) as usize,
            )
        };
        let mut skia_pixmap = tiny_skia::PixmapMut::from_bytes(
            slice,
            self.surface_width,
            self.surface_height,
        ).unwrap();
        skia_pixmap.fill_rect(
            Self::softbuffer_to_skia_rect(rect),
            &Color::into(color),
            tiny_skia::Transform::identity(),
            None,
        );
    }

    fn softbuffer_to_skia_rect(rect: softbuffer::Rect) -> tiny_skia::Rect {
        tiny_skia::Rect::from_xywh(
            rect.x as f32,
            rect.y as f32,
            rect.width.get() as f32,
            rect.height.get() as f32,
        ).unwrap()
    }

    
}

// If skia is not used (desabled features), we still support minimal drawing operations.
#[cfg(not(feature = "skia"))]
impl<'a> Canvas<'a> {

    pub fn draw_pixel(&mut self, x: u32, y: u32, color: u32){
        self.buffer[(y * self.surface_width + x) as usize] = color;
    }

    pub fn fill_rect(&mut self, rect: softbuffer::Rect, color: Color) {
        let color = color.value();
        for y in rect.y..rect.y + rect.height.get() {
            for x in rect.x..rect.x + rect.width.get() {
                self.draw_pixel(x, y, color);
            }
        }
    }
}