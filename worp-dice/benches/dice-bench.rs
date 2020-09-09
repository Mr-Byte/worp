use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use worp_dice::Dice;

fn criterion_benchmark(criterion: &mut Criterion) {
    let mut dice = Dice::default();
    let mut group = criterion.benchmark_group("core");
    group.measurement_time(Duration::from_millis(100000));
    group.sample_size(1000);

    group.bench_function("loop - in place addition", |bencher| {
        bencher.iter(|| {
            dice.run_script(black_box("let mut x = 0; while x < 1000000 { x += 1; }"))
                .unwrap()
        })
    });

    group.bench_function("loop - addition with assignment", |bencher| {
        bencher.iter(|| {
            dice.run_script(black_box("let mut x = 0; while x < 1000000 { x = x + 1; }"))
                .unwrap()
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
