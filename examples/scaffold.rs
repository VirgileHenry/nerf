use nerf::{App, Background, Empty, Color, Scaffold, ScreenSide, SizedBox, Center, Padder, PaddType};



fn main() {
    let app = App::new(
        Scaffold::new(
            ScreenSide::Top,
            SizedBox::height(
                100,
                Background::new(
                    Color::rgb(0, 0, 255),
                    Empty::expand(),
                ),
            ),
            Center::new(
                SizedBox::new(
                    500,
                    400,
                    Background::new(
                        Color::rgb(255, 0, 0),
                        Padder::new(
                            PaddType::ALL,
                            10,
                            Scaffold::new(
                                ScreenSide::Left,
                                SizedBox::width(
                                    100,
                                    Background::new(
                                        Color::rgb(40, 40, 0),
                                        Empty::shrink())),
                                Background::new(
                                    Color::rgb(200, 0, 100),
                                    Empty::expand()
                                ),
                            ),
                        ),
                    ),
                )
            )
        ),
    );

    app.run()
}

