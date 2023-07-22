use std::num::NonZeroU32;

use crate::{
    Widget,
    WidgetSizeRequirement,
    Canvas,
    TextStyle
};

pub(crate) mod text_overflow;

pub type TextAlign = cosmic_text::Align;

pub struct Text {
    text: String,
    style: TextStyle,
}

impl Text {
    pub fn new(text: String, style: TextStyle) -> Box<Text> {
        Box::new(Text {
            text,
            style,
        })
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

impl Widget for Text {
    fn draw(&self, canvas: &mut Canvas, rect: softbuffer::Rect) {
        canvas.draw_text(&self.text, rect, &self.style);
    }

    fn min_space_requirements(&self) -> (WidgetSizeRequirement, WidgetSizeRequirement) {
        (
            WidgetSizeRequirement::Flex(unsafe { NonZeroU32::new_unchecked(1) }),
            WidgetSizeRequirement::Flex(unsafe { NonZeroU32::new_unchecked(1) }),
        )
    }

    fn handle_event(&mut self, _event: crate::app::event::input_event::InputEvent, _rect: softbuffer::Rect) -> crate::app::event::event_responses::EventResponse {
        crate::app::event::event_responses::EventResponse::NONE
    }
}