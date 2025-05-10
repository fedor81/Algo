use criterion::{Criterion, black_box, criterion_group, criterion_main};
use lab7::tasks::task2::{self, Railway};
use rand::Rng;

fn bench_task2(c: &mut Criterion) {
    let mut rng = rand::rng();
    let mut group = c.benchmark_group("task2");

    let count = 5000;
    let mut railways = vec![vec![Railway::None; count]; count];

    for i in 0..count {
        for j in (i + 1)..count {
            if rng.random_bool(0.5) {
                railways[i][j] = Railway::R;
            } else {
                railways[i][j] = Railway::B;
            }
        }
    }

    group.bench_function("random_test", |b| {
        b.iter(|| {
            task2::solve(black_box(&railways));
        })
    });
}

pub fn criterion_benchmark(c: &mut Criterion) {
    bench_task2(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
