use std::num::NonZeroU32;

use crate::{
    Widget,
    geometry::size_requirements::WidgetSizeRequirement,
    app::event::{input_event::InputEvent, event_responses::EventResponse}
};


/// The Column widget will take a const generic parameter number of children, and will display them vertically.
/// It will try to distribute the provided space between the children to best feed all of their needs.
/// There are no cross axis alignment, and I am not sure to add one.
/// Each children can be wrapped in a Align widget to get a cross axis alignment.
pub struct Column<const N: usize> {
    children: [Box<dyn Widget>; N],
}

impl<const N: usize> Column<N> {
    /// Creates a new column widget from the given childs.
    pub fn new(children: [Box<dyn Widget>; N]) -> Box<Column<N>> {
        Box::new(Column {
            children,
        })
    }

    fn get_height_requirement(&self, requirements: Vec<WidgetSizeRequirement>) -> WidgetSizeRequirement {
        let mut requirement = WidgetSizeRequirement::None;

        for child_requirement in requirements.into_iter() {
            requirement = requirement & child_requirement;
        }

        requirement
    }
    
    fn get_width_requirement(&self, requirements: Vec<WidgetSizeRequirement>) -> WidgetSizeRequirement {
        let mut requirement = WidgetSizeRequirement::None;

        for child_requirement in requirements.into_iter() {
            requirement = requirement | child_requirement;
        }

        requirement
    }

}

impl<const N: usize> Widget for Column<N> {
    fn draw(&self, canvas: &mut crate::drawing::canvas::Canvas, rect: softbuffer::Rect) {
        let mut height_requirements = [WidgetSizeRequirement::None; N];
        for (i, child) in self.children.iter().enumerate() {
            height_requirements[i] = child.min_space_requirements().1;
        }
        let heights = WidgetSizeRequirement::distribute_available_size(height_requirements, rect.height);
        let mut offset = 0;
        for (child, height) in self.children.iter().zip(heights.into_iter()) {
            match NonZeroU32::new(height) {
                Some(height) => child.draw(canvas, softbuffer::Rect {
                    x: rect.x,
                    y: rect.y + offset,
                    width: rect.width,
                    height,
                }),
                None => {},
            }
            offset += height;
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

    fn handle_event(&mut self, event: InputEvent, rect: softbuffer::Rect) -> EventResponse {
        let mut height_requirements = [WidgetSizeRequirement::None; N];
        for (i, child) in self.children.iter().enumerate() {
            height_requirements[i] = child.min_space_requirements().1;
        }
        let heights = WidgetSizeRequirement::distribute_available_size(height_requirements, rect.height);
        let mut offset = 0;
        let mut result = EventResponse::NONE;
        for (child, height) in self.children.iter_mut().zip(heights.into_iter()) {
            match NonZeroU32::new(height) {
                Some(height) => result = result | child.handle_event(event.clone(), softbuffer::Rect {
                    x: rect.x,
                    y: rect.y + offset,
                    width: rect.width,
                    height,
                }),
                None => {},
            }
            offset += height;
        }
        result
    }
}