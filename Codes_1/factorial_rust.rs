use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Rust factorial", |b| {
        b.iter(|| factorial(black_box(10_000_000)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);