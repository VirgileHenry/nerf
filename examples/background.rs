use nerf::*;



fn main() {
    run_app::<(), _>(Background::new(
        Color::rgb(123, 175, 150),
        Empty::expand(),
    ), None).unwrap();
}

