use std::num::NonZeroU32;

use crate::{geometry::size_requirements::WidgetSizeRequirement, drawing::canvas::Canvas};

use super::Widget;

/// Centers it's children in the available space.
/// If the children takes all available space, this widget is useless and will give all space to children.
/// This widget is a shortcut for Align(Alignment::Center, child).
pub struct Center {
    child: Box<dyn Widget>,
}

impl Center {
    pub fn new(child: Box<dyn Widget>) -> Box<Center> {
        Box::new(Center {
            child,
        })
    }

    fn get_child_size_and_spacing(requirement: WidgetSizeRequirement, available_size: NonZeroU32) -> (NonZeroU32, u32) {
        match requirement {
            // child can have any size, so we give it all the available space.
            WidgetSizeRequirement::None |
            WidgetSizeRequirement::Flex(_)  |
            WidgetSizeRequirement::Min(_) => (available_size, 0),
            // child does not want to be bigger than a given size,
            // so we give it the minimum between the available space and the size it wants.
            WidgetSizeRequirement::Fixed(size) |
            WidgetSizeRequirement::Max(size) |
            WidgetSizeRequirement::MinMax(_, size) => {
                if size > available_size {
                    (available_size, 0)
                } else {
                    (size, (available_size.get() - size.get()) / 2)
                }
            },
        }
    }
}

impl Widget for Center {
    fn draw(&self, buffer: &mut Canvas, rect: softbuffer::Rect) {
        // get the size requirements of our child
        let (
            width_requirement,
            height_requirement
        ) = self.child.min_space_requirements();
        let (width, width_spacing) = Self::get_child_size_and_spacing(width_requirement, rect.width);
        let (height, height_spacing) = Self::get_child_size_and_spacing(height_requirement, rect.height);
        let rect = softbuffer::Rect {
            x: rect.x + width_spacing,
            y: rect.y + height_spacing,
            width,
            height,
        };
        self.child.draw(buffer, rect);
    }

    fn min_space_requirements(&self) -> (WidgetSizeRequirement, WidgetSizeRequirement) {
        self.child.min_space_requirements()
    }
}