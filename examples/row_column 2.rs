use nerf::{App, Background, Empty, Color, Row, SizedBox, Column};



fn main() {
    let app = App::new(
        Row::new(
            [
                SizedBox::new(100, 100, Background::new(
                    Color::rgb(0, 0, 255),
                    Empty::shrink(),
                )),
                SizedBox::new(120, 120, Background::new(
                    Color::rgb(0, 50, 255),
                    Empty::shrink(),
                )),
                SizedBox::new(220, 220, Background::new(
                    Color::rgb(0, 100, 255),
                    Empty::shrink(),
                )),
                Column::new(
                    [
                        SizedBox::new(100, 100, Background::new(
                            Color::rgb(255, 0, 0),
                            Empty::shrink(),
                        )),
                        Row::new(
                            [
                                SizedBox::new(100, 100, Background::new(
                                    Color::rgb(0, 255, 0),
                                    Empty::shrink(),
                                )),
                                SizedBox::new(120, 120, Background::new(
                                    Color::rgb(0, 255, 50),
                                    Empty::shrink(),
                                )),
                                SizedBox::new(220, 220, Background::new(
                                    Color::rgb(0, 255, 100),
                                    Empty::shrink(),
                                )),
                                Empty::expand(),
                                SizedBox::new(80, 80, Background::new(
                                    Color::rgb(0, 255, 150),
                                    Empty::shrink(),
                                )),
                            ]
                        ),
                        SizedBox::new(120, 120, Background::new(
                            Color::rgb(255, 50, 0),
                            Empty::shrink(),
                        )),
                        SizedBox::new(220, 220, Background::new(
                            Color::rgb(255, 100, 0),
                            Empty::shrink(),
                        )),
                        SizedBox::new(80, 80, Background::new(
                            Color::rgb(255, 150, 0),
                            Empty::shrink(),
                        )),
                    ]
                ),
                SizedBox::new(80, 80, Background::new(
                    Color::rgb(0, 150, 255),
                    Empty::shrink(),
                )),
            ]
        )
    );

    app.run()
}

