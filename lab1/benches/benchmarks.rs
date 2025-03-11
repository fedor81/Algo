use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use lab1::{
    modules::big_int::BigInt,
    tasks::task10::{convert_to_familiar_hashmap, solve},
};

fn task10_bench(c: &mut Criterion) {
    let mut familiar: Vec<(u16, u16)> = Vec::new();

    for i in 1..=23 {
        for j in i + 1..=24 {
            familiar.push((i, j));
        }
    }

    c.bench_with_input(
        BenchmarkId::new("bench task10", "(24, 12, (1..24))"),
        &familiar,
        |b, familiar| {
            b.iter(|| solve(24, 12, convert_to_familiar_hashmap(familiar.clone())));
        },
    );
}

pub fn criterion_benchmark(c: &mut Criterion) {
    big_int_benchmark(c);
    task10_bench(c);
}

fn big_int_benchmark(c: &mut Criterion) {
    c.bench_function("big int(task5)", |b| {
        b.iter(|| {
            let a = black_box(BigInt::from_str(&"1".repeat(1000)));
            let b = black_box(BigInt::from_str(&"1".repeat(1000)));
            let _result = a + b;
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
