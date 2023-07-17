use std::num::NonZeroU32;

use crate::{Widget, geometry::size_requirements::WidgetSizeRequirement};





pub struct Expanded {
    flex: NonZeroU32,
    child: Box<dyn Widget>,
}



impl Widget for Expanded {
    fn draw(&self, canvas: &mut crate::drawing::canvas::Canvas, rect: softbuffer::Rect) {
        self.child.draw(canvas, rect);
    }

    fn min_space_requirements(&self) -> (WidgetSizeRequirement, WidgetSizeRequirement) {
        (
            WidgetSizeRequirement::Flex(self.flex),
            WidgetSizeRequirement::Flex(self.flex),
        )
    }
}







