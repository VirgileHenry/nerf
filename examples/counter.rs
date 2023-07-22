use std::num::NonZeroU32;
use nerf::*;

fn main() {
    // Create a new app, specifying the root widget.
    let app = App::new(
        Align::new(
            Alignment::CENTER,
            Center::new(Counter::new()),
        )
    );

    // Run the app.
    app.run()
}

// Create a widget to represent our counter.
// Any app data arr wrapped into widgets. User widgets "bound" default widgets to create new ones,
// and implement their own logic.
struct Counter {
    // state of the widget
    count: u32,
    // the button that will throw callbacks
    button: Box<dyn Widget>,
    // the display text. We keep it here, to be able to reference and mofify it.
    text: Box<Text>,
}

impl Counter {
    pub fn new() -> Box<Counter> {
        Box::new(
            Counter {
                count: 0,
                button: Background::new(
                    Color::rgb(200, 255, 200),
                    Button::new(Empty::expand()),
                ),
                text: Text::new(
                    "0".to_string(),
                    TextStyle::default()
                        .sized(30.0)
                        .styled(FontStyle::Italic)
                ),
            }
        )
    }
}

// This is the only required thing by the app: implement the widget trait.
impl Widget for Counter {
    // the draw function is straight forward. For most custom widgets, draw your children.
    fn draw(&self, canvas: &mut Canvas, rect: softbuffer::Rect) {
        // we could not draw the button, but here we put the background in in there too. so let's draw it.
        self.button.draw(canvas, rect);
        // then, draw the text.
        self.text.draw(canvas, rect);
    }

    // The handle event func implement widget response.
    fn handle_event(&mut self, event: InputEvent, rect: softbuffer::Rect) -> EventResponse {
        // here, give the event to the button. It will return a response.
        let result = self.button.handle_event(event, rect);
        // if the response contains a callback, it means the button have been pressed.
        if result.contains(EventResponse::CALLBACK) {
            // in that case, increment the counnter and update the text.
            self.count += 1;
            self.text.set_text(self.count.to_string())
        }
        // remove the callback flag, and add a redraw request flag.
        // it is important to ask for a redraw when we change our state, as the engine will not redraw otherwise.
        // however here, it is likely that the button will ask for a redraw too, so we could remove this line.
        let result = result | EventResponse::REDRAW_REQUEST | !EventResponse::CALLBACK;
        // finally, return the response.
        // the response MUST be returned to the parent widget, so it can handle it too.
        result
    }

    // finaly, the space requirements will give hint on how to layout the widget whith his environment.
    fn min_space_requirements(&self) -> (WidgetSizeRequirement, WidgetSizeRequirement) {
        // here, we manually ask for a fixed size, like a sized box would do.
        (
            WidgetSizeRequirement::Fixed(unsafe {NonZeroU32::new_unchecked(200)}),
            WidgetSizeRequirement::Fixed(unsafe {NonZeroU32::new_unchecked(70)}),
        )
    }
}