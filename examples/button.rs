
use std::fmt::Debug;

use nerf::*;


pub struct EventPrinter<UserEvent, Child: Widget<UserEvent>> {
    _m: core::marker::PhantomData<UserEvent>,
    child: Child
}

impl<UserEvent, Child: Widget<UserEvent>> Widget<UserEvent> for EventPrinter<UserEvent, Child>
where Child::EventResponse: Debug
{
    type EventResponse = Child::EventResponse;
    fn draw(&self, canvas: &mut Canvas, rect: Rect) {
        self.child.draw(canvas, rect)
    }
    fn min_space_requirements(&self) -> (WidgetSizeRequirement, WidgetSizeRequirement) {
        self.child.min_space_requirements()
    }
    fn handle_event(&mut self, event: &AppEvent<UserEvent>, rect: nerf::Rect) -> Self::EventResponse {
        let response = self.child.handle_event(event, rect);
        if !response.is_none() {   
            println!("Received response: {response:?}");
        }
        response
    }
}


fn main() {
    run_app::<(), _>(Align::new(
        Alignment::CENTER,
        EventPrinter {
            _m: core::marker::PhantomData,
            child: Button::new(
                SizedBox::new(
                    200, 80,
                    Background::new(
                        Color::rgb(200, 220, 255),
                        Empty::expand()
                    )
                )
            )
        }
    ), None).unwrap();
}
