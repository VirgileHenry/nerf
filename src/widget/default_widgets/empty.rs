use std::num::NonZeroU32;

use crate::{
    app::event::AppEvent, drawing::canvas::Canvas, geometry::size_requirements::WidgetSizeRequirement, Rect, Widget
};

pub enum EmptyBehavior {
    /// The empty widget will take all the available space.
    Expand,
    /// The empty widget will take no space.
    Shrink,
}

/// This is an empty widget, used as a node in widget trees.
pub struct Empty {
    behavior: EmptyBehavior,
}

impl Empty {
    pub fn shrink() -> Empty {
        Empty {
            behavior: EmptyBehavior::Shrink,
        }
    }

    pub fn expand() -> Empty {
        Empty {
            behavior: EmptyBehavior::Expand,
        }
    }
}

impl<UserEvent> Widget<UserEvent> for Empty {
    type EventResponse = ();

    fn draw(&self, _buffer: &mut Canvas, _rect: Rect) {}

    fn min_space_requirements(&self) -> (WidgetSizeRequirement, WidgetSizeRequirement) {
        match self.behavior {
            EmptyBehavior::Shrink => (WidgetSizeRequirement::None, WidgetSizeRequirement::None),
            EmptyBehavior::Expand => (
                WidgetSizeRequirement::Flex { flex: unsafe { NonZeroU32::new_unchecked(1) } },
                WidgetSizeRequirement::Flex { flex: unsafe { NonZeroU32::new_unchecked(1) } },
            ),
        }
    }

    fn handle_event(&mut self, _: &AppEvent<UserEvent>, _: Rect) -> Self::EventResponse { }
}