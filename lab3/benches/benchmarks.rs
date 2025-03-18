use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use lab3::tasks::task10::{solve, Segment};
use rand::Rng;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::rng();
    let count: u32 = 100000;
    let max_right = 1000000000;

    let mut random_increasing_segments = Vec::with_capacity(count as usize);
    let mut increasing_segments = Vec::with_capacity(count as usize);
    let mut decreasing_segments = Vec::with_capacity(count as usize);
    let mut random_decreasing_segments = Vec::with_capacity(count as usize);

    // Нельзя использовать полностью рандомный отрезок, у которого два конца случайны
    // Так как нельзя гарантировать, что отрезки либо не пересекутся, либо будут полностью вложены
    for i in 1..=count {
        random_increasing_segments.push(Segment::new(1, rng.random_range(2..=max_right)));
        random_decreasing_segments.push(Segment::new(rng.random_range(1..max_right), max_right));
        increasing_segments.push(Segment::new(i, i * 10000));
        decreasing_segments.push(Segment::new(i * 10000 - 1, max_right));
    }

    random_increasing_segments.sort_by_key(|segment| segment.right());
    random_decreasing_segments.sort_by_key(|segment| segment.left());

    let mut points: Vec<_> = (0..count)
        .map(|_| rng.random_range(1..=max_right))
        .collect();
    points.sort();

    let mut group = c.benchmark_group("task10");
    group.bench_function("random_increasing_segments", |b| {
        b.iter(|| solve(black_box(&random_increasing_segments), black_box(&points)))
    });
    group.bench_function("random_decreasing_segments", |b| {
        b.iter(|| solve(black_box(&random_decreasing_segments), black_box(&points)))
    });
    group.bench_function("increasing_segments", |b| {
        b.iter(|| solve(black_box(&increasing_segments), black_box(&points)))
    });
    group.bench_function("decreasing_segments", |b| {
        b.iter(|| solve(black_box(&decreasing_segments), black_box(&points)))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
