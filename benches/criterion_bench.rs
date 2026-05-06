use broken_app::{algo, leak_buffer, normalize, sum_even, average_positive};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::Path;

fn bench_sum_even(c: &mut Criterion) {
    let data: Vec<i64> = (0..50_000).collect();
    c.bench_function("sum_even/50k", |b| b.iter(|| sum_even(black_box(&data))));
}

fn bench_leak_buffer(c: &mut Criterion) {
    let data: Vec<u8> = (0..100_000).map(|x| (x % 256) as u8).collect();
    c.bench_function("leak_buffer/100k", |b| b.iter(|| leak_buffer(black_box(&data))));
}

fn bench_average_positive(c: &mut Criterion) {
    let data: Vec<i64> = (0..50_000).map(|x| if x % 2 == 0 { x } else { -x }).collect();
    c.bench_function("average_positive/50k", |b| b.iter(|| average_positive(black_box(&data))));
}

fn bench_normalize(c: &mut Criterion) {
    let text = "Hello World   This   Is   A   Test   String ".repeat(1000);
    c.bench_function("normalize/long", |b| b.iter(|| normalize(black_box(&text))));
}

fn bench_slow_dedup(c: &mut Criterion) {
    let data: Vec<u64> = (0..5_000).flat_map(|n| [n, n]).collect();
    c.bench_function("slow_dedup/10k", |b| b.iter(|| algo::slow_dedup(black_box(&data))));
}

fn bench_slow_fib(c: &mut Criterion) {
    // n=25 ~13M вызовов (без оптимизаций займет ~10-20 сек на итерацию)
    c.bench_function("slow_fib/25", |b| b.iter(|| algo::slow_fib(black_box(25))));
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .output_directory(Path::new("artifacts/criterion_output"));
    targets = bench_sum_even, bench_leak_buffer, bench_average_positive, bench_normalize, bench_slow_dedup, bench_slow_fib
}

criterion_main!(benches);
