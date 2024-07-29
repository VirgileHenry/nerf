use nerf::*;



fn main() {
    run_app::<(), _>(Padder::new(
        PaddType::ALL,
        20,
        Background::new(
            Color::rgb(220, 255, 230),
            Empty::expand(),
        ),
    ), None).unwrap();
}

