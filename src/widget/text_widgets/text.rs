use std::num::NonZeroU32;

use crate::{
    Widget,
    WidgetSizeRequirement,
    Canvas,
    TextStyle,
    Rect
};

pub(crate) mod text_overflow;

pub type TextAlign = cosmic_text::Align;

pub struct Text<UserEvent> {
    _m: core::marker::PhantomData<UserEvent>,
    text: String,
    style: TextStyle,
}

impl<UserEvent> Text<UserEvent> {
    pub fn new(text: String, style: TextStyle) -> Self {
        Text {
            _m: core::marker::PhantomData,
            text,
            style,
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

impl<UserEvent> Widget<UserEvent> for Text<UserEvent> {
    type EventResponse = ();
    fn draw(&self, canvas: &mut Canvas, rect: Rect) {
        canvas.draw_text(&self.text, rect, &self.style);
    }

    fn min_space_requirements(&self) -> (WidgetSizeRequirement, WidgetSizeRequirement) {
        (
            WidgetSizeRequirement::Flex { flex: unsafe { NonZeroU32::new_unchecked(1) } },
            WidgetSizeRequirement::Flex { flex: unsafe { NonZeroU32::new_unchecked(1) } },
        )
    }

    fn handle_event(&mut self, _: &crate::app::event::AppEvent<UserEvent>, _: Rect) -> Self::EventResponse {
        ()
    }
}