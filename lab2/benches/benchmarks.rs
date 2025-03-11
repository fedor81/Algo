use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use lab2::{
    modules::{merge_sort::merge_sort, quick_sort::quick_sort_non_recursive},
    tasks::task2::Competitor,
};
use rand::Rng;

fn task2_bench(c: &mut Criterion) {
    let mut competitors = Vec::new();
    let mut competitors_random = Vec::new();
    let mut rng = rand::rng();

    for i in 0..1000000 {
        competitors.push(Competitor::new(format!("competitor_{}", i), i, i));
        competitors_random.push(Competitor::new(
            format!("competitor_{}", i),
            rng.random_range(0..1000000),
            rng.random_range(0..1000000),
        ));
    }

    let mut group = c.benchmark_group("task2");

    group.sample_size(10);
    group.bench_function("quick_sort", |b| {
        b.iter(|| quick_sort_non_recursive(black_box(&mut competitors[0..25000]), true));
    });

    group.sample_size(100);
    group.bench_function("quick_sort_random_state", |b| {
        b.iter(|| quick_sort_non_recursive(black_box(&mut competitors_random.clone()), true));
    });

    group.bench_function("merge_sort_random", |b| {
        b.iter(|| merge_sort(black_box(&mut competitors_random.clone()), true));
    });

    group.finish();
}

fn task3_bench(c: &mut Criterion) {
    let numbers = vec![1000000; 1000000];
    let mut group = c.benchmark_group("task3");

    group.bench_function(BenchmarkId::new("solve", "bench1"), |b| {
        b.iter(|| lab2::tasks::task3::solve(black_box(numbers.clone())));
    });
    group.finish();
}

fn task5_bench(c: &mut Criterion) {
    let mut rng = rand::rng();
    let mut points = Vec::new();
    let half_count = 710;
    const DISTANCE: i32 = 10000;

    for i in 0..half_count {
        for j in 0..half_count {
            points.push(lab2::tasks::task5::Point::new(
                rng.random_range(-DISTANCE..DISTANCE),
                rng.random_range(-DISTANCE..DISTANCE),
            ));
        }
    }

    let mut group = c.benchmark_group("task5");

    group.bench_function(BenchmarkId::new("random_state", "bench1"), |b| {
        b.iter(|| lab2::tasks::task5::solve(black_box(&mut points.clone())));
    });
    group.finish();
}

pub fn criterion_benchmark(c: &mut Criterion) {
    task2_bench(c);
    task3_bench(c);
    task5_bench(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
