use criterion::{black_box, criterion_group, criterion_main, Criterion};
use worp_dice::Dice;

fn criterion_benchmark(c: &mut Criterion) {
    let mut dice = Dice::default();

    c.bench_function("loop", |b| {
        b.iter(|| {
            dice.run_script(black_box("let mut x = 0; while x < 100000 { x += 1; }"))
                .unwrap()
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
