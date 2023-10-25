use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn factorial(n: u128) -> u128 {
    (1..=n).product()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Rust Factorial", |b| {
        b.iter(|| factorial(black_box(30)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);