
use crate::{Widget, app::event::input_event::InputEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ButtonState {
    Idle,
    Hovered,
    Pressed,
    PressedLeft,
}

pub struct Button {
    child: Box<dyn Widget>,
    state: ButtonState,
}


impl Button {
    pub fn new(child: Box<dyn Widget>) -> Box<Button> {
        Box::new(Button {
            child,
            state: ButtonState::Idle,
        })
    }
}

impl Widget for Button {
    fn draw(&self, canvas: &mut crate::drawing::canvas::Canvas, rect: softbuffer::Rect) {
        self.child.draw(canvas, rect);
    }

    fn min_space_requirements(&self) -> (crate::geometry::size_requirements::WidgetSizeRequirement, crate::geometry::size_requirements::WidgetSizeRequirement) {
        self.child.min_space_requirements()
    }

    fn handle_event(&mut self, event: InputEvent, rect: softbuffer::Rect) -> bool {
        match (event, self.state) {
            (InputEvent::CursorMoved { position }, ButtonState::Idle) => if position.is_in_rect(rect) {
                self.state = ButtonState::Hovered;
                true
            } else {
                false
            },
            (InputEvent::CursorMoved { position }, ButtonState::Hovered) => if position.is_in_rect(rect) {
                false
            } else {
                self.state = ButtonState::Idle;
                true
            },
            (InputEvent::CursorMoved { position }, ButtonState::Pressed) => if position.is_in_rect(rect) {
                false
            } else {
                self.state = ButtonState::PressedLeft;
                true
            },
            (InputEvent::CursorMoved { position }, ButtonState::PressedLeft) => if position.is_in_rect(rect) {
                self.state = ButtonState::Hovered;
                true
            } else {
                false
            },
            (InputEvent::MouseInput { state, button }, ButtonState::Hovered) => if state == winit::event::ElementState::Pressed && button == winit::event::MouseButton::Left {
                self.state = ButtonState::Pressed;
                true
            } else {
                false
            },
            (InputEvent::MouseInput { state, button }, ButtonState::Pressed) => if state == winit::event::ElementState::Released && button == winit::event::MouseButton::Left {
                self.state = ButtonState::Hovered;
                true
            } else {
                false
            },
            (InputEvent::MouseInput { state, button }, ButtonState::PressedLeft) => if state == winit::event::ElementState::Released && button == winit::event::MouseButton::Left {
                self.state = ButtonState::Idle;
                true
            } else {
                false
            },
            _ => false,

        }
    }
}
