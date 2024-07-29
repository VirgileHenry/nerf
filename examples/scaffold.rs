use nerf::*;



fn main() {
    run_app::<(), _>(Scaffold::new(
        ScreenSide::Top,
        SizedBox::height(
            100,
            Background::new(
                Color::rgb(0, 0, 255),
                Empty::expand(),
            ),
        ),
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
            )
        )
    ), None).unwrap();
}

