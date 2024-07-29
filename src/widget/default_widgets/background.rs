use crate::{
    app::event::AppEvent, drawing::{canvas::Canvas, color::Color}, geometry::size_requirements::WidgetSizeRequirement, Rect, Widget
};


/// The background widget will draw a colored background behind its child.
/// This is a drawing widget that is in the default widgets, because filling a rect have a non-skia backup.
pub struct Background<UserEvent, Child: Widget<UserEvent>> {
    _m: core::marker::PhantomData<UserEvent>,
    child: Child,
    color: Color,
}

impl<UserEvent, Child: Widget<UserEvent>> Background<UserEvent, Child> {
    pub fn new(color: Color, child: Child) -> Self {
        Background {
            _m: core::marker::PhantomData,
            child,
            color,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl<UserEvent, Child: Widget<UserEvent>> Widget<UserEvent> for Background<UserEvent, Child> {
    type EventResponse = Child::EventResponse;

    fn draw(&self, canvas: &mut Canvas, rect: Rect) {
        canvas.fill_rect(rect, self.color);
        self.child.draw(canvas, rect);
    }

    fn min_space_requirements(&self) -> (WidgetSizeRequirement, WidgetSizeRequirement) {
        self.child.min_space_requirements()       
    }

    fn handle_event(&mut self, event: &AppEvent<UserEvent>, rect: Rect) -> Self::EventResponse {
        self.child.handle_event(event, rect)
    }
}