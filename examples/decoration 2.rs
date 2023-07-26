use nerf::*;


fn main() {
    let app = App::new(
        Padder::new(
            PaddType::ALL,
            100,
            DecoratedBackground::new(
                Some(Color::rgb(200, 255, 200)),
                Some(Color::rgb(20, 100, 20)),
                BorderType::ROUND_ALL,
                30,
                50,
                Empty::expand(),
            ),
        )
    );

 
    app.run()
}

