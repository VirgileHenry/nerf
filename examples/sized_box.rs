use nerf::{App, Empty, Background, Color, SizedBox, Align, Alignment};



fn main() {
    let app = App::new(
        Align::new(
            Alignment::CENTER,
            SizedBox::new(
                400,
                300,
                Background::new(
                    Color::rgb(255, 0, 0),
                    SizedBox::height(
                        50,
                        Background::new(
                            Color::rgb(0, 255, 0),
                            Empty::shrink(),
                        )
                    ),
                )
            )
        )
    );

    app.run()
}

