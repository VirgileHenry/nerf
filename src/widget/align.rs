use std::num::NonZeroU32;

use crate::{
    Widget,
    geometry::{alignment::Alignment, size_requirements::WidgetSizeRequirement}
};



/// The Align widget will take the same space as it's child.
/// However, if we provide it with more space, it will align the child in the extra space,
/// according the the alignment parameter.
pub struct Align {
    child: Box<dyn Widget>,
    alignment: Alignment,
}

impl Align {
    pub fn new(alignment: Alignment, child: Box<dyn Widget>) -> Box<Align> {
        Box::new(Align {
            child,
            alignment,
        })
    }

    pub fn get_child_and_remaining_size(requirement: WidgetSizeRequirement, available_size: NonZeroU32) -> (NonZeroU32, u32) {
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
                    (size, available_size.get() - size.get())
                }
            },
        }
    }
}

impl Widget for Align {
    fn draw(&self, canvas: &mut crate::drawing::canvas::Canvas, rect: softbuffer::Rect) {
        let (width_req, height_req) = self.child.min_space_requirements();
        let (child_width, remaining_width) = Self::get_child_and_remaining_size(width_req, rect.width);
        let (child_height, remaining_height) = Self::get_child_and_remaining_size(height_req, rect.height);
        let child_rect = softbuffer::Rect {
            x: rect.x + self.alignment.get_left_space(remaining_width),
            y: rect.y + self.alignment.get_top_space(remaining_height),
            width: child_width,
            height: child_height,
        };
        self.child.draw(canvas, child_rect);
    }

    fn min_space_requirements(&self) -> (crate::geometry::size_requirements::WidgetSizeRequirement, crate::geometry::size_requirements::WidgetSizeRequirement) {
        self.child.min_space_requirements()
    }
}


