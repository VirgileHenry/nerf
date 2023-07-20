use crate::{
    Widget,
    geometry::size_requirements::WidgetSizeRequirement,
    drawing::{color::Color, canvas::Canvas}, app::event::{input_event::InputEvent, event_responses::EventResponse}
};


/// The background widget will draw a colored background behind its child.
/// This is a drawing widget that is in the default widgets, because filling a rect have a non-skia backup.
pub struct Background {
    child: Box<dyn Widget>,
    color: Color,
}

impl Background {
    pub fn new(color: Color, child: Box<dyn Widget>) -> Box<Background> {
        Box::new(Background {
            child,
            color,
        })
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl Widget for Background {
    fn draw(&self, canvas: &mut Canvas, rect: softbuffer::Rect) {
        canvas.fill_rect(rect, self.color);
        self.child.draw(canvas, rect);
    }

    fn min_space_requirements(&self) -> (WidgetSizeRequirement, WidgetSizeRequirement) {
        self.child.min_space_requirements()       
    }

    fn handle_event(&mut self, event: InputEvent, rect: softbuffer::Rect) -> EventResponse {
        self.child.handle_event(event, rect)
    }
}