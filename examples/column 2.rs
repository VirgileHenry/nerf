use nerf::{App, Background, Empty, Color, Column, SizedBox};



fn main() {
    let app = App::new(
        Column::new(
            [
                SizedBox::new(100, 100, Background::new(
                    Color::rgb(0, 0, 255),
                    Empty::shrink(),
                )),
                SizedBox::new(120, 120, Background::new(
                    Color::rgb(0, 255, 255),
                    Empty::shrink(),
                )),
                SizedBox::new(220, 220, Background::new(
                    Color::rgb(0, 255, 0),
                    Empty::shrink(),
                )),
                Empty::expand(),
                SizedBox::new(80, 80, Background::new(
                    Color::rgb(255, 0, 255),
                    Empty::shrink(),
                )),
            ]
        )
    );

    app.run()
}

