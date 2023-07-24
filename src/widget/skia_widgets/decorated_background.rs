

use crate::{
    Widget,
    geometry::size_requirements::WidgetSizeRequirement,
    drawing::{color::Color, canvas::Canvas}, app::event::{input_event::InputEvent, event_responses::EventResponse}, Rect, BorderType
};


/// The background widget will draw a colored background behind its child.
/// This is a drawing widget that is in the default widgets, because filling a rect have a non-skia backup.
pub struct DecoratedBackground {
    child: Box<dyn Widget>,
    fill_color: Option<Color>,
    border_color: Option<Color>,
    border_type: BorderType,
    border_width: u32,
    corner_radius: u32,
}

impl DecoratedBackground {
    pub fn new(
        fill_color: Option<Color>,
        border_color: Option<Color>,
        border_type: BorderType,
        border_width: u32,
        corner_radius: u32,
        child: Box<dyn Widget>
    ) -> Box<DecoratedBackground> {
        Box::new(DecoratedBackground {
            child,
            fill_color,
            border_color,
            border_type,
            border_width,
            corner_radius,
        })
    }

}

impl Widget for DecoratedBackground {
    fn draw(&self, canvas: &mut Canvas, rect: Rect) {
        canvas.fill_decorated(
            rect,
            self.fill_color,
            self.border_color,
            self.border_type,
            self.corner_radius,
            self.border_width,
        );
        self.child.draw(canvas, rect);
    }

    fn min_space_requirements(&self) -> (WidgetSizeRequirement, WidgetSizeRequirement) {
        self.child.min_space_requirements()       
    }

    fn handle_event(&mut self, event: InputEvent, rect: Rect) -> EventResponse {
        self.child.handle_event(event, rect)
    }
}