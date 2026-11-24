//! Benchmarks of the table generator and the size estimate.
//!
//! Run with `cargo bench -p long-multiplication-core`. Not part of CI:
//! timings on shared runners are too noisy to gate on. Compare runs
//! on the same machine:
//! `cargo bench -p long-multiplication-core --bench get_table -- --save-baseline before`,
//! then the same command with `--baseline before`.

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use long_multiplication_core::{LongMultiplication, estimate_table_bytes, get_table};
use std::fmt;
use std::hint::black_box;

/// A writer like the CLI's `Sink` sending to `/dev/null`: it gathers
/// the text in 256 KiB blocks and drops each full block.
struct Blocks {
    buffer: Vec<u8>,
    written: usize,
}

impl Blocks {
    const BLOCK: usize = 256 * 1024;

    fn new() -> Self {
        Self { buffer: Vec::with_capacity(Self::BLOCK), written: 0 }
    }
}

impl fmt::Write for Blocks {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        if self.buffer.len() + text.len() > Self::BLOCK {
            self.written += black_box(&self.buffer).len();
            self.buffer.clear();
        }
        self.buffer.extend_from_slice(text.as_bytes());
        Ok(())
    }
}

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

fn bench_streaming(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_table in 256 KiB blocks");
    let cases = [
        ("mixed 200 digits", digits(200), digits(200)),
        ("nines 50 digits", "9".repeat(50), "9".repeat(50)),
        ("nines 100 digits", "9".repeat(100), "9".repeat(100)),
    ];
    for (name, a, b) in &cases {
        group.throughput(Throughput::Bytes(get_table(a, b).len() as u64));
        let process = LongMultiplication::new(a, b).expect("digits");
        group.bench_with_input(BenchmarkId::from_parameter(name), &process, |bench, process| {
            bench.iter(|| {
                let mut out = Blocks::new();
                process.write_table(&mut out).expect("never fails");
                out.written + out.buffer.len()
            });
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

criterion_group!(benches, bench_get_table, bench_streaming, bench_estimate);
criterion_main!(benches);
