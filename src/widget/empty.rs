use std::num::NonZeroU32;

use crate::{geometry::size_requirements::WidgetSizeRequirement, drawing::canvas::Canvas};

use super::Widget;

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
    pub fn shrink() -> Box<Empty> {
        Box::new(Empty {
            behavior: EmptyBehavior::Shrink,
        })
    }

    pub fn expand() -> Box<Empty> {
        Box::new(Empty {
            behavior: EmptyBehavior::Expand,
        })
    }
}

impl Widget for Empty {
    fn draw(&self, _buffer: &mut Canvas, _rect: softbuffer::Rect) {}

    fn min_space_requirements(&self) -> (WidgetSizeRequirement, WidgetSizeRequirement) {
        match self.behavior {
            EmptyBehavior::Shrink => (WidgetSizeRequirement::None, WidgetSizeRequirement::None),
            EmptyBehavior::Expand => (
                WidgetSizeRequirement::Flex(unsafe { NonZeroU32::new_unchecked(1) }),
                WidgetSizeRequirement::Flex(unsafe { NonZeroU32::new_unchecked(1) })
            ),
        }
    }

    fn handle_event(&mut self, _event: crate::app::event::input_event::InputEvent, _rect: softbuffer::Rect) -> bool {
        false
    }
}