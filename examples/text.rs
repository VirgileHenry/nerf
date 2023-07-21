use nerf::{App, Align, Alignment, Text, TextStyle, Color};



fn main() {
    let app = App::new(
        Align::new(
            Alignment::CENTER,
            Text::new(
                "Hello,\nRust! 🦀".to_string(),
                TextStyle::default().colored(Color::WHITE),
            ),
        )
    );

    app.run()
}