
use nerf::{App, Scaffold, Column, SizedBox, Background, Empty, Color, Padder, PaddType};




fn main() {
    let app = App::new(
        Scaffold::new(
            nerf::ScreenSide::Left,
            Background::new(
                Color::rgb(100, 100, 100),
                Padder::new(
                    PaddType::ALL,
                    20,
                    Column::new([
                        SizedBox::new(
                            200, 80,
                            Background::new(Color::rgb(0, 0, 255), Empty::expand())
                        ),
                        SizedBox::height(20, Empty::shrink()),
                        SizedBox::new(
                            200, 80,
                            Background::new(Color::rgb(0, 0, 255), Empty::expand())
                        ),
                        SizedBox::height(20, Empty::shrink()),
                        SizedBox::new(
                            200, 80,
                            Background::new(Color::rgb(0, 0, 255), Empty::expand())
                        ),
                        SizedBox::height(20, Empty::shrink()),
                        SizedBox::new(
                            200, 80,
                            Background::new(Color::rgb(0, 0, 255), Empty::expand())
                        ),
                        SizedBox::width(1, Empty::expand()),
                        SizedBox::height(20, Empty::shrink()),
                        SizedBox::new(
                            200, 80,
                            Background::new(Color::rgb(0, 0, 255), Empty::expand())
                        ),
                    ])
                )
            ),
            Background::new(
                Color::rgb(150, 150, 150),
                Empty::expand()
            )
        )
    );

    app.run()
}