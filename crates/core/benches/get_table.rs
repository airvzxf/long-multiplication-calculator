//! Benchmarks of the table generator and the size estimate.
//!
//! Run with `cargo bench -p long-multiplication-core`. Not part of CI:
//! timings on shared runners are too noisy to gate on. Compare runs
//! on the same machine:
//! `cargo bench -p long-multiplication-core --bench get_table -- --save-baseline before`,
//! then the same command with `--baseline before`.

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use long_multiplication_core::{estimate_table_bytes, get_table};
use std::hint::black_box;

/// Digits that repeat `0123456789`, so every size has the same mix.
fn digits(len: usize) -> String {
    (0..len).map(|i| char::from(b'0' + (i % 10) as u8)).collect()
}

fn bench_get_table(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_table");
    let cases = [
        ("13597x8642", "13597".to_owned(), "8642".to_owned()),
        ("mixed 50 digits", digits(50), digits(50)),
        ("mixed 200 digits", digits(200), digits(200)),
        // Nines need one `Sub n.` section per digit: the worst case.
        ("nines 50 digits", "9".repeat(50), "9".repeat(50)),
    ];
    for (name, a, b) in &cases {
        group.throughput(Throughput::Bytes(get_table(a, b).len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(name), &(a, b), |bench, (a, b)| {
            bench.iter(|| get_table(black_box(a), black_box(b)));
        });
    }
    group.finish();
}

fn bench_estimate(c: &mut Criterion) {
    let nines = "9".repeat(1000);
    c.bench_function("estimate_table_bytes/nines 1000 digits", |bench| {
        bench.iter(|| estimate_table_bytes(black_box(&nines), black_box(&nines)));
    });
}

criterion_group!(benches, bench_get_table, bench_estimate);
criterion_main!(benches);
