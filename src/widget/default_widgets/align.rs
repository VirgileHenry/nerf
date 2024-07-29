use std::num::NonZeroU32;

use crate::{
    app::event::AppEvent, geometry::{
        alignment::Alignment,
        size_requirements::WidgetSizeRequirement
    }, Rect, Widget
};



/// The Align widget will take the same space as it's child.
/// However, if we provide it with more space, it will align the child in the extra space,
/// according the the alignment parameter.
pub struct Align<UserEvent, Child: Widget<UserEvent>> {
    _m: core::marker::PhantomData<UserEvent>,
    child: Child,
    alignment: Alignment,
}

impl<UserEvent, Child: Widget<UserEvent>> Align<UserEvent, Child> {
    pub fn new(alignment: Alignment, child: Child) -> Self {
        Align {
            _m: core::marker::PhantomData,
            child,
            alignment,
        }
    }

    fn get_child_and_remaining_size(child_requirement: WidgetSizeRequirement, available_size: NonZeroU32) -> (NonZeroU32, u32) {
        match child_requirement {
            // child can have any size, so we give it all the available space.
            WidgetSizeRequirement::None |
            WidgetSizeRequirement::Flex { .. } |
            WidgetSizeRequirement::Min { .. } => (available_size, 0),
            // child does not want to be bigger than a given size,
            // so we give it the minimum between the available space and the size it wants.
            WidgetSizeRequirement::Fixed { size: max, .. } |
            WidgetSizeRequirement::Max { max, .. } |
            WidgetSizeRequirement::MinMax { max, .. } => (
                max.min(available_size),
                available_size.get().checked_sub(max.get()).unwrap_or(0)
            ),
        }
    }

    fn compute_child_rect(&self, from_rect: softbuffer::Rect) -> softbuffer::Rect {
        let (width_req, height_req) = self.child.min_space_requirements();
        let (child_width, remaining_width) = Self::get_child_and_remaining_size(width_req, from_rect.width);
        let (child_height, remaining_height) = Self::get_child_and_remaining_size(height_req, from_rect.height);
        softbuffer::Rect {
            x: from_rect.x + self.alignment.get_left_space(remaining_width),
            y: from_rect.y + self.alignment.get_top_space(remaining_height),
            width: child_width,
            height: child_height,
        }
    }
}

impl<UserEvent, Child: Widget<UserEvent>> Widget<UserEvent> for Align<UserEvent, Child> {
    type EventResponse = Child::EventResponse;
    fn draw(&self, canvas: &mut crate::drawing::canvas::Canvas, rect: Rect) {
        let child_rect = self.compute_child_rect(rect);
        self.child.draw(canvas, child_rect);
    }

    fn min_space_requirements(&self) -> (crate::geometry::size_requirements::WidgetSizeRequirement, crate::geometry::size_requirements::WidgetSizeRequirement) {
        self.child.min_space_requirements()
    }

    fn handle_event(&mut self, event: &AppEvent<UserEvent>, rect: Rect) -> Self::EventResponse {
        let child_rect = self.compute_child_rect(rect);
        self.child.handle_event(event, child_rect)
    }
}


