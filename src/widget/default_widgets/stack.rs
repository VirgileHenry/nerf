use crate::{Widget, WidgetSizeRequirement, EventResponse};





/// A widget that stacks its children on top of each other.
/// The first child is at the bottom, the last child is at the top.
pub struct Stack<const N: usize> {
    children: [Box<dyn Widget>; N],
}

impl<const N: usize> Stack<N> {
    pub fn new(children: [Box<dyn Widget>; N]) -> Box<Stack<N>> {
        Box::new(Stack {
            children,
        })
    }
}

impl<const N: usize> Widget for Stack<N> {
    fn draw(&self, canvas: &mut crate::Canvas, rect: crate::Rect) {
        for child in self.children.iter() {
            child.draw(canvas, rect);
        }
    }

    fn min_space_requirements(&self) -> (crate::WidgetSizeRequirement, crate::WidgetSizeRequirement) {
        let mut min_width = WidgetSizeRequirement::None;
        let mut min_height = WidgetSizeRequirement::None;
        for child in self.children.iter() {
            let (child_min_width, child_min_height) = child.min_space_requirements();
            min_width = min_width | child_min_width;
            min_height = min_height | child_min_height;
        }
        (min_width, min_height)
    }

    fn handle_event(&mut self, event: crate::app::event::input_event::InputEvent, rect: crate::Rect) -> crate::app::event::event_responses::EventResponse {
        let mut response = EventResponse::NONE;
        for child in self.children.iter_mut() {
            response = response | child.handle_event(event.clone(), rect);
        }
        response
    }
}

