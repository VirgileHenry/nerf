use std::num::NonZeroU32;

use crate::{
    app::event::AppEvent, geometry::size_requirements::WidgetSizeRequirement, Rect, Widget
};





pub struct Expanded<UserEvent, Child: Widget<UserEvent>> {
    _m: core::marker::PhantomData<UserEvent>,
    flex: NonZeroU32,
    child: Child,
}


impl<UserEvent, Child: Widget<UserEvent>> Widget<UserEvent> for Expanded<UserEvent, Child> {
    type EventResponse = Child::EventResponse;
    fn draw(&self, canvas: &mut crate::drawing::canvas::Canvas, rect: Rect) {
        self.child.draw(canvas, rect);
    }

    fn min_space_requirements(&self) -> (WidgetSizeRequirement, WidgetSizeRequirement) {
        (
            WidgetSizeRequirement::Flex { flex: self.flex },
            WidgetSizeRequirement::Flex { flex: self.flex },
        )
    }

    fn handle_event(&mut self, event: &AppEvent<UserEvent>, rect: Rect) -> Self::EventResponse {
        self.child.handle_event(event, rect)
    }
}







