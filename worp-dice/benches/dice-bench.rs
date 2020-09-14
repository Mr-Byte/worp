use criterion::{black_box, criterion_group, criterion_main, Criterion};
use worp_dice::Dice;

fn loop_in_place_addition(criterion: &mut Criterion) {
    let mut dice = Dice::default();

    criterion.bench_function("in-place-addition", |bencher| {
        bencher.iter(|| {
            dice.run_script(black_box("let mut x = 0; while x < 1000 { x += 1; }"))
                .unwrap()
        })
    });
}

fn loop_addition_with_assignment(criterion: &mut Criterion) {
    let mut dice = Dice::default();

    criterion.bench_function("addition-with-assignment", |bencher| {
        bencher.iter(|| {
            dice.run_script(black_box("let mut x = 0; while x < 1000 { x = x + 1; }"))
                .unwrap()
        })
    });
}

fn loop_function_call(criterion: &mut Criterion) {
    let mut dice = Dice::default();

    criterion.bench_function("function-call", |bencher| {
        bencher.iter(|| {
            dice.run_script(black_box(
                "fn one() { 1 } let mut n = 0; while n < 1000 { n += one(); }",
            ))
            .unwrap()
        })
    });
}

criterion_group!(
    loops,
    loop_in_place_addition,
    loop_addition_with_assignment,
    loop_function_call
);
criterion_main!(loops);
