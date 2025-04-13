use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lab5::{modules::dictionary, tasks::task5};
use rand::Rng;

fn bench_task5(c: &mut Criterion) {
    let mut rng = rand::rng();
    let count = 30000;
    let points: Vec<_> = (0..count)
        .map(|i| task5::Point {
            x: rng.random_range(i32::MIN..i32::MAX) as f64,
            y: rng.random_range(i32::MIN..i32::MAX) as f64,
        })
        .collect();

    c.bench_function("task5_random_points", |b| {
        b.iter(|| task5::solve(black_box(rng.random()), black_box(&points)))
    });
}

fn bench_dictionary(c: &mut Criterion) {
    let mut group = c.benchmark_group("dictionary");
    let mut rng = rand::rng();
    let count = 1000000;
    let max_value = 1000000000;

    group.bench_function("insert_operations", |b| {
        b.iter(|| {
            let mut dict = dictionary::Dict::with_capacity(count);
            for _ in 0..count {
                let key = rng.random_range(0..max_value);
                let value = rng.random_range(0..max_value);
                dict.insert(key, value);
            }
        })
    });
    group.bench_function("random_operations", |b| {
        b.iter(|| {
            let mut dict = dictionary::Dict::with_capacity(count);
            for _ in 0..count {
                let operation = rng.random_range(0..3);
                let key = rng.random_range(0..max_value);
                let value = rng.random_range(0..max_value);

                match operation {
                    0 => dict.insert(key, value),
                    1 => dict.get(key),
                    2 => dict.remove(key),
                    _ => panic!("Invalid operation"),
                };
            }
        })
    });
}

pub fn criterion_benchmark(c: &mut Criterion) {
    bench_task5(c);
    bench_dictionary(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
