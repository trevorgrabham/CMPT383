use criterion::{black_box, criterion_group, criterion_main, Criterion};
use exer8::hailstone::{hailstone_sequence_append, hailstone_sequence_prealloc};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("hailstone_sequence_append_7", |b| {
        b.iter(|| hailstone_sequence_append(black_box(7)))
    });
    c.bench_function("hailstone_sequence_prealloc_7", |b| {
        b.iter(|| hailstone_sequence_prealloc(black_box(7)))
    });
    c.bench_function("hailstone_sequence_append_162964", |b| {
        b.iter(|| hailstone_sequence_append(black_box(162964)))
    });
    c.bench_function("hailstone_sequence_prealloc_162964", |b| {
        b.iter(|| hailstone_sequence_prealloc(black_box(162964)))
    });
    c.bench_function("hailstone_sequence_append_686901248", |b| {
        b.iter(|| hailstone_sequence_append(black_box(686901248)))
    });
    c.bench_function("hailstone_sequence_prealloc_686901248", |b| {
        b.iter(|| hailstone_sequence_prealloc(black_box(686901248)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
