mod compute;

use compute::data_locality::bench_seq_vs_rand;
use compute::fibonacci::fibonacci_bench;
use criterion::{criterion_group, criterion_main};

// Define benchmark groups
criterion_group!(basic, fibonacci_bench);
criterion_group!(data_locality, bench_seq_vs_rand);

// Register all benchmark groups
criterion_main!(basic, data_locality,);
