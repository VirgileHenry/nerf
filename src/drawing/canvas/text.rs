use std::num::NonZeroU32;

use crate::{Canvas, TextStyle};




impl<'a> Canvas<'a> {
    pub fn draw_text(&mut self, text: &str, rect: softbuffer::Rect, style: &TextStyle) {
        // Text metrics indicate the font size and line height of a buffer
        // note to myself : line height is the space between the top of the buffer and the baseline of the text,
        // then, it is also the space between the baseline of the text and the baseline of the next line.
        let metrics = cosmic_text::Metrics::new(style.size, style.size + style.additional_interline);
        // the option is a hack for now, because the draw closure requires another access to self.
        // this will decouple the assets from the canvas
        let assets = self.assets.take().unwrap();
        let (font_system, swash_cache) = assets.text_mut().fonts_and_cache();

        // A Buffer provides shaping and layout for a UTF-8 string, create one per text widget
        let mut buffer = cosmic_text::Buffer::new(font_system, metrics);

        // Borrow buffer together with the font system for more convenient method calls
        let mut buffer = buffer.borrow_with(font_system);

        // Set a size for the text buffer, in pixels
        buffer.set_size(rect.width.get() as f32, rect.height.get() as f32);
        buffer.set_wrap(style.overflow.into());

        // Add some text!
        buffer.set_text(text, style.into(), cosmic_text::Shaping::Advanced);

        // Perform shaping as desired
        buffer.shape_until_scroll(); // todo what is this for ? works fine without

        // Draw the buffer (for performance, instead use SwashCache directly)
        // todo : improve perf using the above indication
        buffer.draw(swash_cache, style.color.into(), |x, y, w, h, color| {
            match (NonZeroU32::new(w), NonZeroU32::new(h), u32::try_from(x), u32::try_from(y)) {
                (Some(width), Some(height), Ok(x), Ok(y)) => {
                    let rect = softbuffer::Rect { x: rect.x + x, y: rect.y + y, width, height };
                    self.fill_rect(rect, color.into());
                },
                _ => {},
            }
        });

        self.assets = Some(assets);
    }
}