extern crate substring;

use substring::{substring_v1, substring_v2, substring_v3, substring_v4, substring_v5};

use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark(c: &mut Criterion) {
    let text = "Hello✨, 🎈 this 🎉 is a text.";
    let (start, end) = (1, 20);
    let expected = "ello✨, 🎈 this 🎉 is ";
    c.bench_function("skip take and collect", |bencher| {
        bencher.iter(|| {
            assert_eq!(expected, substring_v1(text, start, end));
        })
    });
    c.bench_function("manual byte indices", |bencher| {
        bencher.iter(|| {
            assert_eq!(expected, substring_v2(text, start, end));
        })
    });
    c.bench_function("2 char_indices()", |bencher| {
        bencher.iter(|| {
            assert_eq!(expected, substring_v3(text, start, end));
        })
    });
    c.bench_function("1 char_indices()", |bencher| {
        bencher.iter(|| {
            assert_eq!(expected, substring_v4(text, start, end));
        })
    });
    c.bench_function("left-right char skipping", |bencher| {
        bencher.iter(|| {
            assert_eq!(expected, substring_v5(text, start, end));
        })
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
