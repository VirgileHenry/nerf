use nerf::*;



fn main() {
    run_app::<(), _>(Align::new(
        Alignment::CENTER,
        SizedBox::new(
            400,
            300,
            Background::new(
                Color::rgb(255, 0, 0),
                SizedBox::height(
                    50,
                    Background::new(
                        Color::rgb(0, 255, 0),
                        Empty::shrink(),
                    )
                ),
            )
        )
    ), None).unwrap();
}

