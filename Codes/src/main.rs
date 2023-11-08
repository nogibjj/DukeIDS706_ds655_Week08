use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::env;
use std::time::Instant;

fn factorial(n: u128) -> u128 {
    (1..=n).product()
}

fn criterion_benchmark(c: &mut Criterion) {
    let args: Vec<String> = env::args().collect();
    let num: u128 = args[1].parse().expect("Please provide a valid number");
    
    let start = Instant::now();
    let result = factorial(num);
    let duration = start.elapsed();
    
    println!("Factorial of {} is {}", num, result);
    println!("Time taken: {:?}", duration);
    
    c.bench_function("Rust Factorial", |b| {
        b.iter(|| factorial(black_box(num)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);