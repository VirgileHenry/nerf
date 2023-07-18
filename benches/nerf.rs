use std::num::NonZeroU32;

use nerf::*;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};

fn compute_widget_sizes(c: &mut Criterion) {
    let root = black_box(Column::new(
        [
            Row::new([
                SizedBox::new(120, 80, Empty::expand()), Empty::expand(),
                SizedBox::new(240, 300, Empty::expand()), Empty::expand(),
                SizedBox::new(40, 100, Empty::expand()), Empty::expand(),
                SizedBox::new(10, 80, Empty::expand()), Empty::expand(),
                SizedBox::new(200, 5, Empty::expand()), Empty::expand(),
            ]),
            Row::new([
                SizedBox::new(120, 80, Empty::expand()), Empty::expand(),
                SizedBox::new(240, 300, Empty::expand()), Empty::expand(),
                SizedBox::new(40, 100, Empty::expand()), Empty::expand(),
                SizedBox::new(10, 80, Empty::expand()), Empty::expand(),
                SizedBox::new(200, 5, Empty::expand()), Empty::expand(),
            ]),
            Row::new([
                SizedBox::new(120, 80, Empty::expand()), Empty::expand(),
                SizedBox::new(240, 300, Empty::expand()), Empty::expand(),
                SizedBox::new(40, 100, Empty::expand()), Empty::expand(),
                SizedBox::new(10, 80, Empty::expand()), Empty::expand(),
                SizedBox::new(200, 5, Empty::expand()), Empty::expand(),
            ]),
            Row::new([
                SizedBox::new(120, 80, Empty::expand()), Empty::expand(),
                SizedBox::new(240, 300, Empty::expand()), Empty::expand(),
                SizedBox::new(40, 100, Empty::expand()), Empty::expand(),
                SizedBox::new(10, 80, Empty::expand()), Empty::expand(),
                SizedBox::new(200, 5, Empty::expand()), Empty::expand(),
            ]),
            Row::new([
                SizedBox::new(120, 80, Empty::expand()), Empty::expand(),
                SizedBox::new(240, 300, Empty::expand()), Empty::expand(),
                SizedBox::new(40, 100, Empty::expand()), Empty::expand(),
                SizedBox::new(10, 80, Empty::expand()), Empty::expand(),
                SizedBox::new(200, 5, Empty::expand()), Empty::expand(),
            ]),
            Row::new([
                SizedBox::new(120, 80, Empty::expand()), Empty::expand(),
                SizedBox::new(240, 300, Empty::expand()), Empty::expand(),
                SizedBox::new(40, 100, Empty::expand()), Empty::expand(),
                SizedBox::new(10, 80, Empty::expand()), Empty::expand(),
                SizedBox::new(200, 5, Empty::expand()), Empty::expand(),
            ]),
            Row::new([
                SizedBox::new(120, 80, Empty::expand()), Empty::expand(),
                SizedBox::new(240, 300, Empty::expand()), Empty::expand(),
                SizedBox::new(40, 100, Empty::expand()), Empty::expand(),
                SizedBox::new(10, 80, Empty::expand()), Empty::expand(),
                SizedBox::new(200, 5, Empty::expand()), Empty::expand(),
            ]),
            Row::new([
                SizedBox::new(120, 80, Empty::expand()), Empty::expand(),
                SizedBox::new(240, 300, Empty::expand()), Empty::expand(),
                SizedBox::new(40, 100, Empty::expand()), Empty::expand(),
                SizedBox::new(10, 80, Empty::expand()), Empty::expand(),
                SizedBox::new(200, 5, Empty::expand()), Empty::expand(),
            ]),
            Row::new([
                SizedBox::new(120, 80, Empty::expand()), Empty::expand(),
                SizedBox::new(240, 300, Empty::expand()), Empty::expand(),
                SizedBox::new(40, 100, Empty::expand()), Empty::expand(),
                SizedBox::new(10, 80, Empty::expand()), Empty::expand(),
                SizedBox::new(200, 5, Empty::expand()), Empty::expand(),
            ]),
            Row::new([
                SizedBox::new(120, 80, Empty::expand()), Empty::expand(),
                SizedBox::new(240, 300, Empty::expand()), Empty::expand(),
                SizedBox::new(40, 100, Empty::expand()), Empty::expand(),
                SizedBox::new(10, 80, Empty::expand()), Empty::expand(),
                SizedBox::new(200, 5, Empty::expand()), Empty::expand(),
            ]),
        ]
    ));
    let rect = softbuffer::Rect {
        x: 0,
        y: 0,
        width: unsafe { NonZeroU32::new_unchecked(1920) },
        height: unsafe { NonZeroU32::new_unchecked(1080) },
    };
    c.bench_function("Compute size for 100 widgets :", |b| {
        b.iter(|| {
            // this is SUPER UNSAFE: we are building a fake canvas to pass to the draw function.
            // however, the widgets won't actually draw anything, so it's fine.
            let fake_canvas = [0u8; 8];
            let canvas = unsafe { std::mem::transmute(fake_canvas) };
            black_box(root.draw(canvas, rect));
        })
    });
}



criterion_group!(
    benches,
    compute_widget_sizes,
);

criterion_main!(benches);
