
use crate::{
    app::event::AppEvent, utils::nonable::Nonable, Rect, Widget
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Idle,
    Hovered,
    Pressed,
    PressedLeft,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonResponse {
    None,
    Changed {
        prev: ButtonState,
        new: ButtonState,
    },
    /// Special value for Changed where prev is pressed, new is hovered
    Clicked,
}

impl Nonable for ButtonResponse {
    fn none() -> Self { ButtonResponse::None }
    fn is_none(&self) -> bool {
        match self {
            ButtonResponse::None => true,
            _ => false,
        }
    }
}

pub struct Button<UserEvent, Child: Widget<UserEvent>> {
    _m: core::marker::PhantomData<UserEvent>,
    child: Child,
    state: ButtonState,
}


impl<UserEvent, Child: Widget<UserEvent>> Button<UserEvent, Child> {
    pub fn new(child: Child) -> Self {
        Button {
            _m: core::marker::PhantomData,
            child,
            state: ButtonState::Idle,
        }
    }
}

impl<UserEvent, Child: Widget<UserEvent>> Widget<UserEvent> for Button<UserEvent, Child> {
    type EventResponse = (ButtonResponse, Child::EventResponse);
    fn draw(&self, canvas: &mut crate::drawing::canvas::Canvas, rect: Rect) {
        self.child.draw(canvas, rect);
    }

    fn min_space_requirements(&self) -> (crate::geometry::size_requirements::WidgetSizeRequirement, crate::geometry::size_requirements::WidgetSizeRequirement) {
        self.child.min_space_requirements()
    }

    fn handle_event(&mut self, event: &AppEvent<UserEvent>, rect: Rect) -> Self::EventResponse {
        let child_response = self.child.handle_event(event, rect);
        let own_response = match (event, self.state) {
            (AppEvent::CursorMoved { position }, ButtonState::Idle) => if position.is_in_rect(rect) {
                self.state = ButtonState::Hovered;
                ButtonResponse::Changed { prev: ButtonState::Idle, new: ButtonState::Hovered }
            } else {
                ButtonResponse::None
            },
            (AppEvent::CursorMoved { position }, ButtonState::Hovered) => if position.is_in_rect(rect) {
                ButtonResponse::None
            } else {
                self.state = ButtonState::Idle;
                ButtonResponse::Changed { prev: ButtonState::Hovered, new: ButtonState::Idle }
            },
            (AppEvent::CursorMoved { position }, ButtonState::Pressed) => if position.is_in_rect(rect) {
                ButtonResponse::None
            } else {
                self.state = ButtonState::PressedLeft;
                ButtonResponse::Changed { prev: ButtonState::Pressed, new: ButtonState::PressedLeft }
            },
            (AppEvent::CursorMoved { position }, ButtonState::PressedLeft) => if position.is_in_rect(rect) {
                self.state = ButtonState::Pressed;
                ButtonResponse::Changed { prev: ButtonState::PressedLeft, new: ButtonState::Pressed }
            } else {
                ButtonResponse::None
            },
            (AppEvent::MouseInput { state: winit::event::ElementState::Pressed, button: winit::event::MouseButton::Left }, ButtonState::Hovered) => {
                self.state = ButtonState::Pressed;
                ButtonResponse::Changed { prev: ButtonState::Hovered, new: ButtonState::Pressed }
            },
            (AppEvent::MouseInput { state: winit::event::ElementState::Released, button: winit::event::MouseButton::Left }, ButtonState::Pressed) => {
                self.state = ButtonState::Hovered;
                ButtonResponse::Clicked
            },
            (AppEvent::MouseInput { state: winit::event::ElementState::Released, button: winit::event::MouseButton::Left }, ButtonState::PressedLeft) => {
                self.state = ButtonState::Idle;
                ButtonResponse::Changed { prev: ButtonState::PressedLeft, new: ButtonState::Idle }
            },
            _ => ButtonResponse::None
        };

        (own_response, child_response)
    }
}
