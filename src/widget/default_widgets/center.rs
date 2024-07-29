use std::num::NonZeroU32;

use crate::{
    app::event::AppEvent, 
    drawing::canvas::Canvas,
    geometry::size_requirements::WidgetSizeRequirement,
    Rect,
    Widget
};

/// Centers it's children in the available space.
/// If the children takes all available space, this widget is useless and will give all space to children.
/// This widget is a shortcut for Align(Alignment::Center, child).
pub struct Center<UserEvent, Child: Widget<UserEvent>> {
    _m: core::marker::PhantomData<UserEvent>,
    child: Child,
}

impl<UserEvent, Child: Widget<UserEvent>> Center<UserEvent, Child> {
    pub fn new(child: Child) -> Self {
        Center {
            _m: core::marker::PhantomData,
            child,
        }
    }

    fn get_child_size_and_spacing(child_requirement: WidgetSizeRequirement, available_size: NonZeroU32) -> (NonZeroU32, u32) {
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
                available_size.get().checked_sub(max.get()).unwrap_or(0) / 2
            ),
        }
    }

    fn compute_child_rect(&self, from_rect: softbuffer::Rect) -> softbuffer::Rect {
        let (
            width_requirement,
            height_requirement
        ) = self.child.min_space_requirements();
        let (width, width_spacing) = Self::get_child_size_and_spacing(width_requirement, from_rect.width);
        let (height, height_spacing) = Self::get_child_size_and_spacing(height_requirement, from_rect.height);
        softbuffer::Rect {
            x: from_rect.x + width_spacing,
            y: from_rect.y + height_spacing,
            width,
            height,
        }
    }
}

impl<UserEvent, Child: Widget<UserEvent>> Widget<UserEvent> for Center<UserEvent, Child> {
    type EventResponse = Child::EventResponse;
    fn draw(&self, buffer: &mut Canvas, rect: Rect) {
        let rect = self.compute_child_rect(rect);
        self.child.draw(buffer, rect);
    }

    fn min_space_requirements(&self) -> (WidgetSizeRequirement, WidgetSizeRequirement) {
        self.child.min_space_requirements()
    }

    fn handle_event(&mut self, event: &AppEvent<UserEvent>, rect: Rect) -> Self::EventResponse {
        self.child.handle_event(event, self.compute_child_rect(rect))
    }
}