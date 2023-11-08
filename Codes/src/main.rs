use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn factorial(n: u128) -> u128 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let num: u128 = args[1].parse().expect("Please provide a valid number");
    
    let start = Instant::now();
    let result = factorial(num);
    let duration = start.elapsed();
    let time_taken = duration.as_secs_f64();
    
    let mut file = File::create("../../Resources/Summary.md").expect("Unable to create file");
    write!(file, "Rust factorial: {}\n", result).expect("Unable to write to file");
    write!(file, "Rust Time taken for factorial: {} seconds\n", time_taken).expect("Unable to write to file");
    
    println!("Rust factorial: {}", result);
    println!("Rust Time taken for factorial: {} seconds", time_taken);
}