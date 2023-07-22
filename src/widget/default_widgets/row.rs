use std::num::NonZeroU32;

use crate::{
    Widget,
    geometry::size_requirements::WidgetSizeRequirement,
    app::event::{input_event::InputEvent, event_responses::EventResponse}, Rect
};


/// The Row widget will take a const generic parameter number of children, and will display them vertically.
/// It will try to distribute the provided space between the children to best feed all of their needs.
/// There are no cross axis alignment, and I am not sure to add one.
/// Each children can be wrapped in a Align widget to get a cross axis alignment.
pub struct Row<const N: usize> {
    children: [Box<dyn Widget>; N],
}

impl<const N: usize> Row<N> {
    /// Creates a new Row widget from the given childs.
    pub fn new(children: [Box<dyn Widget>; N]) -> Box<Row<N>> {
        Box::new(Row {
            children,
        })
    }

    fn get_height_requirement(&self, requirements: Vec<WidgetSizeRequirement>) -> WidgetSizeRequirement {
        let mut requirement = WidgetSizeRequirement::None;

        for child_requirement in requirements.into_iter() {
            requirement = requirement | child_requirement;
        }

        requirement
    }
    
    fn get_width_requirement(&self, requirements: Vec<WidgetSizeRequirement>) -> WidgetSizeRequirement {
        let mut requirement = WidgetSizeRequirement::None;

        for child_requirement in requirements.into_iter() {
            requirement = requirement & child_requirement;
        }

        requirement
    }

}

impl<const N: usize> Widget for Row<N> {
    fn draw(&self, canvas: &mut crate::drawing::canvas::Canvas, rect: Rect) {
        let mut width_requirements = [WidgetSizeRequirement::None; N];
        for (i, child) in self.children.iter().enumerate() {
            width_requirements[i] = child.min_space_requirements().0;
        }
        let widths = WidgetSizeRequirement::distribute_available_size(width_requirements, rect.width);
        let mut offset = 0;
        for (child, width) in self.children.iter().zip(widths.into_iter()) {
            match NonZeroU32::new(width) {
                Some(width) => child.draw(canvas, softbuffer::Rect {
                    x: rect.x + offset,
                    y: rect.y,
                    width,
                    height: rect.height,
                }),
                None => {},
            }
            offset += width;
        }
    }

    fn min_space_requirements(&self) -> (WidgetSizeRequirement, WidgetSizeRequirement) {
        let (
            width_requirements,
            height_requirements
        ): (Vec<_>, Vec<_>) = self.children.iter().map(|child| child.min_space_requirements()).unzip();
        (
            self.get_width_requirement(width_requirements),
            self.get_height_requirement(height_requirements),
        )
    }

    fn handle_event(&mut self, event: InputEvent, rect: Rect) -> EventResponse {
        let mut width_requirements = [WidgetSizeRequirement::None; N];
        for (i, child) in self.children.iter().enumerate() {
            width_requirements[i] = child.min_space_requirements().0;
        }
        let widths = WidgetSizeRequirement::distribute_available_size(width_requirements, rect.width);
        let mut offset = 0;
        let mut result = EventResponse::NONE;
        for (child, width) in self.children.iter_mut().zip(widths.into_iter()) {
            match NonZeroU32::new(width) {
                Some(width) => result = result | child.handle_event(event.clone(), softbuffer::Rect {
                    x: rect.x + offset,
                    y: rect.y,
                    width,
                    height: rect.height,
                }),
                None => {},
            }
            offset += width;
        }
        result
    }


}