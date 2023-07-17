use nerf::{App, Padder, Background, Empty, Color, PaddType};



fn main() {
    let app = App::new(
        Padder::new(
            PaddType::ALL,
            20,
            Background::new(
                Color::rgb(0, 255, 0),
                Empty::expand(),
            ),
        )
    );

    app.run()
}

