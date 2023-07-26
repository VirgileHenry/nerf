

use nerf::{App, Background, Color, SizedBox, Align, Alignment, Button, Text, Widget, EventResponse, InputEvent, WidgetSizeRequirement, TextStyle, FontStyle, FontWeight};



fn main() {
    let app = App::new(
        Align::new(
            Alignment::CENTER,
            Box::new(SwitchingButton::new())
        )
    );

    app.run()
}

struct SwitchingButton {
    button1: Box<Button>,
    button2: Box<Button>,
    button3: Box<Button>,
    state: u8,
}

impl SwitchingButton {
    fn new() -> SwitchingButton {
        SwitchingButton {
            button1: Button::new(
                SizedBox::new(
                    250, 80,
                    Background::new(Color::rgb(200, 200, 255), Text::new(
                        "Luck Fogan".to_string(),
                        TextStyle::default()
                            .sized(30.0)
                            .styled(FontStyle::Italic)
                    ))
                )
            ),
            button2: Button::new(
                SizedBox::new(
                    250, 80,
                    Background::new(Color::rgb(200, 255, 200), Text::new(
                        "Juck Forge".to_string(),
                        TextStyle::default()
                            .sized(30.0)
                            .weighted(FontWeight::BOLD)
                    ))
                )
            ),
            button3: Button::new(
                SizedBox::new(
                    250, 80,
                    Background::new(Color::rgb(255, 200, 200), Text::new(
                        "Nuck Fils".to_string(),
                        TextStyle::default()
                            .sized(30.0)
                    ))
                )
            ),
            state: 0,
        }
    }
}

impl Widget for SwitchingButton {
    fn draw(&self, canvas: &mut nerf::Canvas, rect: softbuffer::Rect) {
        match self.state {
            0 => self.button1.draw(canvas, rect),
            1 => self.button2.draw(canvas, rect),
            2 => self.button3.draw(canvas, rect),
            _ => {},
        }
    }

    fn min_space_requirements(&self) -> (nerf::WidgetSizeRequirement, nerf::WidgetSizeRequirement) {
        match self.state {
            0 => self.button1.min_space_requirements(),
            1 => self.button2.min_space_requirements(),
            2 => self.button3.min_space_requirements(),
            _ => (
                WidgetSizeRequirement::None,
                WidgetSizeRequirement::None,
            ),
        }
    }

    fn handle_event(&mut self, event: InputEvent, rect: softbuffer::Rect) -> EventResponse {
        let mut response = match self.state {
            0 => self.button1.handle_event(event, rect),
            1 => self.button2.handle_event(event, rect),
            2 => self.button3.handle_event(event, rect),
            _ => EventResponse::NONE,
        };
        if response.contains(EventResponse::CALLBACK) {
            self.state = (self.state + 1) % 3;
            response = response | EventResponse::REDRAW_REQUEST;
            response = response & !EventResponse::CALLBACK;
        }
        response
    }
}