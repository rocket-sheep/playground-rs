use criterion::Criterion;
use playground::compute::fibonacci::fib;
use std::hint::black_box;

pub fn fibonacci_bench(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fib(black_box(20))));
}
