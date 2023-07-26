use nerf::{App, Background, Empty, Color, Row, SizedBox};



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
                Empty::expand(),
                SizedBox::new(80, 80, Background::new(
                    Color::rgb(0, 150, 255),
                    Empty::shrink(),
                )),
            ]
        )
    );

    app.run()
}

