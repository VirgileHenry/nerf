use std::num::NonZeroU32;

use crate::{
    app::event::AppEvent, drawing::canvas::Canvas, geometry::size_requirements::WidgetSizeRequirement, Rect, Widget
};

enum SizedBoxConstraints {
    Width(NonZeroU32),
    Height(NonZeroU32),
    Both(NonZeroU32, NonZeroU32),
}

/// The Sized box provide fixed size constraints to it's child.
/// It can have vertical, horizontal or both constraints.
/// It is NOT guaranteed that the child will have the provided size, if the parent does not have enough space.
/// However, best efforts will be made to respect the constraints. 
pub struct SizedBox<UserEvent, Child: Widget<UserEvent>> {
    _m: core::marker::PhantomData<UserEvent>,
    constraints: SizedBoxConstraints,
    child: Child,
}

impl<UserEvent, Child: Widget<UserEvent>> SizedBox<UserEvent, Child> {
    pub fn new(width: u32, height: u32, child: Child) -> Self {
        let width = NonZeroU32::new(width).expect("Width of a sized box must be non-zero");
        let height = NonZeroU32::new(height).expect("Height of a sized box must be non-zero");
        SizedBox {
            _m: core::marker::PhantomData,
            constraints: SizedBoxConstraints::Both(width, height),
            child,
        }
    }

    pub fn width(width: u32, child: Child) -> Self {
        let width = NonZeroU32::new(width).expect("Width of a sized box must be non-zero");
        SizedBox {
            _m: core::marker::PhantomData,
            constraints: SizedBoxConstraints::Width(width),
            child,
        }
    }

    pub fn height(height: u32, child: Child) -> Self {
        let height = NonZeroU32::new(height).expect("Height of a sized box must be non-zero");
        SizedBox {
            _m: core::marker::PhantomData,
            constraints: SizedBoxConstraints::Height(height),
            child,
        }
    }

    fn get_width(&self, available_space: NonZeroU32) -> NonZeroU32 {
        #[cfg(debug_assertions)]
        // in debug mode, don't use the min that are far more optimized (no branching) but allow overflow debug
        match &self.constraints {
            SizedBoxConstraints::Width(width) => if *width > available_space {
                println!("Overflow : SizedBox does not have enough horizontal space ({}) to meet its size constraints ({}).", available_space, width);
                    available_space
                } else { *width },
            SizedBoxConstraints::Height(_) => available_space,
            SizedBoxConstraints::Both(width, _) => if *width > available_space {
                    println!("Overflow : SizedBox does not have enough horizontal space ({}) to meet its size constraints ({}).", available_space, width);
                    available_space
                } else { *width },
        }
        #[cfg(not(debug_assertions))]
        match &self.constraints {
            SizedBoxConstraints::Width(width) => available_space.min(*width),
            SizedBoxConstraints::Height(_) => available_space,
            SizedBoxConstraints::Both(width, _) => available_space.min(*width),
        }
    }

    fn get_height(&self, available_space: NonZeroU32) -> NonZeroU32 {
        #[cfg(debug_assertions)]
        // in debug mode, don't use the min that are far more optimized (no branching) but allow overflow debug
        match &self.constraints {
            SizedBoxConstraints::Width(_) => available_space,
            SizedBoxConstraints::Height(height) => if *height > available_space {
                println!("Overflow : SizedBox does not have enough vertical space ({}) to meet its size constraints ({}).", available_space, height);
                available_space
            } else { *height },
            SizedBoxConstraints::Both(_, height) => if *height > available_space {
                println!("Overflow : SizedBox does not have enough vertical space ({}) to meet its size constraints ({}).", available_space, height);
                available_space
            } else { *height },
        }
        #[cfg(not(debug_assertions))]
        match &self.constraints {
            SizedBoxConstraints::Width(_) => available_space,
            SizedBoxConstraints::Height(height) => available_space.min(*height),
            SizedBoxConstraints::Both(_, height) => available_space.min(*height),
        }
    }
}

impl<UserEvent, Child: Widget<UserEvent>> Widget<UserEvent> for SizedBox<UserEvent, Child> {
    type EventResponse = Child::EventResponse;
    fn draw(&self, buffer: &mut Canvas, rect: Rect) {
        let rect = softbuffer::Rect {
            x: rect.x,
            y: rect.y,
            width: self.get_width(rect.width),
            height: self.get_height(rect.height),
        };
        self.child.draw(buffer, rect);
    }

    fn min_space_requirements(&self) -> (WidgetSizeRequirement, WidgetSizeRequirement) {
        match self.constraints {
            SizedBoxConstraints::Both(width, height) => (
                WidgetSizeRequirement::Fixed { size: width },
                WidgetSizeRequirement::Fixed { size: height },
            ),
            SizedBoxConstraints::Width(width) => (
                WidgetSizeRequirement::Fixed { size: width },
                self.child.min_space_requirements().1,
            ),
            SizedBoxConstraints::Height(height) => (
                self.child.min_space_requirements().0,
                WidgetSizeRequirement::Fixed { size: height },
            ),
        }
    }

    fn handle_event(&mut self, event: &AppEvent<UserEvent>, rect: Rect) -> Self::EventResponse {
        self.child.handle_event(event, rect)
    }
}