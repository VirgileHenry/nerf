use nerf::*;



fn main() {
    run_app(Align::new(
        Alignment::CENTER,
        Text::<()>::new(
            "HELLO,\nRust! 🦀".to_string(),
            TextStyle::default()
                .colored(Color::WHITE),
        ),
    ), None).unwrap();
}