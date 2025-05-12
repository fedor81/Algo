use criterion::{Criterion, black_box, criterion_group, criterion_main};
use lab8::tasks::task2::can_segment_text;
use std::collections::HashSet;

fn bench_task2(c: &mut Criterion) {
    let mut group = c.benchmark_group("task2");
    let s = black_box("ab".repeat(50000));
    let words = black_box((1..=50).into_iter().map(|i| "ab".repeat(i)).collect::<HashSet<_>>());

    group.bench_function("abababa", |b| b.iter(|| can_segment_text(&s, &words)));

    let s = black_box("a".repeat(100000));
    let words = black_box((1..=100).into_iter().map(|i| "a".repeat(i)).collect::<HashSet<_>>());
    group.bench_function("aaaaaaa", |b| b.iter(|| can_segment_text(&s, &words)));
}

pub fn criterion_benchmark(c: &mut Criterion) {
    bench_task2(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
