use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use lab4::tasks::{task2, task3, task5};
use rand::Rng;

fn test_task2(c: &mut Criterion) {
    let mut group = c.benchmark_group("task2");
    group.bench_function(BenchmarkId::new("solve", "1000000"), |b| {
        b.iter(|| task2::solve(black_box(&vec![1000000.0; 1000000])))
    });
    group.bench_function(BenchmarkId::new("solve", "1000000_to_1"), |b| {
        b.iter(|| {
            task2::solve(black_box(
                &(1..=1000000).rev().map(|n| n as f64).collect::<Vec<f64>>(),
            ))
        })
    });
    group.finish();
}

fn task3_test(c: &mut Criterion) {
    let mut rng = rand::rng();
    let mut group = c.benchmark_group("task3");
    group.sample_size(20);

    let count_sets = 1000;
    let count_numbers_in_set = 375;
    let max_value = 2000000000;

    let same_numbers =
        vec![(0..count_numbers_in_set as i32).map(|i| i).collect(); count_numbers_in_set];
    let random_numbers = (0..count_sets)
        .map(|_| {
            (0..count_numbers_in_set)
                .map(|_| rng.random_range(-max_value..=max_value))
                .collect()
        })
        .collect();

    group.bench_function(BenchmarkId::new("solve", "zero_to_count"), |b| {
        b.iter(|| task3::find_largest_size_by_intersecting(black_box(&same_numbers)))
    });

    group.bench_function(BenchmarkId::new("solve", "random_numbers"), |b| {
        b.iter(|| task3::find_largest_size_by_intersecting(black_box(&random_numbers)))
    });

    group.finish();
}

fn task5_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("task5");

    group.bench_function("1-10", |b| {
        b.iter(|| task5::solve_binary(black_box(task5::Config::new(200000000, 1, 10))))
    });

    group.bench_function("5-5", |b| {
        b.iter(|| task5::solve_binary(black_box(task5::Config::new(200000000, 5, 5))))
    });

    group.bench_function("3-7", |b| {
        b.iter(|| task5::solve_binary(black_box(task5::Config::new(200000000, 3, 7))))
    });

    group.finish();
}

pub fn criterion_benchmark(c: &mut Criterion) {
    test_task2(c);
    task3_test(c);
    task5_bench(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
