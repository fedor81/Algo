use core::num;
use std::{time::Duration, u32};

use criterion::{
    black_box, criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion,
    PlotConfiguration,
};
use lab4::tasks::{task10, task2, task3, task5, task7};
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

fn task7_bench(c: &mut Criterion) {
    let mut rng = rand::rng();
    let count = 500000;
    let counts = vec![count / 4, count / 2, count];
    let probabilities = vec![0.1, 0.3, 0.5, 0.7, 0.9];

    let mut group = c.benchmark_group("task7");
    group.sample_size(10);

    for &count in &counts {
        for &probability in &probabilities {
            let (numbers, queries) = get_task7_numbers_queries(count, count, probability, &mut rng);
            group.bench_function(
                format!(
                    "solve_on_other_bigint(sum/replace: {}, count: {})",
                    probability, count
                ),
                |b| {
                    b.iter(|| {
                        task7::solve_on_other_bigint(black_box(&numbers), black_box(&queries))
                    })
                },
            );
        }
    }

    for &count in &counts {
        for &probability in &probabilities {
            let (numbers, queries) = get_task7_numbers_queries(count, count, probability, &mut rng);
            group.bench_function(
                format!(
                    "solve_on_my_bigint(sum/replace: {}, count: {})",
                    probability, count
                ),
                |b| b.iter(|| task7::solve_on_my_bigint(black_box(&numbers), black_box(&queries))),
            );
        }
    }

    group.finish();
}

fn get_task7_numbers_queries<R: rand::Rng>(
    numbers_count: usize,
    queries_count: usize,
    queries_probability: f64,
    rng: &mut R,
) -> (Vec<u32>, Vec<task7::Query>) {
    (
        (0..numbers_count)
            .into_iter()
            .map(|i| rng.random_range(0..u32::MAX))
            .collect(),
        (1..=queries_count)
            .into_iter()
            .map(|i| match rng.random_bool(queries_probability) {
                true => {
                    let bound = rng.random_range(1..(numbers_count - 1));
                    task7::Query::Sum {
                        start: rng.random_range(0..bound),
                        end: rng.random_range(bound..numbers_count),
                    }
                }
                false => task7::Query::Replace {
                    index: rng.random_range(0..numbers_count),
                    element: rng.random_range(0..u32::MAX),
                },
            })
            .collect(),
    )
}

fn task10_bench(c: &mut Criterion) {
    let mut strings = Vec::new();

    for i in 0..100000 {
        let mut string = String::new();
        for j in 0..10000 {
            string.push(char::from_u32('A' as u32 + (j % 26) as u32).unwrap());
        }
        strings.push(string);
    }

    c.bench_function("task10", |b| b.iter(|| task10::solve(black_box(&strings))));
}

pub fn criterion_benchmark(c: &mut Criterion) {
    test_task2(c);
    task3_test(c);
    task5_bench(c);
    task7_bench(c);
    task10_bench(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
