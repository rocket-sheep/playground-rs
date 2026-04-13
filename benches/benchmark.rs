mod compute;

use compute::fibonacci::fibonacci_bench;
use criterion::{criterion_group, criterion_main};

// Define benchmark groups
criterion_group!(basic, fibonacci_bench);

// Register all benchmark groups
criterion_main!(
    basic,
);
