use criterion::{AxisScale, BenchmarkId, Criterion, PlotConfiguration, Throughput};
use playground::data::random::{make_deterministic_rng, random_vec};
use rand::prelude::SliceRandom;
use std::hint::black_box;

// ----------------------------------------------------------------------
// Benchmark drivers
// ----------------------------------------------------------------------

const KI: usize = 1024;
const MI: usize = 1024 * 1024;

pub fn bench_seq_vs_rand(c: &mut Criterion) {
    let mut rng = make_deterministic_rng();

    let mut group = c.benchmark_group("agg_sum_seq_vs_rand");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for item_count in [1 * KI, 4 * KI, 16 * KI, 64 * KI, 256 * KI, 1 * MI, 4 * MI].iter() {
        let input_data = random_vec::<i32>(&mut rng, *item_count);
        let seq_indices: Vec<usize> = (0..*item_count).collect();
        let mut rand_indices: Vec<usize> = (0..*item_count).collect();
        rand_indices.shuffle(&mut rng);

        group.throughput(Throughput::ElementsAndBytes {
            elements: *item_count as u64,
            bytes: *item_count as u64 * size_of::<i32>() as u64,
        });

        group.bench_function(BenchmarkId::new("seq", item_count), |b| {
            b.iter(|| {
                compute_sum(&input_data, &seq_indices);
            })
        });
        group.bench_function(BenchmarkId::new("rand", item_count), |b| {
            b.iter(|| {
                compute_sum(&input_data, &rand_indices);
            })
        });
    }

    group.finish();
}

fn compute_sum(input_data: &[i32], indices: &[usize]) {
    let mut sum = 0;
    for idx in indices {
        sum += input_data[*idx];
    }
    let _ = black_box(sum);
}
