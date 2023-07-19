use nerf::{App, Background, Empty, Color};



fn main() {
    let app = App::new(
        Background::new(
            Color::rgb(0, 255, 0),
            Empty::expand(),
        ),
    );

    app.run()
}

