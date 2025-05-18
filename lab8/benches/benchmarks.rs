use criterion::{Criterion, black_box, criterion_group, criterion_main};
use lab8::tasks::{task2::can_segment_text, task5};
use rand::distr::{Alphanumeric, SampleString};
use std::collections::{HashSet, hash_set};

fn bench_task2(c: &mut Criterion) {
    let mut group = c.benchmark_group("task2");
    let s = black_box("ab".repeat(50000));
    let words = black_box((1..=50).into_iter().map(|i| "ab".repeat(i)).collect::<HashSet<_>>());

    group.bench_function("abababa", |b| b.iter(|| can_segment_text(&s, &words)));

    let s = black_box("a".repeat(100000));
    let words = black_box((1..=100).into_iter().map(|i| "a".repeat(i)).collect::<HashSet<_>>());
    group.bench_function("aaaaaaa", |b| b.iter(|| can_segment_text(&s, &words)));
}

fn bench_prefixes(c: &mut Criterion) {
    let mut group = c.benchmark_group("prefixes");
    let len_word = 75000;

    let test_func = |word: &str, jack_word: &str| {
        let word_vec = word.chars().collect::<Vec<_>>();
        let jack_word_vec = jack_word.chars().collect::<Vec<_>>();
        let mut prefixes = vec![0usize; len_word];

        for start in 0..len_word {
            let mut prefix_len = 0;

            for i in start..len_word {
                if jack_word_vec[i] == word_vec[i] {
                    prefix_len += 1;
                } else {
                    if prefix_len > 0 {
                        prefixes[start] = prefix_len;
                    }
                    break;
                }
            }
        }
        prefixes
    };

    group.bench_function("abcbe", |b| {
        b.iter(|| {
            test_func(&"abcbe".repeat(len_word / 5), &"abc".repeat(len_word / 3));
        })
    });

    group.bench_function("aaaaabc", |b| {
        b.iter(|| {
            test_func(&"a".repeat(len_word), &"aaabc".repeat(len_word / 5));
        })
    });

    group.bench_function("aaaaaaaa", |b| {
        b.iter(|| {
            test_func(&"a".repeat(len_word), &"a".repeat(len_word));
        })
    });
}

fn bench_hash_set(c: &mut Criterion) {
    let mut group = c.benchmark_group("hash_set");
    let mut hash_set = HashSet::new();
    let len_entertaining = 75000;
    let entertaining_word = "abc".repeat(len_entertaining / 3);

    group.bench_function("add-to-hashset", |b| {
        b.iter(|| {
            for i in 1..=len_entertaining {
                hash_set.insert(&entertaining_word[..i]);
            }
        })
    });
}

fn bench_task5(c: &mut Criterion) {
    let count = 10000;
    let mut group = c.benchmark_group("task5");
    let s = Alphanumeric.sample_string(&mut rand::rng(), count);

    group.bench_function("random_str", |b| b.iter(|| black_box(task5::create_palindrome(&s))));
    group.bench_function("aaaaaaaaaa", |b| {
        b.iter(|| black_box(task5::create_palindrome(&"a".repeat(count))))
    });
    group.bench_function("abcabcabc", |b| {
        b.iter(|| black_box(task5::create_palindrome(&"abc".repeat(count / 3))))
    });
}

pub fn criterion_benchmark(c: &mut Criterion) {
    bench_task2(c);
    bench_hash_set(c);
    bench_prefixes(c);
    bench_task5(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
